# Chat Example

This example demonstrates how to use the Inference Gateway SDK for chat
applications. It includes creating chat completions, streaming responses,
multi-turn conversations, and function calling using the Rust SDK.

## Features Demonstrated

1. **Simple Chat Completion** - Basic request/response chat
2. **Streaming Chat Completion** - Real-time streaming responses
3. **Multi-turn Conversation** - Maintaining conversation context
4. **Function Calling** - Using tools/functions with models

## Getting Started

1. Ensure you have the Inference Gateway running locally or have access to an
   instance. If not, please read the [Quick Start](../README.md#quick-start)
   section in the main examples README.

2. (Optional) Set environment variables to override the default
   provider/model:

   ```bash
   export PROVIDER=deepseek
   export LLM=deepseek-v4-flash
   ```

   Or for OpenAI:

   ```bash
   export PROVIDER=openai
   export LLM=gpt-4o
   ```

3. Run the example from the SDK root:

   ```bash
   cargo run -p chat-example
   ```

   Or from this directory:

   ```bash
   cargo run
   ```

## Defaults

If `PROVIDER` and `LLM` are not set, the example uses:

- `PROVIDER=deepseek`
- `LLM=deepseek-v4-flash`

## Example Output

The example will demonstrate:

- A simple chat completion about Rust
- A streaming story about a robot learning to paint
- A multi-turn conversation about Rust programming
- Function calling for weather information (simulated)

## Supported Providers

This example works with any provider supported by the Inference Gateway:

- `deepseek` - DeepSeek models (default)
- `openai` - OpenAI models (GPT-4, GPT-3.5, etc.)
- `groq` - Groq's fast inference models
- `anthropic` - Claude models
- `ollama` - Local models via Ollama
- `cohere` - Cohere models
- `cloudflare` - Cloudflare Workers AI
- `google` - Google Gemini models
- `mistral` - Mistral AI models
- `moonshot` - Moonshot AI models

## Notes

- The function calling example simulates a weather API call - in a real
  application, you would implement actual function execution.
- Streaming responses provide real-time output, perfect for interactive
  applications.
- Multi-turn conversations maintain context across multiple exchanges.
- The `max_tokens` parameter can be adjusted on the client via
  `.with_max_tokens(Some(...))` for different use cases.
