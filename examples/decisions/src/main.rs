use std::env;
use std::process::ExitCode;

use inference_gateway_sdk::{
    CreateDecisionRequest, CreateDecisionRequestInput, DecisionQuestion, DecisionQuestionType,
    GatewayError, InferenceGatewayAPI, InferenceGatewayClient, Provider,
};

const DEFAULT_PROVIDER: &str = "openai";
const DEFAULT_MODEL: &str = "gpt-6-luna";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error in decisions example: {e}");
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

    let response = client
        .create_decision(
            provider,
            CreateDecisionRequest {
                model: format!("{provider_str}/{model}"),
                input: CreateDecisionRequestInput::String("Buy now!!!".to_string()),
                questions: vec![DecisionQuestion {
                    name: "is_spam".to_string(),
                    type_: DecisionQuestionType::Predicate,
                    instructions: "Is this message unsolicited spam?".to_string(),
                    choices: vec![],
                    levels: vec![],
                }],
                prompt_cache_breakpoint: None,
                prompt_cache_key: None,
                prompt_cache_options: None,
            },
        )
        .await?;

    for answer in &response.answers {
        println!(
            "{}",
            serde_json::to_string_pretty(answer).unwrap_or_default()
        );
    }
    if let Some(usage) = response.usage {
        println!("Usage: input_tokens={}", usage.input_tokens.unwrap_or(0));
    }

    Ok(())
}
