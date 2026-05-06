use std::env;
use std::io::Write;
use std::process::ExitCode;

use futures_util::{StreamExt, pin_mut};
use inference_gateway_sdk::{
    ChatCompletionTool, ChatCompletionToolType, CreateChatCompletionStreamResponse, FinishReason,
    FunctionObject, FunctionParameters, GatewayError, InferenceGatewayAPI, InferenceGatewayClient,
    Message, MessageContent, MessageRole, Provider,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

const DEFAULT_PROVIDER: &str = "deepseek";
const DEFAULT_MODEL: &str = "deepseek-v4-flash";

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

fn message_content_text(content: &MessageContent) -> String {
    match content {
        MessageContent::String(s) => s.clone(),
        MessageContent::Array(parts) => serde_json::to_string(parts).unwrap_or_default(),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error in chat examples: {e}");
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

    // Example 1: Simple chat completion
    println!("🤖 Example 1: Simple Chat Completion");
    let client = InferenceGatewayClient::new(&base_url).with_max_tokens(Some(150));

    let response = client
        .generate_content(
            provider,
            &model,
            vec![
                message(
                    MessageRole::System,
                    "You are a helpful assistant that provides concise answers.",
                ),
                message(MessageRole::User, "Tell me a fun fact about Rust."),
            ],
        )
        .await?;

    if let Some(choice) = response.choices.first() {
        println!(
            "Response: {}",
            message_content_text(&choice.message.content)
        );
    }
    if let Some(usage) = response.usage.as_ref() {
        println!(
            "Usage: prompt={} completion={} total={}",
            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
        );
    }
    println!("---\n");

    // Example 2: Streaming chat completion
    println!("🌊 Example 2: Streaming Chat Completion");
    println!("Assistant: ");

    let stream_client = InferenceGatewayClient::new(&base_url);
    let stream = stream_client.generate_content_stream(
        provider,
        &model,
        vec![
            message(
                MessageRole::System,
                "You are a creative storyteller. Tell engaging short stories.",
            ),
            message(
                MessageRole::User,
                "Tell me a short story about a robot learning to paint.",
            ),
        ],
    );
    pin_mut!(stream);

    let mut stdout = std::io::stdout();
    while let Some(event) = stream.next().await {
        let event = event?;
        if event.data == "[DONE]" {
            break;
        }
        let chunk: CreateChatCompletionStreamResponse =
            serde_json::from_str(&event.data).map_err(GatewayError::DeserializationError)?;

        if let Some(usage) = chunk.usage.as_ref() {
            println!("\n\n[Usage: {} tokens]", usage.total_tokens);
            break;
        }

        let Some(choice) = chunk.choices.first() else {
            continue;
        };

        if let Some(content) = choice.delta.content.as_deref() {
            print!("{content}");
            stdout.flush().ok();
        }

        if matches!(choice.finish_reason, Some(FinishReason::Stop)) {
            println!("\n[Stream completed]");
            break;
        }
    }

    println!("\n---\n");

    // Example 3: Multi-turn conversation
    println!("💬 Example 3: Multi-turn Conversation");

    let mut conversation = vec![
        message(MessageRole::System, "You are a helpful programming tutor."),
        message(
            MessageRole::User,
            "What is the difference between String and &str in Rust?",
        ),
    ];

    let tutor_client = InferenceGatewayClient::new(&base_url).with_max_tokens(Some(200));
    let first = tutor_client
        .generate_content(provider, &model, conversation.clone())
        .await?;

    let first_text = first
        .choices
        .first()
        .map(|c| message_content_text(&c.message.content))
        .unwrap_or_default();
    println!("Tutor: {first_text}");

    conversation.push(message(MessageRole::Assistant, &first_text));
    conversation.push(message(
        MessageRole::User,
        "Can you give me a simple code example showing the difference?",
    ));

    let tutor_client = InferenceGatewayClient::new(&base_url).with_max_tokens(Some(300));
    let second = tutor_client
        .generate_content(provider, &model, conversation)
        .await?;

    if let Some(choice) = second.choices.first() {
        println!(
            "\nTutor (follow-up): {}",
            message_content_text(&choice.message.content)
        );
    }
    println!("---\n");

    // Example 4: Function calling
    println!("🔧 Example 4: Function Calling");

    let parameters = json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
            },
            "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "The unit of temperature"
            }
        },
        "required": ["location"]
    });

    let tools = vec![ChatCompletionTool {
        type_: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "get_weather".to_string(),
            description: Some("Get the current weather in a given location".to_string()),
            parameters: Some(FunctionParameters(
                parameters.as_object().cloned().unwrap_or_default(),
            )),
            strict: false,
        },
    }];

    let initial_messages = vec![message(
        MessageRole::User,
        "What's the weather like in San Francisco?",
    )];

    let tool_client = InferenceGatewayClient::new(&base_url).with_tools(Some(tools.clone()));
    let tool_response = tool_client
        .generate_content(provider, &model, initial_messages.clone())
        .await?;

    let Some(choice) = tool_response.choices.first() else {
        println!("No response from model");
        return Ok(());
    };

    if !choice.message.tool_calls.is_empty() {
        let mut follow_up = initial_messages.clone();
        let mut assistant_turn = choice.message.clone();
        assistant_turn.role = MessageRole::Assistant;
        follow_up.push(assistant_turn);

        for tool_call in &choice.message.tool_calls {
            println!("\n🔧 Tool called: {}", tool_call.function.name);
            println!("Arguments: {}", tool_call.function.arguments);

            if tool_call.function.name == "get_weather" {
                let args: WeatherArgs = tool_call.function.parse_arguments()?;
                let result = get_weather(args);

                follow_up.push(Message {
                    role: MessageRole::Tool,
                    content: MessageContent::String(result),
                    reasoning: None,
                    reasoning_content: None,
                    tool_call_id: Some(tool_call.id.clone()),
                    tool_calls: Vec::new(),
                });
            }
        }

        let final_client = InferenceGatewayClient::new(&base_url).with_tools(Some(tools));
        let final_response = final_client
            .generate_content(provider, &model, follow_up)
            .await?;

        if let Some(choice) = final_response.choices.first() {
            println!(
                "\nFinal answer: {}",
                message_content_text(&choice.message.content)
            );
        }
    } else {
        println!(
            "\nModel response (no tool call): {}",
            message_content_text(&choice.message.content)
        );
    }

    println!("\n[Function calling completed]");
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherArgs {
    location: String,
    #[serde(default)]
    unit: Option<String>,
}

fn get_weather(args: WeatherArgs) -> String {
    let unit = args.unit.as_deref().unwrap_or("celsius");
    format!(
        "The weather in {} is currently sunny with a temperature of 22° ({})",
        args.location, unit
    )
}
