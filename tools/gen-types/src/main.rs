//! Generates `src/generated/schemas.rs` from `openapi.yaml`.
//!
//! Reads the OpenAPI 3.1 spec, extracts `components.schemas`, rewrites refs
//! to typify's `#/definitions/...` convention, runs typify, and writes
//! the rendered Rust file with a `// @generated` header.
//!
//! Run via `task generate-types` (or `cargo run -p gen-types --release`).

use std::collections::BTreeMap;
use std::path::PathBuf;

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

/// Patch known divergences between the spec and the wire format.
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
