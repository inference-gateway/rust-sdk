# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust SDK (`inference-gateway-sdk`) for the [Inference Gateway][ig] — a unified
HTTP interface for interacting with various LLM providers (Ollama, OllamaCloud,
Groq, OpenAI, Cloudflare, Cohere, Anthropic, DeepSeek, Google, Mistral, Moonshot).
Edition 2024.

[ig]: https://github.com/inference-gateway/inference-gateway

## Build & Development Commands

Prefer `task` (Taskfile.yml) — the underlying `cargo` invocations are equivalent.

```bash
task test         # cargo test --all-targets --all-features
task lint         # cargo fmt --all -- --check  AND  markdownlint
task analyse      # cargo clippy --all-targets --all-features -- -D warnings
cargo fmt         # apply formatting
cargo test <name> # run a single test by name
```

### Regenerating types from OpenAPI

`src/generated/schemas.rs` is produced by `tools/gen-types` (typify) from
`openapi.yaml`. **Do not hand-edit it** — the `@generated` header is enforced
and CI checks for drift.

```bash
task oas-download    # refresh openapi.yaml from upstream inference-gateway/schemas
task generate-types  # regenerate src/generated/schemas.rs
task oas-sync        # both of the above
```

## Architecture

Cargo workspace. Members: `.` (the SDK), `tools/gen-types` (codegen binary),
`examples/list`, `examples/chat`.

### Source layout

- `src/lib.rs` — `InferenceGatewayClient`, the `InferenceGatewayAPI` trait,
  `GatewayError`, the `SSEvents` SSE wrapper, and HTTP plumbing.
- `src/tests.rs` — all unit/integration tests (mockito-based). Tests live in
  this separate file, **not** inline at the bottom of `lib.rs`.
- `src/generated/schemas.rs` — generated request/response/enum types
  (`Provider`, `Message`, `MessageRole`, `MessageContent`, `ChatCompletionTool`,
  `ChatCompletionToolType`, `FunctionObject`, `FunctionParameters`,
  `CreateChatCompletionRequest`/`Response`, `CreateChatCompletionStreamResponse`,
  `ListModelsResponse`, `ListToolsResponse`, `FinishReason`, …). Re-exported
  from the crate root via `pub use generated::schemas::*;`.
- `src/ext/` — hand-written impls layered on top of generated types
  (currently just `tool_call.rs::parse_arguments()`). Add new impls here when
  the schema cannot describe the behavior; do not patch `schemas.rs`.

### Key invariants worth knowing

- **`health_check` URL quirk**: `/health` is served from the server root, not
  under the versioned API prefix. `health_url()` strips a trailing
  `/v<digits>` segment from the configured `base_url`. Don't "simplify" this.
- **Streaming requests omit tools and max_tokens**: `build_chat_request` deliberately
  drops `tools` and `max_tokens` when `stream == true`. Preserve this when
  touching the request builder.
- **Builder pattern is consuming**: `with_token`, `with_tools`, `with_max_tokens`
  take `mut self` and return `Self`. Tests and examples chain them.
- **Streaming uses `async_stream::try_stream!`** producing
  `impl Stream<Item = Result<SSEvents, GatewayError>>`. Consumers `pin_mut!`
  and use `futures_util::StreamExt`. SSE parsing is line-based on
  `event:` / `data:` prefixes, with `[DONE]` as the sentinel.
- **`SSEvents` (SDK type) ≠ `SsEvent` (generated)** — the generated type
  constrains `event` to a fixed enum; the SDK wrapper accepts arbitrary
  upstream event names.
- **Error mapping**: `map_error_status` translates HTTP status codes to
  specific `GatewayError` variants and tries to parse `{ "error": "..." }`
  from the body before falling back to the canonical reason.

## Testing

Tests use `mockito::Server` to spin up an HTTP mock and point the client at
`server.url()`. When adding tests, follow the helper-function pattern in
`src/tests.rs` (e.g. `user_message`, `system_message`, `function_params`).

## Examples

`examples/list` and `examples/chat` are workspace member crates. Run from
the repo root (the gateway must be reachable, default `http://localhost:8080/v1`):

```bash
cargo run -p list-example
PROVIDER=deepseek LLM=deepseek-v4-flash cargo run -p chat-example
```

Env vars: `PROVIDER`, `LLM`, `INFERENCE_GATEWAY_URL`. See `examples/README.md`.

## Commit Message Convention

Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `style:`, `test:`,
`chore:`, `ci:`, `perf:`. Releases are driven by semantic-release based on
commit history.
