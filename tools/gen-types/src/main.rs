//! Generates `src/generated/schemas.rs` from `openapi.yaml`.
//!
//! Reads the OpenAPI 3.1 spec, extracts `components.schemas`, rewrites refs
//! to typify's `#/definitions/...` convention, runs typify, and writes
//! the rendered Rust file with a `// @generated` header.
//!
//! Run via `task generate-types` (or `cargo run -p gen-types --release`).
//!
//! Two distinct kinds of local patch live in this file; they are not
//! interchangeable:
//!
//! - [`apply_known_patches`] tweaks the in-memory `components/schemas` so the
//!   generated Rust types match wire reality. It never touches the spec file.
//! - [`local_spec_patches`] records divergences the vendored `openapi.yaml`
//!   keeps from upstream `inference-gateway/schemas` at points the generator
//!   ignores (a `paths` response, an `x-provider-configs` vendor extension).
//!   Running with `--patch-spec` (wired in as `task oas-patch`, between
//!   `oas-download` and `generate-types`) reapplies them to the file after the
//!   wholesale upstream overwrite, so a clean `oas-sync` never silently
//!   reverts them.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use schemars::schema::{RootSchema, Schema};
use serde_json::Value;
use typify::{TypeSpace, TypeSpaceSettings};

const HEADER: &str = "\
// @generated - DO NOT EDIT.
// Regenerate with `task generate-types` (or `cargo run -p gen-types --release`).
// Source: openapi.yaml (components.schemas).

#![allow(clippy::all)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

";

fn main() -> Result<()> {
    let workspace_root = workspace_root()?;
    let spec_path = workspace_root.join("openapi.yaml");

    // `--patch-spec` (run by `task oas-patch`, between `oas-download` and
    // `generate-types`) reapplies local divergences to the vendored
    // openapi.yaml after the wholesale upstream overwrite, then exits without
    // generating. See `local_spec_patches`.
    if std::env::args().any(|arg| arg == "--patch-spec") {
        return patch_spec_file(&spec_path);
    }

    let out_path = workspace_root.join("src/generated/schemas.rs");

    let spec_text = std::fs::read_to_string(&spec_path)
        .with_context(|| format!("reading {}", spec_path.display()))?;

    let spec: Value = serde_yaml::from_str(&spec_text)
        .with_context(|| format!("parsing {} as YAML", spec_path.display()))?;

    let schemas_obj = spec
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .cloned()
        .ok_or_else(|| anyhow!("openapi.yaml has no components.schemas"))?;

    let mut schemas_value = Value::Object(schemas_obj);
    rewrite_refs_in_place(&mut schemas_value);
    strip_vendor_extensions(&mut schemas_value);
    normalize_schemas(&mut schemas_value);
    apply_known_patches(&mut schemas_value)?;

    let definitions: BTreeMap<String, Schema> = schemas_value
        .as_object()
        .ok_or_else(|| anyhow!("schemas not an object"))?
        .clone()
        .into_iter()
        .map(|(name, schema)| {
            let parsed: Schema = serde_json::from_value(schema)
                .with_context(|| format!("parsing schema {name} as JSON Schema"))?;
            Ok((name, parsed))
        })
        .collect::<Result<_>>()?;

    let root = RootSchema {
        meta_schema: None,
        schema: Default::default(),
        definitions,
    };

    let mut settings = TypeSpaceSettings::default();
    settings.with_struct_builder(false);
    settings.with_derive("Clone".to_string());

    let mut type_space = TypeSpace::new(&settings);
    type_space
        .add_root_schema(root)
        .map_err(|e| anyhow!("typify add_root_schema: {e}"))?;

    let body = type_space.to_stream();
    let parsed: syn::File = syn::parse2(body).context("parsing typify output as Rust")?;
    let formatted = prettyplease::unparse(&parsed);

    let pre_fmt = format!("{HEADER}{formatted}");

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&out_path, &pre_fmt)
        .with_context(|| format!("writing {}", out_path.display()))?;

    // `cargo fmt` would otherwise reformat the generated file on its next pass,
    // breaking CI drift detection. Format here using the project's settings so
    // the file we commit is identical to what `cargo fmt --check` expects.
    cargo_fmt_file(&workspace_root, &out_path)?;

    println!("Wrote {}", out_path.display());
    Ok(())
}

