# Inference Gateway Rust SDK

An SDK written in Rust for the [Inference Gateway](https://github.com/edenreich/inference-gateway).

- [Inference Gateway Rust SDK](#inference-gateway-rust-sdk)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Creating a Client](#creating-a-client)
    - [Listing Models](#listing-models)
    - [Generating Content](#generating-content)
    - [Health Check](#health-check)
  - [License](#license)

## Installation

Add this to your `Cargo.toml` or run `cargo add inference-gateway-rust-sdk`:

```toml
[dependencies]
inference-gateway-rust-sdk = "0.1.0"
```

## Usage

### Creating a Client

```rust
use inference_gateway_rust_sdk::{InferenceGatewayClient, Provider};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = InferenceGatewayClient::new("http://localhost:8080");

    // List available models
    let models = client.list_models()?;
    for provider_models in models {
        println!("Provider: {}", provider_models.provider);
        for model in provider_models.models {
            println!("  Model: {}", model.id);
        }
    }

    // Generate content
    let response = client.generate_content(
        Provider::Ollama,
        "llama2",
        "Tell me a joke"
    )?;
    println!("Response: {}", response.response);

    Ok(())
}
```

### Listing Models

To list available models, use the `list_models` method:

```rust
let models = client.list_models()?;
for provider_models in models {
    println!("Provider: {}", provider_models.provider);
    for model in provider_models.models {
        println!("  Model: {}", model.id);
    }
}
```

### Generating Content

To generate content using a model, use the `generate_content` method:

```rust
let response = client.generate_content(
    Provider::Ollama,
    "llama2",
    "Tell me a joke"
)?;
println!("Provider: {}", response.provider);
println!("Response: {}", response.response);
```

### Health Check

To check if the Inference Gateway is running, use the `health_check` method:

```rust
let is_healthy = client.health_check()?;
println!("API is healthy: {}", is_healthy);
```

## License

This SDK is distributed under the MIT License, see [LICENSE](LICENSE) for more information.
