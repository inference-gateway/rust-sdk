# Inference Gateway Rust SDK

An SDK written in Rust for the [Inference Gateway](https://github.com/inference-gateway/inference-gateway).

- [Inference Gateway Rust SDK](#inference-gateway-rust-sdk)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Creating a Client](#creating-a-client)
    - [Listing Models](#listing-models)
    - [Generating Content](#generating-content)
    - [Health Check](#health-check)
  - [Contributing](#contributing)
  - [License](#license)

## Installation

Run `cargo add inference-gateway-sdk`.

## Usage

### Creating a Client

```rust
use inference_gateway_sdk::{InferenceGatewayClient, Message, Provider};
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
            info!("Model: {:?}", model.id);
        }
    }

    let response = client.generate_content(
        Provider::Ollama,
        "llama2",
        vec![Message {
            role: "user".to_string(),
            content: "Tell me a joke".to_string(),
        }],
    )?;

    info!("Response: {:?}", response);
    Ok(())
}
```

### Listing Models

To list available models, use the `list_models` method:

```rust
use log::info;

// List all providers and models
let models = client.list_models()?;
for provider_models in models {
    info!("Provider: {:?}", provider_models.provider);
    for model in provider_models.models {
        info!("Model: {:?}", model.id);
    }
}

// List models for a specific provider
let models = client.list_models_for_provider(Provider::Ollama)?;
for model in models {
    info!("Model: {:?}", model.id);
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
        role: "user".to_string(),
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