fn cargo_fmt_file(workspace_root: &std::path::Path, file: &std::path::Path) -> Result<()> {
    use std::process::Command;

    // Run rustfmt repeatedly until the file stops changing. rustfmt is not
    // always idempotent on its first pass for prettyplease-emitted multi-line
    // doc comments; the second pass usually converges. Cap at 5 to avoid an
    // infinite loop if rustfmt itself oscillates on something we missed.
    let mut prev: Option<String> = None;
    for _ in 0..5 {
        let status = Command::new("rustfmt")
            .arg("--edition=2024")
            .arg(file)
            .current_dir(workspace_root)
            .status()
            .context("spawning rustfmt - is the rustfmt component installed?")?;
        if !status.success() {
            return Err(anyhow!("rustfmt exited with {status}"));
        }
        let now = std::fs::read_to_string(file)?;
        if prev.as_deref() == Some(now.as_str()) {
            return Ok(());
        }
        prev = Some(now);
    }
    Err(anyhow!(
        "rustfmt did not converge after 5 passes on {}",
        file.display()
    ))
}

fn workspace_root() -> Result<PathBuf> {
    // CARGO_MANIFEST_DIR for this binary is tools/gen-types/. Go up two levels.
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set; run via cargo")?;
    let root = PathBuf::from(manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow!("could not derive workspace root"))?
        .to_path_buf();
    Ok(root)
}

/// Rewrite `$ref: "#/components/schemas/X"` → `$ref: "#/definitions/X"` in place.
fn rewrite_refs_in_place(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                if k == "$ref" {
                    if let Some(s) = v.as_str() {
                        if let Some(rest) = s.strip_prefix("#/components/schemas/") {
                            *v = Value::String(format!("#/definitions/{rest}"));
                        }
                    }
                } else {
                    rewrite_refs_in_place(v);
                }
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                rewrite_refs_in_place(v);
            }
        }
        _ => {}
    }
}

/// Drop `x-*` vendor extensions and OpenAPI-only fields that schemars rejects
/// or that confuse typify's schema interpretation.
fn strip_vendor_extensions(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|k, _| {
                !k.starts_with("x-")
                    && k != "example"
                    && k != "examples"
                    && k != "discriminator"
                    && k != "externalDocs"
            });
            for (_, v) in map.iter_mut() {
                strip_vendor_extensions(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                strip_vendor_extensions(v);
            }
        }
        _ => {}
    }
}

/// Patch known divergences between the schema and the wire format, **in memory
/// only**, for codegen correctness. This runs on the extracted
/// `components/schemas` and never writes back to `openapi.yaml`, so it can only
/// reach schema definitions - not `paths` or vendor extensions. For divergences
/// the vendored *file* must keep across `oas-sync`, see [`local_spec_patches`].
///
/// Each patch must include a comment explaining *why* - this is the seam where
/// hand-maintenance can creep back in, so keep it auditable. Prefer fixing the
/// spec upstream over adding patches here.
fn apply_known_patches(value: &mut Value) -> Result<()> {
    let schemas = value
        .as_object_mut()
        .ok_or_else(|| anyhow!("schemas not an object"))?;

    if let Some(Value::Object(s)) = schemas.get_mut("ChatCompletionStreamChoice") {
        if let Some(Value::Array(req)) = s.get_mut("required") {
            req.retain(|v| v.as_str() != Some("finish_reason"));
        }
    }

    if let Some(Value::Object(s)) = schemas.get_mut("ChatCompletionStreamResponseDelta") {
        if let Some(Value::Array(req)) = s.get_mut("required") {
            req.retain(|v| {
                let s = v.as_str();
                s != Some("content") && s != Some("role")
            });
        }
    }

    Ok(())
}

/// `--patch-spec` entry point: reapply local spec divergences to the vendored
/// `openapi.yaml` on disk, then exit. Wired in as `task oas-patch`, between
/// `oas-download` (which overwrites the file with upstream) and
/// `generate-types`. Idempotent, so it is safe to run on an already-patched
/// tree - it only rewrites the file when something actually changed.
fn patch_spec_file(spec_path: &Path) -> Result<()> {
    let original = std::fs::read_to_string(spec_path)
        .with_context(|| format!("reading {}", spec_path.display()))?;

    let (patched, actions) = apply_local_spec_patches(&original)?;
    for action in &actions {
        println!("  {action}");
    }

    if patched == original {
        println!("openapi.yaml already matches local intent; nothing to reapply");
        return Ok(());
    }

    std::fs::write(spec_path, &patched)
        .with_context(|| format!("writing {}", spec_path.display()))?;
    println!("Reapplied local spec patches to {}", spec_path.display());
    Ok(())
}

