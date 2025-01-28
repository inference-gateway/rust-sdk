# Inference Gateway Rust SDK

An SDK written in Rust for the [Inference Gateway](https://github.com/inference-gateway/inference-gateway).

- [Inference Gateway Rust SDK](#inference-gateway-rust-sdk)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Creating a Client](#creating-a-client)
    - [Listing Models](#listing-models)
    - [Listing Models from a specific provider](#listing-models-from-a-specific-provider)
    - [Generating Content](#generating-content)
    - [Health Check](#health-check)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

Run `cargo add inference-gateway-sdk`.

## Usage

### Creating a Client

```rust
use inference_gateway_sdk::{InferenceGatewayClient, Message, Provider, MessageRole};
use log::info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client = InferenceGatewayClient::new("http://localhost:8080");

    // List available models
    let models = client.list_models()?;
    for provider_models in models {
        info!("Provider: {:?}", provider_models.provider);
        for model in provider_models.models {
            info!("Model: {:?}", model.name);
        }
    }

    let response = client.generate_content(
        Provider::Ollama,
        "llama2",
        vec![Message {
            role: MessageRole::User,
            content: "Tell me a joke".to_string(),
        }],
    )?;

    info!("Response: {:?}", response);
    Ok(())
}
```

### Listing Models

To list all available models from all configured providers, use the `list_models` method:

```rust
use inference_gateway_sdk::{InferenceGatewayClient, Message, Provider, MessageRole};
use log::info;

fn main() -> Result<(), Box<dyn Error>> {
    // ...create a client

    // List all providers and models
    let models = client.list_models()?;
    for provider_models in models {
        info!("Provider: {:?}", provider_models.provider);
        for model in provider_models.models {
            info!("Model: {:?}", model.name);
        }
    }

    // List models for a specific provider
    let resp = client.list_models_by_provider(Provider::Ollama)?;
    let models = resp.models;
    info!("Provider: {:?}", resp.provider);
    for model in models {
        info!("Model: {:?}", model.name);
    }
}
```

### Listing Models from a specific provider

To list all available models from a specific provider, use the `list_models_by_provider` method:

```rust
use log::info;

let resp = client.list_models_by_provider(Provider::Ollama)?;
let models = resp.models;
info!("Provider: {:?}", resp.provider);
for model in models {
    info!("Model: {:?}", model.name);
}
```

### Generating Content

To generate content using a model, use the `generate_content` method:

```rust
use log::info;

let response = client.generate_content(
    Provider::Ollama,
    "llama2",
    vec![Message {
        role: MessageRole::User,
        content: "Tell me a joke".to_string(),
    }],
)?;

info!("Provider: {:?}", response.provider);
info!("Response: {:?}", response.response);
```

### Health Check

To check if the Inference Gateway is running, use the `health_check` method:

```rust
use log::info;

let is_healthy = client.health_check()?;
info!("API is healthy: {}", is_healthy);
```

## Contributing

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for information about how to get involved. We welcome issues, questions, and pull requests.

## License

This SDK is distributed under the MIT License, see [LICENSE](LICENSE) for more information.
