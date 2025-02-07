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
    - [Health Check](#health-check)
    - [Tool-Use](#tool-use)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

Run `cargo add inference-gateway-sdk`.

## Usage

### Creating a Client

Here is a full example of how to create a client and interact with the Inference Gateway API:

```rust
use inference_gateway_sdk::{
    GatewayError,
    InferenceGatewayAPI,
    InferenceGatewayClient,
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
    let models = client.list_models().await?;
    for provider_models in models {
        info!("Provider: {:?}", provider_models.provider);
        for model in provider_models.models {
            info!("Model: {:?}", model.name);
        }
    }

    // List models for a specific provider
    let resp = client.list_models_by_provider(Provider::Groq).await?;
    let models = resp.models;
    info!("Provider: {:?}", resp.provider);
    for model in models {
        info!("Model: {:?}", model.name);
    }

    // Generate content - choose from available providers and models
    let resp = client.generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", vec![
    Message{
        role: MessageRole::System,
        content: "You are an helpful assistent.".to_string()
    },
    Message{
        role: MessageRole::User,
        content: "Tell me a funny joke".to_string()
    }
    ]).await?;

    log::info!("Generated from provider: {:?}", resp.provider);
    log::info!("Generated response: {:?}", resp.response.role);
    log::info!("Generated content: {:?}", resp.response.content);

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
    Message,
};
use log::info;

#[tokio::main]
fn main() -> Result<(), GatewayError> {
    // ...Create a client

    // List all models and all providers
    let models = client.list_models().await?;
    for provider_models in models {
        info!("Provider: {:?}", provider_models.provider);
        for model in provider_models.models {
            info!("Model: {:?}", model.name);
        }
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
    Provider,
};
use log::info;

// ...Open main function

// List models for a specific provider
let resp = client.list_models_by_provider(Provider::Ollama).await?;
let models = resp.models;
info!("Provider: {:?}", resp.provider);
for model in models {
    info!("Model: {:?}", model.name);
}

// ...Rest of the main function
```

### Generating Content

To generate content using a model, use the `generate_content` method:

```rust
use inference_gateway_sdk::{
    GatewayError,
    InferenceGatewayAPI,
    InferenceGatewayClient,
    Message,
    Provider,
    MessageRole
};

// Generate content - choose from available providers and models
let resp = client.generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", vec![
Message{
    role: MessageRole::System,
    content: "You are an helpful assistent.".to_string()
},
Message{
    role: MessageRole::User,
    content: "Tell me a funny joke".to_string()
}
]).await?;

log::info!("Generated from provider: {:?}", resp.provider);
log::info!("Generated response: {:?}", resp.response.role);
log::info!("Generated content: {:?}", resp.response.content);
```

### Streaming Content

You need to add the following tiny dependencies:

- `futures-util` for the `StreamExt` trait
- `serde` with feature `derive` and `serde_json` for serialization and deserialization of the response content

```rust
use inference_gateway_sdk::{
    InferenceGatewayAPI,
    InferenceGatewayClient, Message, MessageRole, Provider, ResponseContent
};
use futures_util::{StreamExt, pin_mut};
// ...rest of the imports

// ...main function
    let system_message = "You are an helpful assistent.".to_string();
    let model = "deepseek-r1-distill-llama-70b";
    let messages = vec![
        Message {
            role: MessageRole::System,
            content: system_message,
        },
        Message {
            role: MessageRole::User,
            content: "Write a poem".to_string(),
        },
    ];
    let client = InferenceGatewayClient::new("http://localhost:8080");
    let stream = client.generate_content_stream(Provider::Groq, model, messages);
    pin_mut!(stream);
    let content_delta = Some("content-delta".to_string());
    // Iterate over the stream of Server Sent Events
    while let Some(ssevent) = stream.next().await {
        let resp = ssevent?;

        // Only content-delta events contains the actual tokens
        // There are also events like:
        // - content-start
        // - content-end
        // - etc..
        if resp.event != content_delta {
            continue;
        }

        // Deserialize the event response
        let generate_response: ResponseContent = serde_json::from_str(&resp.data)?;
        // Print the token out as it's being sent from the server
        print!("{}", generate_response.content);
    }
// ...rest of the main function
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
    ToolType,
    ToolParameters,
    ToolParameterType,
};

let resp = client.generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", vec![
Message{
    role: MessageRole::System,
    content: "You are an helpful assistent.".to_string()
},
Message{
    role: MessageRole::User,
    content: "What is the current weather in Berlin?".to_string()
}
], vec![
    Tool {
        r#type: ToolType::Function,
        name: "get_current_weather".to_string(),
        description: "Get the weather for a location".to_string(),
        parameters: ToolParameters {
            name: "location".to_string(),
            r#type: ToolParameterType::String,
            default: None,
            description: "The city name".to_string(),
        },
    },
]).await?;
```

## Contributing

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for information about how to get involved. We welcome issues, questions, and pull requests.

## License

This SDK is distributed under the MIT License, see [LICENSE](LICENSE) for more information.

```

```
