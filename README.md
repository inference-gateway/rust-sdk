<div align="center">

# 🦀 Inference Gateway Rust SDK

### A powerful and easy-to-use Rust SDK for the Inference Gateway

[![Crates.io](https://img.shields.io/crates/v/inference-gateway-sdk.svg)](https://crates.io/crates/inference-gateway-sdk)
[![Docs.rs](https://img.shields.io/docsrs/inference-gateway-sdk)](https://docs.rs/inference-gateway-sdk)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Release](https://img.shields.io/github/release/inference-gateway/rust-sdk.svg)](https://github.com/inference-gateway/rust-sdk/releases)
[![Rust Edition](https://img.shields.io/badge/rust-2024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/)

Connect to multiple LLM providers through a unified interface • Stream responses • Function calling • MCP tools support

[Installation](#installation) • [Quick Start](#usage) • [Examples](#examples) • [Documentation](#documentation)

</div>

---

- [🦀 Inference Gateway Rust SDK](#-inference-gateway-rust-sdk)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Creating a Client](#creating-a-client)
    - [Listing Models](#listing-models)
    - [Listing Models from a specific provider](#listing-models-from-a-specific-provider)
    - [Listing MCP Tools](#listing-mcp-tools)
    - [Generating Content](#generating-content)
    - [Streaming Content](#streaming-content)
    - [Messages API (Anthropic-compatible)](#messages-api-anthropic-compatible)
    - [Tool-Use](#tool-use)
    - [Health Check](#health-check)
  - [Examples](#examples)
  - [Supported Providers](#supported-providers)
  - [Documentation](#documentation)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

Run `cargo add inference-gateway-sdk`.

## Usage

### Creating a Client

Here is a full example of how to create a client and interact with the
Inference Gateway API:

```rust
use inference_gateway_sdk::{
    CreateChatCompletionResponse, GatewayError, InferenceGatewayAPI,
    InferenceGatewayClient, ListModelsResponse, ListToolsResponse, Message,
    MessageContent, MessageRole, Provider,
};
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // Create a client
    let client = InferenceGatewayClient::new("http://localhost:8080/v1");

    // List all models and all providers
    let response: ListModelsResponse = client.list_models().await?;
    for model in response.data {
        info!("Model: {:?}", model.id);
    }

    // List models for a specific provider
    let response: ListModelsResponse = client.list_models_by_provider(Provider::Groq).await?;
    info!("Models for provider: {:?}", response.provider);
    for model in response.data {
        info!("Model: {:?}", model.id);
    }

    // Generate content - choose from available providers and models
    let response: CreateChatCompletionResponse = client
        .generate_content(
            Provider::Deepseek,
            "deepseek-v4-flash",
            vec![
                Message {
                    role: MessageRole::System,
                    content: MessageContent::String("You are a helpful assistant.".to_string()),
                    reasoning: None,
                    reasoning_content: None,
                    tool_call_id: None,
                    tool_calls: Vec::new(),
                },
                Message {
                    role: MessageRole::User,
                    content: MessageContent::String("Tell me a funny joke".to_string()),
                    reasoning: None,
                    reasoning_content: None,
                    tool_call_id: None,
                    tool_calls: Vec::new(),
                },
            ],
        )
        .await?;

    log::info!(
        "Generated content: {:?}",
        response.choices[0].message.content
    );

    Ok(())
}
```

### Listing Models

To list all available models from all configured providers, use the
`list_models` method:

```rust
use inference_gateway_sdk::{
    GatewayError
    InferenceGatewayAPI,
    InferenceGatewayClient,
    ListModelsResponse,
    Message,
};
use log::info;

#[tokio::main]
fn main() -> Result<(), GatewayError> {
    // ...Create a client

    // List models from all providers
    let response: ListModelsResponse = client.list_models().await?;
    for model in response.data {
        info!("Model: {:?}", model.id);
    }

    // ...
}
```

### Listing Models from a specific provider

To list all available models from a specific provider, use the
`list_models_by_provider` method:

```rust
use inference_gateway_sdk::{
    GatewayError
    InferenceGatewayAPI,
    InferenceGatewayClient,
    ListModelsResponse,
    Provider,
};
use log::info;

// ...Open main function

// List models for a specific provider
let response: ListModelsResponse = client.list_models_by_provider(Provider::Groq).await?;
info!("Models for provider: {:?}", response.provider);
for model in response.data {
    info!("Model: {:?}", model.id);
}

// Example with Google provider
let response: ListModelsResponse = client.list_models_by_provider(Provider::Google).await?;
info!("Google models: {:?}", response.provider);
for model in response.data {
    info!("Google Model: {:?}", model.id);
}

// ...Rest of the main function
```

### Listing MCP Tools

To list all available MCP (Model Context Protocol) tools from all configured
MCP servers, use the `list_tools` method:

```rust
use inference_gateway_sdk::{
    GatewayError,
    InferenceGatewayAPI,
    InferenceGatewayClient,
    ListToolsResponse,
};
use log::info;

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    // ...Create a client

    // List all MCP tools from all configured servers
    let response: ListToolsResponse = client.list_tools().await?;
    info!("Found {} MCP tools", response.data.len());

    for tool in response.data {
        info!("Tool: {} from server: {}", tool.name, tool.server);
        info!("Description: {}", tool.description);
        if !tool.input_schema.is_empty() {
            info!("Input schema: {:?}", tool.input_schema);
        }
    }

    Ok(())
}
```

Note: This functionality requires that MCP servers are configured and exposed
in your Inference Gateway instance. If MCP is not exposed, you'll receive a
`403 Forbidden` error.

### Generating Content

To generate content using a model, use the `generate_content` method:

```rust
use inference_gateway_sdk::{
    CreateChatCompletionResponse, GatewayError, InferenceGatewayAPI, InferenceGatewayClient,
    Message, MessageContent, MessageRole, Provider,
};

fn message(role: MessageRole, text: &str) -> Message {
    Message {
        role,
        content: MessageContent::String(text.to_string()),
        reasoning: None,
        reasoning_content: None,
        tool_call_id: None,
        tool_calls: Vec::new(),
    }
}

// Generate content - choose from available providers and models
let response: CreateChatCompletionResponse = client
    .generate_content(
        Provider::Deepseek,
        "deepseek-v4-flash",
        vec![
            message(MessageRole::System, "You are a helpful assistant."),
            message(MessageRole::User, "Tell me a funny joke"),
        ],
    )
    .await?;

log::info!(
    "Generated content: {:?}",
    response.choices[0].message.content
);

// Example with Google provider (Gemini models)
let response: CreateChatCompletionResponse = client
    .generate_content(
        Provider::Google,
        "gemini-3-pro",
        vec![
            message(MessageRole::System, "You are a helpful AI assistant."),
            message(MessageRole::User, "Explain quantum computing in simple terms"),
        ],
    )
    .await?;

log::info!(
    "Google generated content: {:?}",
    response.choices[0].message.content
);
```

### Streaming Content

```rust
use futures_util::{pin_mut, StreamExt};
use inference_gateway_sdk::{
    CreateChatCompletionStreamResponse, FinishReason, GatewayError, InferenceGatewayAPI,
    InferenceGatewayClient, Message, MessageContent, MessageRole, Provider,
};
use log::info;
use std::env;

fn message(role: MessageRole, text: &str) -> Message {
    Message {
        role,
        content: MessageContent::String(text.to_string()),
        reasoning: None,
        reasoning_content: None,
        tool_call_id: None,
        tool_calls: Vec::new(),
    }
}

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let model = "deepseek-v4-flash";

    let client = InferenceGatewayClient::new("http://localhost:8080/v1");
    let stream = client.generate_content_stream(
        Provider::Deepseek,
        model,
        vec![
            message(MessageRole::System, "You are a helpful assistant."),
            message(MessageRole::User, "Write a poem"),
        ],
    );
    pin_mut!(stream);
    // Iterate over the stream of Server Sent Events
    while let Some(ssevent) = stream.next().await {
        let ssevent = ssevent?;
        if ssevent.data == "[DONE]" {
            break;
        }

        // Deserialize the event response
        let generate_response_stream: CreateChatCompletionStreamResponse =
            serde_json::from_str(&ssevent.data)?;

        let Some(choice) = generate_response_stream.choices.first() else {
            continue;
        };

        if let Some(usage) = generate_response_stream.usage.as_ref() {
            // Get the usage metrics from the response
            info!("Usage Metrics: {:?}", usage);
            // Probably send them over to a metrics service
            break;
        }

        // Print the token out as it's being sent from the server
        if let Some(content) = choice.delta.content.as_ref() {
            print!("{}", content);
        }

        if let Some(reason) = choice.finish_reason.as_ref() {
            if matches!(reason, FinishReason::Stop) {
                info!("Finished generating content");
                break;
            }
        }
    }

    Ok(())
}
```

### Messages API (Anthropic-compatible)

The gateway also exposes an Anthropic-compatible `POST /messages` endpoint.
Providers without Messages support return `GatewayError::BadRequest`; use
`generate_content` for those providers.

```rust
use futures_util::{pin_mut, StreamExt};
use inference_gateway_sdk::{
    CreateMessagesRequest, GatewayError, InferenceGatewayAPI, InferenceGatewayClient,
    MessagesMessage, MessagesMessageContent, MessagesMessageRole, MessagesResponseContentBlock,
    MessagesStreamEvent, MessagesStreamEventType, Provider,
};

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    let client = InferenceGatewayClient::new("http://localhost:8080/v1");

    let request = CreateMessagesRequest {
        max_tokens: 300,
        messages: vec![MessagesMessage {
            role: MessagesMessageRole::User,
            content: MessagesMessageContent::String("Tell me a fun fact about Rust.".to_string()),
        }],
        metadata: None,
        model: "claude-sonnet-5".to_string(),
        stop_sequences: Vec::new(),
        stream: false,
        system: None,
        temperature: None,
        thinking: None,
        tool_choice: None,
        tools: Vec::new(),
        top_k: None,
        top_p: None,
    };

    // Non-streaming
    let response = client
        .create_message(Some(Provider::Anthropic), request.clone())
        .await?;
    for block in &response.content {
        if let MessagesResponseContentBlock::TextBlock(text) = block {
            println!("{}", text.text);
        }
    }

    // Streaming - each event's `data` is a JSON-serialized `MessagesStreamEvent`
    let stream = client.create_message_stream(Some(Provider::Anthropic), request);
    pin_mut!(stream);
    while let Some(event) = stream.next().await {
        let event: MessagesStreamEvent = serde_json::from_str(&event?.data)?;
        match event.type_ {
            MessagesStreamEventType::ContentBlockDelta => {
                if let Some(text) = event.delta.as_ref().and_then(|d| d.text.as_deref()) {
                    print!("{text}");
                }
            }
            MessagesStreamEventType::MessageStop => break,
            _ => {}
        }
    }

    Ok(())
}
```

### Tool-Use

You can pass to the generate_content function also tools, which will be
available for the LLM to use:

```rust
use inference_gateway_sdk::{
    ChatCompletionTool, ChatCompletionToolType, FunctionObject,
    FunctionParameters, GatewayError, InferenceGatewayAPI,
    InferenceGatewayClient, Message, MessageContent, MessageRole, Provider,
};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

fn message(role: MessageRole, text: &str) -> Message {
    Message {
        role,
        content: MessageContent::String(text.to_string()),
        reasoning: None,
        reasoning_content: None,
        tool_call_id: None,
        tool_calls: Vec::new(),
    }
}

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    // Configure logging
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // API endpoint - store as a variable so we can reuse it
    let api_endpoint = "http://localhost:8080/v1";

    // Initialize the API client
    let client = InferenceGatewayClient::new(api_endpoint);

    // Define the model and provider
    let provider = Provider::Deepseek;
    let model = "deepseek-v4-flash";

    // Define the weather tool
    let parameters = json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The city name"
            }
        },
        "required": ["location"]
    });
    let tools = vec![ChatCompletionTool {
        type_: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "get_current_weather".to_string(),
            description: Some("Get the weather for a location".to_string()),
            parameters: Some(FunctionParameters(
                parameters.as_object().unwrap().clone(),
            )),
            strict: false,
        },
    }];

    // Create initial conversation
    let initial_messages = vec![
        message(
            MessageRole::System,
            "You are a helpful assistant that can check the weather.",
        ),
        message(MessageRole::User, "What is the current weather in Berlin?"),
    ];

    // Make the initial API request
    info!("Sending initial request to model");
    let response = client
        .with_tools(Some(tools.clone()))
        .generate_content(provider, model, initial_messages)
        .await?;

    info!("Received response from model");

    let Some(choice) = response.choices.first() else {
        warn!("No choice returned");
        return Ok(());
    };

    // Check for tool calls in the response
    if !choice.message.tool_calls.is_empty() {
        // Echo the assistant turn (with the tool_calls) before the tool replies.
        let mut assistant_turn = choice.message.clone();
        // Tool-call assistant messages typically have empty content; keep what
        // the model sent so providers that include explanatory text round-trip.
        let mut follow_up_convo = vec![
            message(
                MessageRole::System,
                "You are a helpful assistant that can check the weather.",
            ),
            message(MessageRole::User, "What is the current weather in Berlin?"),
        ];
        // Take the assistant message verbatim so tool_calls are preserved.
        assistant_turn.role = MessageRole::Assistant;
        follow_up_convo.push(assistant_turn);

        // Process each tool call
        for tool_call in &choice.message.tool_calls {
            info!("Tool Call Requested: {}", tool_call.function.name);

            if tool_call.function.name == "get_current_weather" {
                // Parse arguments into a typed value
                let args: Weather = tool_call.function.parse_arguments()?;
                let weather_result = get_current_weather(args);

                // Add the tool response to the conversation
                follow_up_convo.push(Message {
                    role: MessageRole::Tool,
                    content: MessageContent::String(weather_result),
                    reasoning: None,
                    reasoning_content: None,
                    tool_call_id: Some(tool_call.id.clone()),
                    tool_calls: Vec::new(),
                });
            }
        }

        // Send the follow-up request with the tool results
        info!("Sending follow-up request with tool results");

        let follow_up_client = InferenceGatewayClient::new(api_endpoint);
        let follow_up_response = follow_up_client
            .with_tools(Some(tools))
            .generate_content(provider, model, follow_up_convo)
            .await?;

        if let Some(choice) = follow_up_response.choices.first() {
            info!("Final response: {:?}", choice.message.content);
        } else {
            warn!("No response in follow-up");
        }
    } else {
        info!("No tool calls in the response");
        info!("Model response: {:?}", choice.message.content);
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    location: String,
}

fn get_current_weather(args: Weather) -> String {
    info!(
        "Getting weather function was called for {}",
        args.location
    );

    // In a real application, we would call an actual weather API here.
    // For this example, we'll just return a mock response.
    format!(
        "The weather in {} is currently sunny with a temperature of 22°C",
        args.location
    )
}
```

### Health Check

To check if the Inference Gateway is running, use the `health_check` method:

```rust
// ...rest of the imports
use log::info;

// ...main function
let is_healthy = client.health_check().await?;
info!("API is healthy: {}", is_healthy);
```

## Examples

Runnable examples live in the [examples directory](./examples/) - see its [README](./examples/README.md) for the full list and instructions.

## Supported Providers

The Inference Gateway Rust SDK supports the following providers:

- **Ollama** (`Provider::Ollama`) - Local language model server
- **Ollama Cloud** (`Provider::OllamaCloud`) - Ollama Cloud-hosted models
- **Groq** (`Provider::Groq`) - High-speed inference provider
- **llama.cpp** (`Provider::Llamacpp`) - llama.cpp OpenAI-compatible `llama-server`
- **OpenAI** (`Provider::Openai`) - GPT models and other OpenAI services
- **Cloudflare** (`Provider::Cloudflare`) - Cloudflare Workers AI
- **Cohere** (`Provider::Cohere`) - Cohere language models
- **Anthropic** (`Provider::Anthropic`) - Claude models
- **DeepSeek** (`Provider::Deepseek`) - DeepSeek models
- **Google** (`Provider::Google`) - Google Gemini models via Generative AI API
- **Mistral** (`Provider::Mistral`) - Mistral AI models
- **MiniMax** (`Provider::Minimax`) - MiniMax models
- **Moonshot** (`Provider::Moonshot`) - Moonshot AI models

Each provider may support different models and capabilities. Use the
`list_models_by_provider()` method to discover available models for each
provider.

Example:

```rust
use inference_gateway_sdk::{Provider, InferenceGatewayClient, InferenceGatewayAPI};

let client = InferenceGatewayClient::new("http://localhost:8080/v1");

// List Google models
let google_models = client.list_models_by_provider(Provider::Google).await?;
for model in google_models.data {
    println!("Google model: {}", model.id);
}
```

## Documentation

Full API documentation is published on [docs.rs](https://docs.rs/inference-gateway-sdk).

## Contributing

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for information
about how to get involved. We welcome issues, questions, and pull requests.

## License

This SDK is distributed under the Apache 2.0 License, see [LICENSE](LICENSE) for more information.
