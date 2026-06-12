# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common commands

The repo uses [Taskfile](https://taskfile.dev) wrappers around `cargo`. The Task commands are what CI runs; use them locally to match.

- `task test` — `cargo test --all-targets --all-features`
- `task lint` — `cargo fmt --all -- --check` + `markdownlint`
- `task analyse` — `cargo clippy --all-targets --all-features -- -D warnings` (note: CONTRIBUTING.md says `task analyze` but the actual task is `analyse`)
- `task generate-types` — regenerate `src/generated/schemas.rs` from `openapi.yaml`
- `task oas-patch` — reapply this repo's local divergences to `openapi.yaml` (see `local_spec_patches`)
- `task oas-sync` — pull the latest `openapi.yaml` from `inference-gateway/schemas` on `main`, reapply local patches (`oas-patch`), then regenerate types

Running a single test: `cargo test --all-targets --all-features <test_name_substring>`.

Examples are workspace members and run via `-p`:

```bash
cargo run -p list-example
PROVIDER=deepseek LLM=deepseek-v4-flash cargo run -p chat-example
```

Toolchain is pinned in `rust-toolchain.toml` to Rust 1.95.0 (edition 2024) with `rustfmt` + `clippy`. Dev environment is managed via [flox](https://flox.dev/) — `flox activate` from the repo root.

## Architecture

This is a thin Rust client for the Inference Gateway HTTP API. Two boundaries matter:

### Generated vs. hand-written code

- `src/generated/schemas.rs` is **generated** from `openapi.yaml` by the `gen-types` workspace member (`tools/gen-types`). Header banner `// @generated - DO NOT EDIT.` — never edit this file by hand. Re-run `task generate-types` after any spec change.
- `openapi.yaml` is a copy of the upstream spec at `inference-gateway/schemas`. Treat it as read-only in this repo; fix divergences upstream and pull them in with `task oas-sync`. If a hand-patch in this repo is truly unavoidable, record it in one of **two** places in `tools/gen-types/src/main.rs`, each with a comment explaining why — and prefer reconciling upstream over either:
  - **Codegen-only schema tweaks** → `apply_known_patches`. Applied to the in-memory `components/schemas` so the generated Rust types match wire reality (e.g. relaxing `required`). Never written back to `openapi.yaml`, and structurally limited to schema definitions.
  - **Vendored-spec divergences** (anywhere in the document, including `paths` and the `x-provider-configs` vendor extension) → `local_spec_patches`. `task oas-patch` reapplies these to the `openapi.yaml` file after `oas-download` overwrites it, so a clean `oas-sync` brings in only genuine upstream changes and never silently reverts a local edit. Patches are idempotent and fail loudly if upstream reshapes a patched region. Current entries: the SSE `text/event-stream` response stays a bare `SSEvent` ref, and `moonshot.supports_vision` stays `false`.
- Hand-written code lives only in `src/lib.rs` (client + `InferenceGatewayAPI` trait + error type) and `src/ext/` (small impls layered on generated types, like `parse_arguments` on tool-call functions). typify already emits `Display`, `FromStr`, and `TryFrom<&str>` for enums — don't reimplement those.
- CI fails if `task generate-types` produces a diff against committed `src/generated/`. The generator runs `rustfmt` repeatedly until idempotent specifically so `cargo fmt --check` and the codegen output stay aligned — don't disable that loop.

### Client / trait surface

- `InferenceGatewayAPI` is the public API trait; `InferenceGatewayClient` is the only implementation. Adding a new endpoint means adding it to both.
- Builder-style configuration: `with_tools`, `with_token`, `with_max_tokens` consume and return `self`.
- `build_chat_request` deliberately omits `tools` and `max_tokens` when `stream = true` — keep that asymmetry if you touch it; streaming requests must not carry those fields.
- `health_url()` strips a trailing `/v<digits>` segment from `base_url` because `/health` is served from the root, not under the versioned API prefix. Don't "simplify" by appending `/health` to `base_url` directly.
- `SSEvents` (the SDK's SSE wrapper used by `generate_content_stream`) is intentionally distinct from the generated `SsEvent` type: the spec's `SsEvent` constrains `event` to a fixed enum, but upstream providers emit arbitrary event names that must round-trip.

### Workspace layout

Cargo workspace with members declared in the root `Cargo.toml`:

- `.` — the published `inference-gateway-sdk` crate
- `tools/gen-types` — internal codegen binary (`publish = false`)
- `examples/list`, `examples/chat` — runnable examples (`publish = false`)

## Commits and releases

Conventional Commits drive semantic-release (see `.releaserc.yaml`). Allowed types: `feat`, `fix`, `refactor`, `perf`, `impr`, `docs`, `style`, `test`, `chore`, `ci`, `build`. `feat` bumps minor; most others bump patch. Don't hand-edit `CHANGELOG.md` or the `version` field in `Cargo.toml` — semantic-release rewrites both on merge to `main`.