/// A deliberate divergence between the vendored `openapi.yaml` and upstream
/// `inference-gateway/schemas`. Unlike [`apply_known_patches`], these are
/// reapplied to the spec *file* (not the in-memory schema), because they live
/// at points the generator never reads - so they would otherwise be lost the
/// next time `oas-download` overwrites the file.
struct SpecPatch {
    /// Stable identifier, shown in logs and errors.
    name: &'static str,
    /// Why this repo intentionally diverges from upstream.
    reason: &'static str,
    /// Exact upstream text the patch replaces. Must be unique in the file.
    upstream: &'static str,
    /// Local replacement text.
    local: &'static str,
}

/// The full set of local spec divergences. Add an entry here (with a `reason`)
/// for any hand-edit the vendored `openapi.yaml` must keep across `oas-sync`.
/// Prefer reconciling upstream in `inference-gateway/schemas` over adding one.
fn local_spec_patches() -> Vec<SpecPatch> {
    vec![
        // Narrows the streaming response back to a bare `SSEvent` ref. Lives
        // under `paths`, which the generator never reads, so it cannot be an
        // `apply_known_patches` entry.
        SpecPatch {
            name: "sse-stream-response-narrowed",
            reason: "this SDK generates types only from components/schemas and decodes \
                     the stream with the hand-written SSEvents wrapper, so the \
                     text/event-stream response stays a bare SSEvent ref. Upstream's \
                     oneOf only exists so non-Rust generators emit \
                     CreateChatCompletionStreamResponse, which this SDK already generates \
                     from components/schemas",
            upstream: SSE_RESPONSE_UPSTREAM,
            local: SSE_RESPONSE_LOCAL,
        },
        // Flips one provider's vision flag. Lives under the `x-provider-configs`
        // vendor extension, which `strip_vendor_extensions` drops before
        // codegen, so it never reaches `apply_known_patches` either.
        SpecPatch {
            name: "moonshot-supports-vision",
            reason: "vendored spec intentionally pins moonshot.supports_vision = false \
                     (upstream sets true). It lives under the x-provider-configs vendor \
                     extension, stripped before codegen, so generated types are \
                     unaffected. Reconcile upstream in inference-gateway/schemas and drop \
                     this patch if moonshot vision support is confirmed",
            upstream: MOONSHOT_VISION_UPSTREAM,
            local: MOONSHOT_VISION_LOCAL,
        },
    ]
}

/// Reapply every [`local_spec_patches`] entry to `spec_text`, returning the
/// patched text and a human-readable log of what happened.
///
/// Idempotent: a patch already present is left untouched. Loud on drift: if
/// neither the local nor the upstream form is found (or the upstream anchor is
/// not unique), it errors instead of silently doing nothing - so a maintainer
/// notices when upstream reshapes a patched region rather than discovering a
/// silent revert later.
fn apply_local_spec_patches(spec_text: &str) -> Result<(String, Vec<String>)> {
    let mut text = spec_text.to_string();
    let mut actions = Vec::new();

    for patch in local_spec_patches() {
        if text.contains(patch.local) {
            actions.push(format!("{}: already applied", patch.name));
            continue;
        }

        match text.matches(patch.upstream).count() {
            0 => {
                return Err(anyhow!(
                    "local spec patch '{}' did not match: neither the local nor the \
                     upstream form is present in openapi.yaml. Upstream likely reshaped \
                     this region - review and update the patch.\n  reason: {}",
                    patch.name,
                    patch.reason
                ));
            }
            1 => {
                text = text.replacen(patch.upstream, patch.local, 1);
                actions.push(format!("{}: reapplied ({})", patch.name, patch.reason));
            }
            n => {
                return Err(anyhow!(
                    "local spec patch '{}' is ambiguous: its upstream anchor matched {} \
                     times. Tighten the anchor so it is unique.",
                    patch.name,
                    n
                ));
            }
        }
    }

    Ok((text, actions))
}

