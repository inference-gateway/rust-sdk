# AGENTS.md — Inference Gateway Rust SDK

This document provides comprehensive guidance for AI agents working with the
Inference Gateway Rust SDK project. It covers the project structure, architecture,
development workflow, conventions, and actionable instructions.

---

## Project Overview

**Inference Gateway Rust SDK** is a Rust client library (`inference-gateway-sdk`)
for the [Inference Gateway](https://github.com/inference-gateway/inference-gateway)
— a unified API gateway for interacting with various LLM providers through a
single, OpenAI-compatible interface.

### Key Technologies

| Technology | Purpose |
| --- | --- |
| **Rust 2024 edition** | Language and edition |
| **reqwest 0.13** | HTTP client (JSON + stream features) |
| **serde / serde_json** | Serialization/deserialization |
| **tokio 1.52** | Async runtime (multi-thread) |
| **async-stream 0.3** | SSE streaming generation |
| **thiserror 2.0** | Error type derivation |
| **mockito 1.7** | HTTP mocking in tests |
| **typify 0.6** | OpenAPI → Rust code generation |
| **Taskfile** | Task runner (`task` command) |
| **flox** | Development environment manager |
| **semantic-release** | Automated release process |

---

## Architecture and Structure

### Directory Layout

```text
.
├── src/
│   ├── lib.rs                  # Main library: client, API trait, error types, SSE
│   ├── tests.rs                # All integration tests (mockito-based)
│   ├── ext/
│   │   ├── mod.rs              # Hand-written extensions on generated types
│   │   └── tool_call.rs        # ChatCompletionMessageToolCallFunction::parse_arguments()
│   └── generated/
│       ├── mod.rs              # Re-exports schemas module
│       └── schemas.rs          # @generated — DO NOT EDIT. From openapi.yaml via typify
├── tools/
│   └── gen-types/
│       ├── Cargo.toml          # Internal codegen binary deps (typify, syn, schemars)
│       └── src/main.rs         # Reads openapi.yaml, runs typify, writes schemas.rs
├── .github/
│   ├── workflows/
│   │   ├── ci.yml              # CI pipeline (lint, clippy, build, test, drift check)
│   │   ├── release.yml         # Manual release workflow (semantic-release + cargo publish)
│   │   └── claude.yml          # Claude Code PR assistant (triggered by @claude mentions)
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       ├── feature_request.md
│       └── refactor_request.md
├── .flox/                      # flox environment definition (Rust toolchain)
├── Cargo.toml                  # Package config + workspace root
├── Taskfile.yml                # Task definitions (oas-sync, lint, analyse, test)
├── openapi.yaml                # OpenAPI 3.1 spec from upstream inference-gateway/schemas
├── .releaserc.yaml             # semantic-release configuration
├── .editorconfig               # Editor settings (rust: 4 spaces, yaml/toml/json: 2 spaces)
├── .env.example                # Provider API key placeholders
├── .markdownlintignore         # Markdown linting exclusions
├── .gitattributes
├── .gitignore
├── CLAUDE.md                   # Claude-specific guidance (legacy reference)
├── CONTRIBUTING.md             # Contribution guidelines
├── CHANGELOG.md                # Auto-generated release changelog
├── README.md                   # User documentation with examples
└── LICENSE                     # Apache-2.0
```

### Core Components

#### `InferenceGatewayClient` (src/lib.rs)

The main HTTP client struct with builder-pattern configuration:

```rust
pub struct InferenceGatewayClient {
    base_url: String,           // e.g. "http://localhost:8080/v1"
    client: Client,             // reqwest::Client
    token: Option<String>,      // Bearer auth token
    tools: Option<Vec<ChatCompletionTool>>,  // Tools for function calling
    max_tokens: Option<i64>,    // Max tokens per request
}
```

**Builder methods:**

- `new(base_url)` — create client targeting a specific URL
- `new_default()` — uses `INFERENCE_GATEWAY_URL` env var or `http://localhost:8080/v1`
- `.with_token(token)` — set bearer auth
- `.with_tools(tools)` — set tools for function calling
- `.with_max_tokens(max_tokens)` — set token limit

#### `InferenceGatewayAPI` Trait (src/lib.rs)

Defines the API surface — all methods are async:

| Method | HTTP | URL Pattern | Description |
| --- | --- | --- | --- |
| `list_models()` | GET | `{base}/models` | List all models |
| `list_models_by_provider(Provider)` | GET | `{base}/models?provider=` | Filter by provider |
| `generate_content(Provider, model, messages)` | POST | `{base}/chat/completions?provider=` | Chat completion |
| `generate_content_stream(Provider, model, messages)` | POST (streaming) | `{base}/chat/completions?provider=` | SSE streaming |
| `list_tools()` | GET | `{base}/mcp/tools` | List MCP tools |
| `health_check()` | GET | `{base}/health` | Health probe |

#### Provider Enum

11 supported providers (serialized as lowercase with underscore):

```rust
Provider::Ollama       // "ollama"
Provider::OllamaCloud  // "ollama_cloud"
Provider::Groq         // "groq"
Provider::Openai       // "openai"
Provider::Cloudflare   // "cloudflare"
Provider::Cohere       // "cohere"
Provider::Anthropic    // "anthropic"
Provider::Deepseek     // "deepseek"
Provider::Google       // "google"
Provider::Mistral      // "mistral"
Provider::Moonshot     // "moonshot"
```

The `Provider` enum supports `Display`, `FromStr`, `TryFrom<&str>`, and
`serde` serialization/deserialization (all generated by typify).

#### GatewayError Enum (src/lib.rs)

Comprehensive error handling with `thiserror`:

```rust
GatewayError::Unauthorized(String)       // 401
GatewayError::Forbidden(String)          // 403
GatewayError::NotFound(String)           // 404
GatewayError::BadRequest(String)         // 400
GatewayError::InternalError(String)      // 500
GatewayError::StreamError(reqwest::Error)
GatewayError::DecodingError(FromUtf8Error)
GatewayError::RequestError(reqwest::Error)     // auto-from for reqwest errors
GatewayError::DeserializationError(serde_json::Error)
GatewayError::SerializationError(serde_json::Error)  // auto-from for serde errors
GatewayError::Other(Box<dyn Error + Send + Sync>)
```

#### Module Organization

- **`crate::generated::schemas`** — Auto-generated types from `openapi.yaml`.
  Contains all request/response structs, enums, and nested types. DO NOT modify
  directly — regenerate via `task generate-types`.
- **`crate::ext::tool_call`** — Hand-written extension methods on generated
  types (e.g., `parse_arguments()` on tool-call function objects).
- **`crate::lib`** — Client implementation, API trait, SSE parser, error types.
  Re-exports everything from `generated::schemas` at crate root.

### Key Generated Types (src/generated/schemas.rs)

| Type | Description |
| --- | --- |
| `Message` | Chat message with role, content, tool_calls, tool_call_id |
| `MessageRole` | Enum: User, Assistant, System, Tool |
| `MessageContent` | Enum: String or Array of content parts |
| `CreateChatCompletionRequest` | Request body for chat completions |
| `CreateChatCompletionResponse` | Non-streaming response |
| `CreateChatCompletionStreamResponse` | Streaming chunk response |
| `ChatCompletionTool` | Tool definition for function calling |
| `ChatCompletionToolType` | Enum: Function |
| `FunctionObject` | Function definition with name, description, parameters |
| `FunctionParameters` | Newtype wrapper around serde_json::Map |
| `ChatCompletionMessageToolCall` | Tool call in a response message |
| `ChatCompletionMessageToolCallFunction` | Tool call function with name + arguments |
| `ListModelsResponse` | Response from list models endpoint |
| `ListToolsResponse` | Response from list MCP tools endpoint |
| `FinishReason` | Enum: Stop, Length, ToolCalls, etc. |
| `SsEvent` | Generated SSE event type |
| `ToolCallExtraContent` | Extra content for tool calls |

### Code Generation Pipeline

The file `src/generated/schemas.rs` is **auto-generated** and must never be
edited by hand. The pipeline is:

1. **`task oas-download`** — Fetches `openapi.yaml` from upstream
   `inference-gateway/schemas` repository.
2. **`task generate-types`** — Runs `cargo run -p gen-types --release` which:
   - Parses `openapi.yaml`
   - Extracts `components.schemas`
   - Rewrites `$ref` paths from OpenAPI convention to typify's `#/definitions/...`
   - Strips vendor extensions (`x-*`) and OpenAPI-only fields
   - Normalizes type arrays to draft-07 compatible schemas
   - Applies known patches (e.g., making streaming fields optional)
   - Runs typify to generate Rust types
   - Formats with prettyplease and rustfmt
   - Writes to `src/generated/schemas.rs`
3. **`task oas-sync`** — Runs both steps together.

The CI pipeline (`ci.yml`) verifies that generated types are in sync with the
spec by running the generator and checking for a clean `git diff`.

---

## Development Environment Setup

### Prerequisites

- [flox](https://flox.dev/) — manages Rust toolchain and system dependencies
- `task` command — [Task runner](https://taskfile.dev/) v3.48+
- Rust toolchain (installed via flox or [rustup](https://rustup.rs/))

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/inference-gateway/rust-sdk
cd rust-sdk

# Activate the flox development environment
# This installs Rust, cargo, clippy, rustfmt and other tools
flox activate

# Build the library
cargo build

# Run all tests
cargo test --all-targets --all-features
```

### Alternative Setup (without flox)

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Task runner
brew install go-task   # macOS
# or: sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d

# Build and test
cargo build
cargo test --all-targets --all-features
```

---

## Key Commands

### Using Cargo

| Command | Description |
| --- | --- |
| `cargo build` | Build the library |
| `cargo build --release` | Build with optimizations |
| `cargo test --all-targets --all-features` | Run all tests |
| `cargo test test_name` | Run a specific test by name |
| `cargo test --verbose` | Run tests with verbose output |
| `cargo fmt --all -- --check` | Check formatting (CI gate) |
| `cargo fmt` | Auto-format all code |
| `cargo clippy --all-targets --all-features -- -D warnings` | Static analysis (CI gate) |
| `cargo doc --open` | Build and open docs |
| `cargo publish` | Publish to crates.io |
| `cargo check` | Type-check without building |

### Using Task (Taskfile.yml)

| Command | Description |
| --- | --- |
| `task test` | Run `cargo test --all-targets --all-features` |
| `task lint` | Run `cargo fmt --check` + markdownlint |
| `task analyse` | Run `cargo clippy ... -D warnings` |
| `task oas-download` | Download latest openapi.yaml from upstream |
| `task generate-types` | Regenerate `src/generated/schemas.rs` |
| `task oas-sync` | Download spec + regenerate types |

---

## Testing

### Test Structure

- All tests live in `src/tests.rs` (imported via `#[cfg(test)] mod tests;` in `lib.rs`)
- Uses **mockito** for HTTP mocking (no real API calls)
- Async tests use `#[tokio::test]`
- Synchronous tests (serialization) use plain `#[test]`

### Test Patterns

**Mockito Setup** — Every async test creates a mock server:

```rust
#[tokio::test]
async fn test_list_models() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/models")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_response_json)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let response = client.list_models().await?;

    // Assertions...
    mock.assert();
    Ok(())
}
```

**Helper Functions** — Test helpers are defined at the top of `tests.rs`:

```rust
fn user_message(text: &str) -> Message { /* ... */ }
fn system_message(text: &str) -> Message { /* ... */ }
fn function_params(value: serde_json::Value) -> FunctionParameters { /* ... */ }
```

**Streaming Tests** — Use `with_chunked_body()` on mockito mocks:

```rust
let mock = server
    .mock("POST", "/v1/chat/completions?provider=deepseek")
    .with_status(200)
    .with_header("content-type", "text/event-stream")
    .with_chunked_body(move |writer| -> std::io::Result<()> {
        writer.write_all(b"data: ...\n\n")?;
        Ok(())
    })
    .create();
```

### Running Specific Tests

```bash
# Run a single test by name
cargo test test_list_models

# Run all provider related tests
cargo test provider

# Run only async tests
cargo test -- --include-ignored  # if tests are marked #[ignore]

# Run with full output
cargo test -- --nocapture
```

### Test Coverage Areas

The test suite comprehensively covers:

- **Provider enum** — Serialization, deserialization, Display, TryFrom
- **Message serialization** — With/without tool_call_id, role variants
- **Client configuration** — base_url, default URL, auth token
- **Error handling** — 401, 400, 403, 404, 500 status codes with error bodies
- **Model listing** — All models, by provider
- **Content generation** — Basic, with tools, without tools, serialization
- **Streaming** — SSE event parsing, [DONE] termination, error events
- **Tool use** — Tool call response parsing, argument parsing
- **Health check** — Healthy/unhealthy responses
- **MCP tools** — List tools, auth headers, 403 when MCP not exposed

---

## CI/CD Pipeline

### CI (`ci.yml`) — Runs on push/PR to `main`

Steps in order:

1. **Lint** — `cargo fmt --all -- --check`
2. **Static Analysis** — `cargo clippy --all-targets --all-features -- -D warnings`
3. **Build** — `cargo build --verbose`
4. **Test** — `cargo test --all-targets --all-features --verbose`
5. **Drift Check** — Regenerates types, fails if `src/generated/` differs from committed version

### Release (`release.yml`) — Manual trigger

1. **GitHub Release job:**
   - Configures GPG signing
   - Installs semantic-release v25 + conventional-changelog plugins
   - Creates initial `0.1.0` release if none exists (workaround for semver < 1.0)
   - Runs semantic-release dry-run to detect next version
   - Executes semantic-release to bump version, update CHANGELOG.md, create GitHub release
2. **Publish job** (depends on release):
   - Updates version in Cargo.toml
   - Builds release artifact
   - Publishes to crates.io

### Claude Code PR Assistant (`claude.yml`)

Triggered by `@claude` mentions on issues, PRs, and comments. Uses the
Anthropic Claude Code action to respond. Has access to `task:*`, `go:*`,
`gh:*`, and `git:*` Bash commands.

### Branch Strategy

- **`main`** — Production branch, protected
- **`rc/*`** — Release candidate branches (pre-release via semantic-release)
- **`feature/*`** — Feature branches for development
- **`claude/*`** — Auto-created branches by Claude Code action

---

## Project Conventions and Coding Standards

### Rust Style

- **Edition**: 2024
- **Indentation**: 4 spaces for Rust, 2 spaces for YAML/TOML/JSON
- **Line endings**: LF, trailing newline required
- **Naming**: Standard Rust conventions (snake_case for functions/variables,
  CamelCase for types, SCREAMING_SNAKE_CASE for constants)
- **Serde**: Use `#[serde(rename_all = "lowercase")]` for enums, follow
  OpenAI-compatible JSON field names

### Code Organization

- Generated code lives in `src/generated/` — never edit manually
- Hand-written extensions on generated types go in `src/ext/`
- The main library (`lib.rs`) should remain focused on client implementation
  — if it grows significantly, split into submodules
- Re-export generated types at crate root so consumers use
  `inference_gateway_sdk::Message` not `inference_gateway_sdk::generated::schemas::Message`

### Commit Message Convention

Use **conventional commits** with capital letters:

```text
type(scope): Description starting with capital letter
```

| Type | Release | Description |
| --- | --- | --- |
| `feat` | minor | New feature |
| `fix` | patch | Bug fix |
| `refactor` | patch | Code change with no feature/fix |
| `perf` | patch | Performance improvement |
| `docs` | patch | Documentation |
| `style` | patch | Formatting, no code change |
| `test` | patch | Adding/modifying tests |
| `chore` | patch | Build config, maintenance |
| `ci` | patch | CI configuration |
| `build` | patch | Build system |
| `impr` | patch | Improvement (non-feature) |

Examples:

```text
feat(provider): Add Mistral AI provider support
fix(streaming): Handle empty delta chunks correctly
refactor(client): Extract common request logic
docs(readme): Update API examples
```

### Error Handling

- Use `GatewayError` consistently for all fallible operations
- Map HTTP status codes to typed error variants via `map_error_status()`
- Use `thiserror` for error derivation
- Return `Result<T, GatewayError>` from all API methods

### Builder Pattern

Client configuration uses a consuming builder pattern:

```rust
let client = InferenceGatewayClient::new("http://localhost:8080/v1")
    .with_token("my-token")
    .with_tools(Some(my_tools))
    .with_max_tokens(Some(4096));
```

### Streaming

- Uses `async_stream::try_stream!` macro
- Returns `impl Stream<Item = Result<SSEvents, GatewayError>>`
- Parses raw SSE format (`event:`, `data:`, blank line delimiter)
- Consumers use `futures_util::pin_mut!` and `StreamExt::next()`

---

## Important Files and Configurations

### Configuration Files

| File | Purpose |
| --- | --- |
| `Cargo.toml` | Package definition, dependencies, workspace members |
| `Taskfile.yml` | Task automation (test, lint, analyse, oas-sync) |
| `.releaserc.yaml` | semantic-release configuration with release rules |
| `.editorconfig` | Editor consistency settings |
| `.env.example` | Template for provider API keys |

### CI/CD Files

| File | Purpose |
| --- | --- |
| `.github/workflows/ci.yml` | Lint, clippy, build, test, drift check |
| `.github/workflows/release.yml` | Semantic release + cargo publish |
| `.github/workflows/claude.yml` | Claude Code PR assistant |

### Source Files

| File | Lines | Purpose |
| --- | --- | --- |
| `src/lib.rs` | ~340 | Client, API trait, error types, SSE |
| `src/tests.rs` | ~879 | All integration tests |
| `src/generated/schemas.rs` | ~1000+ | Auto-generated types |
| `src/ext/tool_call.rs` | ~11 | Hand-written extensions |
| `tools/gen-types/src/main.rs` | ~264 | Code generation binary |

### Environment Variables

| Variable | Default | Purpose |
| --- | --- | --- |
| `INFERENCE_GATEWAY_URL` | `http://localhost:8080/v1` | Client base URL (used by `new_default()`) |
| `RUST_LOG` | — | Log level for env_logger |

### Dependencies (Cargo.toml)

**Runtime:**

- `reqwest 0.13` (features: json, stream) — HTTP client
- `serde 1.0` (features: derive) — Serialization
- `serde_json 1.0` — JSON handling
- `tokio 1.52` (features: macros, rt-multi-thread) — Async runtime
- `async-stream 0.3` — Streaming support
- `futures-util 0.3` — Stream utilities
- `thiserror 2.0` — Error type derivation

**Dev:**

- `mockito 1.7` — HTTP mocking for tests
- `tokio 1.52` (features: macros, rt) — Async test runtime

**Internal tool (`tools/gen-types/`):**

- `typify 0.6` — OpenAPI to Rust type generation
- `schemars 0.8` — JSON Schema handling
- `syn 2` — Rust syntax parsing
- `prettyplease 0.2` — Rust code formatting
- `serde_yaml 0.9` — YAML parsing for openapi.yaml
- `anyhow 1` — Error handling in codegen

---

## Common Agent Workflows

### Adding a New Provider

1. Update `openapi.yaml` with the new provider in the `Provider` enum schema
2. Run `task generate-types` to regenerate `src/generated/schemas.rs`
3. Add the provider to tests in `src/tests.rs` (serialization, deserialization, Display)
4. Update the list in `README.md` supported providers section
5. Update `.env.example` if the provider requires an API key
6. Run `task test && task lint && task analyse` to verify

### Modifying Generated Types

1. Edit `openapi.yaml` (the source of truth)
2. Run `task generate-types` to regenerate
3. If the spec change breaks compilation, update `tools/gen-types/src/main.rs`
   patches in `apply_known_patches()` or `normalize_schemas()`
4. If hand-written extensions need updating, modify `src/ext/`
5. Update tests to match new types
6. Verify with `task test && task analyse`

### Fixing a Bug or Adding a Feature

1. Create a feature branch: `git checkout -b feature/my-change`
2. Make changes in `src/lib.rs` or `src/ext/`
3. Add/modify tests in `src/tests.rs`
4. Run `task test && task lint && task analyse`
5. Commit with conventional commit message
6. Push and open a PR to `main`

### Syncing with Upstream Spec

```bash
# Full sync
task oas-sync

# Or step by step
task oas-download    # Download latest openapi.yaml
task generate-types  # Regenerate types
```

### Releasing a New Version

Releases are handled via the GitHub Actions workflow — **do not manually bump
versions**:

1. Ensure `main` has the desired commits with conventional commit messages
2. Go to GitHub Actions → Release workflow → "Run workflow"
3. semantic-release determines the next version from commit history
4. Automatically: updates CHANGELOG.md, Cargo.toml, creates GitHub release,
   publishes to crates.io

---

## LLM-specific Guidance

When working with this codebase as an AI agent:

1. **Never edit `src/generated/schemas.rs` directly** — it is auto-generated.
   Edit `openapi.yaml` and regenerate.
2. **Hand-written code that depends on generated types** goes in `src/ext/`.
3. **Tests live in `src/tests.rs`** — add tests there for new functionality.
4. **Follow the builder pattern** for any new client configuration options.
5. **Use `GatewayError`** for all error types — do not introduce new error enums.
6. **Serde derives** (`Serialize`/`Deserialize`) and field attributes must match
   the OpenAI-compatible JSON wire format.
7. **Keep the API OpenAI-compatible** — new features should align with OpenAI's
   chat completion API shape where possible.
8. **Run `task oas-sync`** after any upstream spec changes to keep types in sync.
9. **CI is strict** — `cargo fmt --check` and `cargo clippy -D warnings` must
   pass, and generated types must not drift from the spec.
10. **Markdown files** must pass markdownlint — check with `task lint`.
