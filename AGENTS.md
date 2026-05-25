# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust 2024 Cargo workspace for the Inference Gateway SDK. Core library code
lives in `src/lib.rs`, with SDK extensions in `src/ext/` and generated OpenAPI schema types in
`src/generated/`. Unit and integration-style tests are currently collected in `src/tests.rs` and use
`mockito` for HTTP behavior. Example clients live under `examples/chat/` and `examples/list/`. The
OpenAPI source is `openapi.yaml`, and the generator implementation is in `tools/gen-types/`.

## Build, Test, and Development Commands

- `cargo build --all-targets --all-features`: build the SDK, examples, and tools.
- `task test`: run `cargo test --all-targets --all-features`.
- `task lint`: check Rust formatting and Markdown lint rules.
- `task analyse`: run `cargo clippy --all-targets --all-features -- -D warnings`.
- `task generate-types`: regenerate `src/generated/schemas.rs` from `openapi.yaml`.
- `task oas-sync`: download the latest upstream OpenAPI spec and regenerate types.

Use `flox activate` if you want the same development environment described in `CONTRIBUTING.md`.

## Coding Style & Naming Conventions

Use Rust 1.95.0 from `rust-toolchain.toml`. Format Rust with `rustfmt`; `.editorconfig` specifies LF
endings, final newlines, and 4-space indentation for `*.rs`. Keep public API names idiomatic Rust:
`PascalCase` for types and traits, `snake_case` for functions, modules, and tests. Do not manually
edit generated schema code unless the generator cannot express the required fix; prefer updating
`openapi.yaml` or `tools/gen-types/`.

## Testing Guidelines

Add tests for new SDK behavior, bug fixes, and serialization changes. Follow the existing `test_*`
naming style in `src/tests.rs`. Use `#[tokio::test]` for async client behavior and
`mockito::Server` for HTTP expectations. Run `task test` before opening a pull request, and run
`task analyse` when touching shared client logic or generated type behavior.

## Commit & Pull Request Guidelines

Commit history follows Conventional Commits, for example `feat: Add my feature`,
`fix: Handle stream decoding`, `chore(deps): Bump dev dependencies`, and
`ci(deps): Update workflow action`. Supported types include `feat`, `fix`, `refactor`, `docs`,
`style`, `test`, `chore`, `ci`, and `perf`.

Pull requests should include a clear description, linked issues when applicable, tests for behavior
changes, and documentation updates for user-facing API changes. Before requesting review, ensure
`task test`, `task lint`, and `task analyse` pass.

## Security & Configuration Tips

Do not commit real credentials. Use `.env.example` as the template for local configuration.
`InferenceGatewayClient::new_default()` reads `INFERENCE_GATEWAY_URL` and falls back to
`http://localhost:8080/v1`.