/// Apply small normalizations so OpenAPI 3.1 schemas parse as JSON Schema draft-07.
///
/// - OpenAPI 3.1 may use `type: ["string", "null"]`. schemars 0.8 expects
///   a single `type` plus `nullable: true` (draft-07-ish).
fn normalize_schemas(value: &mut Value) {
    if let Value::Object(map) = value {
        if let Some(t) = map.get("type").cloned() {
            if let Value::Array(types) = t {
                let non_null: Vec<&Value> = types
                    .iter()
                    .filter(|v| v.as_str() != Some("null"))
                    .collect();
                let has_null = types.iter().any(|v| v.as_str() == Some("null"));
                if non_null.len() == 1 {
                    map.insert("type".into(), non_null[0].clone());
                    if has_null {
                        map.insert("nullable".into(), Value::Bool(true));
                    }
                }
            }
        }
        for (_, v) in map.iter_mut() {
            normalize_schemas(v);
        }
    } else if let Value::Array(arr) = value {
        for v in arr.iter_mut() {
            normalize_schemas(v);
        }
    }
}

// Exact spec text for the `local_spec_patches` entries. Each pair must match the
// vendored file byte-for-byte (indentation included): the `*_UPSTREAM` constant
// is what `oas-download` brings in, the `*_LOCAL` constant is what this repo
// keeps. Keep them in sync with `openapi.yaml` if upstream reshapes a region.

const SSE_RESPONSE_UPSTREAM: &str = r#"            text/event-stream:
              schema:
                description: |
                  Server-Sent Events stream. Each frame is an `SSEvent` whose
                  `data` field contains the JSON-serialized payload for that
                  event. For content/message chunk events the payload is a
                  `CreateChatCompletionStreamResponse`. The `oneOf` here makes
                  the streaming payload schemas reachable from this operation
                  so that code generators emit types for them.
                oneOf:
                  - $ref: '#/components/schemas/SSEvent'
                  - $ref: '#/components/schemas/CreateChatCompletionStreamResponse'"#;

const SSE_RESPONSE_LOCAL: &str = r#"            text/event-stream:
              schema:
                $ref: '#/components/schemas/SSEvent'"#;

const MOONSHOT_VISION_UPSTREAM: &str = r#"          url: 'https://api.moonshot.ai/v1'
          auth_type: 'bearer'
          supports_vision: true"#;

const MOONSHOT_VISION_LOCAL: &str = r#"          url: 'https://api.moonshot.ai/v1'
          auth_type: 'bearer'
          supports_vision: false"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh `oas-download` brings in the upstream forms; the patch step must
    /// turn them back into the local forms.
    #[test]
    fn reapplies_divergences_from_upstream() {
        let upstream = format!(
            "paths:\n{SSE_RESPONSE_UPSTREAM}\n        '400': {{}}\n      \
             x-provider-configs:\n{MOONSHOT_VISION_UPSTREAM}\n          endpoints: {{}}\n"
        );

        let (patched, actions) = apply_local_spec_patches(&upstream).unwrap();

        assert!(patched.contains(SSE_RESPONSE_LOCAL));
        assert!(patched.contains(MOONSHOT_VISION_LOCAL));
        // The upstream-only bits are gone.
        assert!(!patched.contains("CreateChatCompletionStreamResponse"));
        assert!(!patched.contains("supports_vision: true"));
        assert!(actions.iter().all(|a| a.contains("reapplied")));
    }

    /// Running on an already-patched tree must be a no-op (so `generate-types`
    /// and CI never see a spurious spec diff).
    #[test]
    fn idempotent_when_already_local() {
        let local = format!("{SSE_RESPONSE_LOCAL}\n{MOONSHOT_VISION_LOCAL}\n");

        let (patched, actions) = apply_local_spec_patches(&local).unwrap();

        assert_eq!(patched, local);
        assert!(actions.iter().all(|a| a.contains("already applied")));
    }

    /// If upstream reshapes a patched region so neither form is found, the patch
    /// must fail loudly rather than silently leave the divergence reverted.
    #[test]
    fn errors_when_region_missing() {
        let err = apply_local_spec_patches("openapi: 3.1.0\n").unwrap_err();
        assert!(err.to_string().contains("did not match"));
    }
}
