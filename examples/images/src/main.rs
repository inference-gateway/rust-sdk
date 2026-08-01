use std::env;
use std::num::NonZeroU64;
use std::process::ExitCode;

use inference_gateway_sdk::{
    CreateImageEditRequest, CreateImageRequest, CreateImageRequestResponseFormat,
    CreateImageVariationRequest, GatewayError, ImageSize, ImagesResponse, InferenceGatewayAPI,
    InferenceGatewayClient, Provider,
};

const DEFAULT_PROVIDER: &str = "openai";
const DEFAULT_MODEL: &str = "dall-e-2";

fn print_images(response: &ImagesResponse) {
    for (i, image) in response.data.iter().enumerate() {
        match (&image.url, &image.b64_json) {
            (Some(url), _) => println!("Image {}: {url}", i + 1),
            (None, Some(b64)) => println!("Image {}: {} bytes of base64 data", i + 1, b64.len()),
            (None, None) => println!("Image {}: no data", i + 1),
        }
    }
    if let Some(usage) = response.usage.as_ref() {
        println!("Usage: total={} tokens", usage.total_tokens);
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error in images examples: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), GatewayError> {
    let base_url = env::var("INFERENCE_GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    let provider_str = env::var("PROVIDER").unwrap_or_else(|_| DEFAULT_PROVIDER.to_string());
    let model = env::var("LLM").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    let provider: Provider = provider_str.parse().map_err(|e| {
        GatewayError::Other(Box::new(std::io::Error::other(format!(
            "invalid PROVIDER '{provider_str}': {e}"
        ))))
    })?;

    println!("Using provider: {provider_str}");
    println!("Using model: {model}");
    println!("---");

    let client = InferenceGatewayClient::new(&base_url);

    // Example 1: Image generation
    println!("🎨 Example 1: Generate Image");
    let response = client
        .generate_image(
            provider,
            CreateImageRequest {
                model: Some(model.clone()),
                n: NonZeroU64::new(1).unwrap(),
                prompt: "A watercolor painting of a crab writing Rust code".to_string(),
                quality: None,
                response_format: CreateImageRequestResponseFormat::Url,
                size: Some(ImageSize::X512x512),
            },
        )
        .await?;
    print_images(&response);
    println!("---\n");

    // Examples 2 and 3 need a local source image; set IMAGE_PATH to run them.
    let Ok(image_path) = env::var("IMAGE_PATH") else {
        println!("IMAGE_PATH not set - skipping edit and variation examples.");
        println!("Set IMAGE_PATH to a png/webp/jpg file to run them.");
        return Ok(());
    };
    let image = std::fs::read(&image_path).map_err(|e| {
        GatewayError::Other(Box::new(std::io::Error::other(format!(
            "failed to read IMAGE_PATH '{image_path}': {e}"
        ))))
    })?;

    // Example 2: Image edit
    println!("✏️ Example 2: Edit Image");
    let response = client
        .create_image_edit(
            Some(provider),
            CreateImageEditRequest {
                image: image.clone(),
                prompt: "Add a rustacean sticker to the laptop".to_string(),
                model: Some(model.clone()),
                ..Default::default()
            },
        )
        .await?;
    print_images(&response);
    println!("---\n");

    // Example 3: Image variation
    println!("🔀 Example 3: Image Variation");
    let response = client
        .create_image_variation(
            Some(provider),
            CreateImageVariationRequest {
                image,
                model: Some(model),
                n: Some(2),
                ..Default::default()
            },
        )
        .await?;
    print_images(&response);

    Ok(())
}
