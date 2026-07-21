use std::env;
use std::io::Write;
use std::process::ExitCode;

use futures_util::{StreamExt, pin_mut};
use inference_gateway_sdk::{
    CreateMessagesRequest, GatewayError, InferenceGatewayAPI, InferenceGatewayClient,
    MessagesMessage, MessagesMessageContent, MessagesMessageRole, MessagesResponseContentBlock,
    MessagesStreamEvent, MessagesStreamEventType, Provider,
};

const DEFAULT_PROVIDER: &str = "anthropic";
const DEFAULT_MODEL: &str = "claude-sonnet-5";

fn request(model: &str, prompt: &str) -> CreateMessagesRequest {
    CreateMessagesRequest {
        max_tokens: 300,
        messages: vec![MessagesMessage {
            role: MessagesMessageRole::User,
            content: MessagesMessageContent::String(prompt.to_string()),
        }],
        metadata: None,
        model: model.to_string(),
        stop_sequences: Vec::new(),
        stream: false,
        system: None,
        temperature: None,
        thinking: None,
        tool_choice: None,
        tools: Vec::new(),
        top_k: None,
        top_p: None,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error in messages examples: {e}");
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

    // Example 1: Simple message creation
    println!("📨 Example 1: Create Message");
    let response = client
        .create_message(
            Some(provider),
            request(&model, "Tell me a fun fact about Rust."),
        )
        .await?;

    for block in &response.content {
        if let MessagesResponseContentBlock::TextBlock(text) = block {
            println!("Response: {}", text.text);
        }
    }
    println!(
        "Usage: input={} output={}",
        response.usage.input_tokens, response.usage.output_tokens
    );
    println!("---\n");

    // Example 2: Streaming message creation
    println!("🌊 Example 2: Streaming Message");
    println!("Assistant: ");

    let stream = client.create_message_stream(
        Some(provider),
        request(&model, "Write a haiku about type safety."),
    );
    pin_mut!(stream);

    let mut stdout = std::io::stdout();
    while let Some(event) = stream.next().await {
        let event = event?;
        let event: MessagesStreamEvent =
            serde_json::from_str(&event.data).map_err(GatewayError::DeserializationError)?;

        match event.type_ {
            MessagesStreamEventType::ContentBlockDelta => {
                if let Some(text) = event.delta.as_ref().and_then(|d| d.text.as_deref()) {
                    print!("{text}");
                    stdout.flush().ok();
                }
            }
            MessagesStreamEventType::MessageDelta => {
                if let Some(usage) = event.usage.as_ref() {
                    println!("\n\n[Usage: {} output tokens]", usage.output_tokens);
                }
            }
            MessagesStreamEventType::MessageStop => {
                println!("[Stream completed]");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
