# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

Rust SDK for the [Inference Gateway][ig] - a unified interface for interacting
with various LLM providers (OpenAI, Anthropic, Groq, Ollama, Cohere,
Cloudflare, DeepSeek, Google).

[ig]: https://github.com/inference-gateway/inference-gateway

## Build & Development Commands

```bash
# Build
cargo build

# Run tests
cargo test --all-targets --all-features

# Run a single test
cargo test test_name

# Lint (format check)
cargo fmt --all -- --check

# Static analysis
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt
```

Using Taskfile (if `task` is installed):

```bash
task test      # Run tests
task lint      # Run linter
task analyse   # Run static analysis
```

## Architecture

This is a single-file library crate (`src/lib.rs`) with ~1900 lines containing:

### Core Types

- **`InferenceGatewayClient`**: HTTP client wrapping reqwest, configured with
  base URL and optional auth token
- **`InferenceGatewayAPI`**: Trait defining the API interface (list_models,
  generate_content, generate_content_stream, list_tools, health_check)
- **`Provider`**: Enum of supported LLM providers (Ollama, Groq, OpenAI,
  Cloudflare, Cohere, Anthropic, Deepseek, Google)
- **`GatewayError`**: Error enum using thiserror for HTTP status codes and
  request errors

### Request/Response Types

- `Message`, `MessageRole`: Chat message structures
- `CreateChatCompletionRequest/Response`: Non-streaming generation
- `CreateChatCompletionStreamResponse`, `SSEvents`: Streaming via SSE
- `Tool`, `FunctionObject`, `ToolType`: Function calling support
- `ListModelsResponse`, `ListToolsResponse`: API listing responses

### Key Patterns

- Builder pattern for client configuration: `.with_token()`, `.with_tools()`, `.with_max_tokens()`
- Async/await with tokio runtime
- Streaming uses `async_stream` crate with `futures_util::Stream`
- All serialization via serde with `#[serde(rename_all = "lowercase")]` for enums

## Testing

Tests use `mockito` for HTTP mocking. Tests are in `#[cfg(test)] mod tests`
at bottom of `lib.rs`. Run with:

```bash
cargo test --verbose
```

## Commit Message Convention

Use conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `style:`,
`test:`, `chore:`, `ci:`, `perf:`
