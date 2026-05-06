# List Example

This example demonstrates how to use the Inference Gateway SDK for listing
models and MCP tools. It includes making requests to the SDK and handling
responses using the Rust SDK.

## Features Demonstrated

1. **List All Models** - Retrieve all available models across providers
2. **List Provider-Specific Models** - Filter models by provider
3. **List MCP Tools** - Discover available Model Context Protocol tools
4. **Health Check** - Verify gateway connectivity

## Getting Started

1. Ensure you have the Inference Gateway running locally or have access to an
   instance. If not, please read the [Quick Start](../README.md#quick-start)
   section in the main examples README.

2. (Optional) Set environment variables to see provider-specific examples:

   ```bash
   export PROVIDER=deepseek
   ```

3. Run the example from the SDK root:

   ```bash
   cargo run -p list-example
   ```

   Or from this directory:

   ```bash
   cargo run
   ```

## Example Output

The example will show:

- Complete list of models grouped by provider
- Provider-specific model details (if `PROVIDER` is set)
- Available MCP tools (if `MCP_EXPOSE=true` on the gateway)
- Gateway health status

## MCP Tools

To see MCP tools in action, ensure the gateway is started with MCP exposed:

```bash
docker run --rm -p 8080:8080 \
  -e MCP_ENABLE=true -e MCP_EXPOSE=true \
  --env-file .env \
  ghcr.io/inference-gateway/inference-gateway:latest
```

## Supported Operations

- **Model Discovery** - Find all available models
- **Provider Filtering** - Get models from specific providers
- **Tool Discovery** - List available MCP tools and their schemas
- **Health Monitoring** - Check gateway status
