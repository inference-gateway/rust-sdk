# Inference Gateway Rust SDK

An SDK written in Rust for the [Inference Gateway](https://github.com/inference-gateway/inference-gateway).

- [Inference Gateway Rust SDK](#inference-gateway-rust-sdk)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Creating a Client](#creating-a-client)
    - [Listing Models](#listing-models)
    - [Listing Models from a specific provider](#listing-models-from-a-specific-provider)
    - [Generating Content](#generating-content)
    - [Streaming Content](#streaming-content)
    - [Tool-Use](#tool-use)
    - [Health Check](#health-check)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

Run `cargo add inference-gateway-sdk`.

## Usage

### Creating a Client

Here is a full example of how to create a client and interact with the Inference Gateway API:

```rust
use inference_gateway_sdk::{
    CreateChatCompletionResponse,
    GatewayError,
    InferenceGatewayAPI,
    InferenceGatewayClient,
    ListModelsResponse,
    Message,
    Provider,
    MessageRole
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
    let client = InferenceGatewayClient::new("http://localhost:8080");

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
    let response: CreateChatCompletionResponse = client.generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", vec![
    Message{
        role: MessageRole::System,
        content: "You are an helpful assistent.".to_string()
    },
    Message{
        role: MessageRole::User,
        content: "Tell me a funny joke".to_string()
    }
    ]).await?;

    log::info!(
        "Generated content: {:?}",
        response.choices[0].message.content
    );

    Ok(())
}
```

### Listing Models

To list all available models from all configured providers, use the `list_models` method:

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

To list all available models from a specific provider, use the `list_models_by_provider` method:

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

// ...Rest of the main function
```

### Generating Content

To generate content using a model, use the `generate_content` method:

```rust
use inference_gateway_sdk::{
    CreateChatCompletionResponse,
    GatewayError,
    InferenceGatewayAPI,
    InferenceGatewayClient,
    Message,
    Provider,
    MessageRole
};

// Generate content - choose from available providers and models
let response: CreateChatCompletionResponse = client.generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", vec![
Message{
    role: MessageRole::System,
    content: "You are an helpful assistent.".to_string(),
    ..Default::default()
},
Message{
    role: MessageRole::User,
    content: "Tell me a funny joke".to_string(),
    ..Default::default()
}
]).await?;

log::info!(
    "Generated content: {:?}",
    response.choices[0].message.content
);
```

### Streaming Content

You need to add the following tiny dependencies:

- `futures-util` for the `StreamExt` trait
- `serde` with feature `derive` and `serde_json` for serialization and deserialization of the response content

```rust
use futures_util::{pin_mut, StreamExt};
use inference_gateway_sdk::{
    CreateChatCompletionStreamResponse, GatewayError, InferenceGatewayAPI, InferenceGatewayClient,
    Message, MessageRole, Provider,
};
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let system_message = "You are an helpful assistent.".to_string();
    let model = "deepseek-r1-distill-llama-70b";

    let client = InferenceGatewayClient::new("http://localhost:8080/v1");
    let stream = client.generate_content_stream(
        Provider::Groq,
        model,
        vec![
            Message {
                role: MessageRole::System,
                content: system_message,
                ..Default::default()
            },
            Message {
                role: MessageRole::User,
                content: "Write a poem".to_string(),
                ..Default::default()
            },
        ],
    );
    pin_mut!(stream);
    // Iterate over the stream of Server Sent Events
    while let Some(ssevent) = stream.next().await {
        let ssevent = ssevent?;

        // Deserialize the event response
        let generate_response_stream: CreateChatCompletionStreamResponse =
            serde_json::from_str(&ssevent.data)?;

        let choice = generate_response_stream.choices.get(0);
        if choice.is_none() {
            continue;
        }
        let choice = choice.unwrap();

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

        if let Some(finish_reason) = choice.finish_reason.as_ref() {
            if finish_reason == "stop" {
                info!("Finished generating content");
                break;
            }
        }
    }

    Ok(())
}
```

### Tool-Use

You can pass to the generate_content function also tools, which will be available for the LLM to use:

```rust
use inference_gateway_sdk::{
    GatewayError,
    InferenceGatewayAPI,
    InferenceGatewayClient,
    Message,
    Provider,
    MessageRole,
    Tool,
    ToolFunction,
    ToolType
};

let tools = vec![
    Tool {
        r#type: ToolType::Function,
        function: ToolFunction {
            name: "get_current_weather".to_string(),
            description: "Get the weather for a location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city name"
                    }
                },
                "required": ["location"]
            }),
        },
    },
];
let resp = client.with_tools(Some(tools)).generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", vec![
Message {
    role: MessageRole::System,
    content: "You are an helpful assistent.".to_string(),
    ..Default::default()
},
Message {
    role: MessageRole::User,
    content: "What is the current weather in Berlin?".to_string(),
    ..Default::default()
}
]).await?;

for tool_call in resp.response.tool_calls {
    log::info!("Tool Call Requested by the LLM: {:?}", tool_call);
    // Make the function call with the parameters requested by the LLM

    let message = Message {
        role: MessageRole::Tool,
        content: "The content from the tool".to_string(),
        tool_call_id: Some(tool_call.id), // the tool call id so the LLM can reference it
        ..Default::default()
    };

    // Append this message to the next request
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

## Contributing

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for information about how to get involved. We welcome issues, questions, and pull requests.

## License

This SDK is distributed under the MIT License, see [LICENSE](LICENSE) for more information.
