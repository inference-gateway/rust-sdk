# AGENTS.md - Inference Gateway Rust SDK

This document provides comprehensive guidance for AI agents working with the
Inference Gateway Rust SDK project.

## Project Overview

**Inference Gateway Rust SDK** is a Rust client library for interacting with the
[Inference Gateway](https://github.com/inference-gateway/inference-gateway) - a
unified interface for various LLM providers including OpenAI, Anthropic, Groq,
Ollama, Cohere, Cloudflare, DeepSeek, and Google.

### Key Technologies

- **Language**: Rust (2021 edition)
- **HTTP Client**: reqwest with async/await
- **Serialization**: serde with JSON
- **Async Runtime**: tokio
- **Testing**: mockito for HTTP mocking
- **Build System**: Cargo
- **Task Runner**: Task (Taskfile.yml)

### Project Structure

```text
.
├── src/
│   └── lib.rs              # Main library code (~1900 lines)
├── .github/workflows/
│   ├── ci.yml              # CI pipeline
│   └── release.yml         # Release automation
├── Cargo.toml              # Package configuration
├── Taskfile.yml            # Task automation
├── CLAUDE.md               # Claude-specific guidance
├── README.md               # User documentation
└── openapi.yaml            # OpenAPI specification
```

## Architecture

### Core Components

1. **`InferenceGatewayClient`** - Main HTTP client with builder pattern
2. **`InferenceGatewayAPI`** - Trait defining API interface
3. **`Provider`** - Enum of supported LLM providers
4. **`GatewayError`** - Comprehensive error handling with thiserror

### Key Design Patterns

- **Builder pattern** for client configuration: `.with_token()`, `.with_tools()`,
  `.with_max_tokens()`
- **Async/await** with tokio runtime
- **Streaming support** using `async_stream` crate
- **OpenAI-compatible** API design
- **MCP (Model Context Protocol)** tool support

### Supported Providers

- `Provider::Ollama` - Local language model server
- `Provider::Groq` - High-speed inference provider
- `Provider::OpenAI` - GPT models and services
- `Provider::Cloudflare` - Cloudflare Workers AI
- `Provider::Cohere` - Cohere language models
- `Provider::Anthropic` - Claude models
- `Provider::Deepseek` - DeepSeek models
- `Provider::Google` - Google Gemini models

## Development Environment Setup

### Prerequisites

- Rust toolchain (via rustup)
- Docker (for development container)
- Task runner (`task` command)

### Quick Setup

```bash
# Clone repository
git clone https://github.com/inference-gateway/rust-sdk
cd rust-sdk

# Build project
cargo build

# Run tests
cargo test
```

### Development Container (Recommended)

1. Open in VS Code
2. Click "Reopen in Container" button
3. Development environment is automatically configured

## Key Commands

### Using Cargo

```bash
# Build
cargo build

# Run tests
cargo test --all-targets --all-features

# Run specific test
cargo test test_name

# Lint (format check)
cargo fmt --all -- --check

# Static analysis
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Build documentation
cargo doc --open
```

### Using Task (Taskfile.yml)

```bash
# Run tests
task test

# Run linter
task lint

# Run static analysis
task analyse

# Download OpenAPI spec
task oas-download
```

## Testing

### Test Structure

- All tests are in `#[cfg(test)] mod tests` at bottom of `src/lib.rs`
- Uses `mockito` for HTTP mocking
- Async tests with `#[tokio::test]`
- Comprehensive error handling tests

### Running Tests

```bash
# Run all tests
cargo test --all-targets --all-features

# Run tests with verbose output
cargo test --verbose

# Run specific test module
cargo test tests::test_name

# Run tests with coverage (if cargo-tarpaulin installed)
cargo tarpaulin --ignore-tests
```

### Test Coverage

- Provider enum serialization/deserialization
- HTTP client with authentication
- Error handling for various HTTP status codes
- MCP tool listing functionality
- Streaming content generation
- Tool-use functionality

## Development Workflow

### Branch Strategy

- Create feature branches: `git checkout -b feature/my-feature`
- Use conventional commit messages
- Submit PRs to `main` branch

### Conventional Commits

```bash
feat: Add new feature
fix: Bug fix
refactor: Code refactoring
docs: Documentation updates
style: Code style changes
test: Test additions/changes
chore: Maintenance tasks
ci: CI/CD changes
perf: Performance improvements
```

### CI/CD Pipeline

- **CI**: Runs on push/PR to `main`
  - Linting (`cargo fmt --check`)
  - Static analysis (`cargo clippy`)
  - Build verification
  - Test execution
- **Release**: Manual trigger from GitHub Actions
  - Semantic versioning
  - Changelog generation
  - Cargo.toml version update
  - GitHub release creation

### Release Process

1. Merge to `main` triggers CI checks
2. Manual release workflow from GitHub Actions
3. Version determined by commit messages
4. Changelog automatically generated
5. Cargo.toml version updated automatically
