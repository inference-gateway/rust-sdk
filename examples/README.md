# Examples

This directory contains examples that demonstrate how to use the Rust SDK.

## Pre-requisites

You should have Docker installed (to run the Inference Gateway) and a recent
stable Rust toolchain (1.94+ recommended; the SDK uses `edition = "2024"`).

## Quick Start

1. Copy the `.env.example` file to `.env` and fill in your API key(s).

2. Start the Inference Gateway locally:

   ```bash
   docker run --rm -p 8080:8080 --env-file .env ghcr.io/inference-gateway/inference-gateway:latest
   ```

3. On another terminal, export the provider and the LLM you intend to use:

   ```bash
   export PROVIDER=deepseek
   export LLM=deepseek-v4-flash
   ```

4. Review the different examples in the specific directories:

   - [List](./list): Demonstrates listing models, MCP tools, and health checks.
   - [Chat](./chat): Shows chat completions, streaming responses, multi-turn
     conversations, and function calling.

## Examples Overview

### [List Example](./list)

**Purpose**: Explore available models and MCP tools across providers

**Features**:

- List all available models across providers
- Filter models by specific provider
- Discover MCP tools and their schemas
- Health check validation

**Best for**: Understanding what's available in your Inference Gateway setup

### [Chat Example](./chat)

**Purpose**: Demonstrate various chat completion patterns

**Features**:

- Simple request/response chat completions
- Real-time streaming responses
- Multi-turn conversation handling
- Function/tool calling with AI models

**Best for**: Building chat applications and understanding different
interaction patterns

## Running Examples

Each example is a standalone binary crate in this workspace. From the SDK
root:

```bash
# List example
cargo run -p list-example

# Chat example (requires PROVIDER and LLM env vars)
PROVIDER=deepseek LLM=deepseek-v4-flash cargo run -p chat-example
```

Or, from inside an example directory:

```bash
cd list
cargo run
```

## Environment Variables

All examples support these environment variables:

- `PROVIDER` - AI provider to use (`groq`, `openai`, `anthropic`, `deepseek`,
  `google`, `cohere`, `cloudflare`, `ollama`, `ollama_cloud`, `mistral`,
  `moonshot`)
- `LLM` - Specific model to use (e.g. `meta-llama/llama-3.3-70b-versatile`)
- `INFERENCE_GATEWAY_URL` - Override the gateway base URL (defaults to
  `http://localhost:8080/v1`)

Provider-specific API keys should be set in the gateway's `.env` file (see
`.env.example`).

## Example Combinations

You can combine concepts from different examples:

1. **List + Chat**: Discover available models, then use them for chat
2. **Chat + MCP**: Use function calling with MCP tools for enhanced
   capabilities
3. **List + MCP**: Explore MCP tools, then integrate them into conversations
