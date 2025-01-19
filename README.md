# Inference Gateway Rust SDK

An SDK written in Rust for the [Inference Gateway](https://github.com/edenreich/inference-gateway).

- [Inference Gateway Rust SDK](#inference-gateway-rust-sdk)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Creating a Client](#creating-a-client)
    - [Listing Models](#listing-models)
    - [Generating Content](#generating-content)
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
use inference_gateway_rust_sdk::InferenceGatewayClient;

fn main() {
    let client = InferenceGatewayClient::new("http://localhost:8080");

    let models = client.list_models();
    println!("Available models: {:?}", models);

    let response = client.generate_content("providerName", "modelName", "your prompt here");
    println!("Generated content: {:?}", response["Response"]["Content"]);
}
```

### Listing Models

To list available models, use the `list_models` method:

```rust
let models = client.list_models();
println!("Available models: {:?}", models);
```

### Generating Content

To generate content using a model, use the `generate_content` method:

```rust
let response = client.generate_content("providerName", "modelName", "your prompt here");
println!("Generated content: {:?}", response["Response"]["Content"]);
```

## License

This SDK is distributed under the MIT License, see [LICENSE](LICENSE) for more information.
