use crate::{
    ChatCompletionNamedToolChoice, ChatCompletionNamedToolChoiceFunction, ChatCompletionTool,
    ChatCompletionToolChoiceOption, ChatCompletionToolChoiceOptionString, ChatCompletionToolType,
    CreateChatCompletionRequest, CreateChatCompletionRequestReasoningEffort,
    CreateChatCompletionRequestResponseFormat, CreateChatCompletionRequestStop,
    CreateChatCompletionResponse, CreateChatCompletionStreamResponse, FinishReason, FunctionObject,
    FunctionParameters, GatewayError, InferenceGatewayAPI, InferenceGatewayClient, Message,
    MessageContent, MessageRole, Provider, ResponseFormatJsonObject, ResponseFormatJsonObjectType,
    ResponseFormatJsonSchema, ResponseFormatJsonSchemaJsonSchema, ResponseFormatJsonSchemaType,
    ResponseFormatText, ResponseFormatTextType,
};
use futures_util::{StreamExt, pin_mut};
use mockito::{Matcher, Server};
use serde_json::json;

fn user_message(text: &str) -> Message {
    Message {
        role: MessageRole::User,
        content: MessageContent::String(text.to_string()),
        reasoning: None,
        reasoning_content: None,
        tool_call_id: None,
        tool_calls: Vec::new(),
    }
}

fn system_message(text: &str) -> Message {
    Message {
        role: MessageRole::System,
        content: MessageContent::String(text.to_string()),
        reasoning: None,
        reasoning_content: None,
        tool_call_id: None,
        tool_calls: Vec::new(),
    }
}

fn function_params(value: serde_json::Value) -> FunctionParameters {
    let map = value
        .as_object()
        .expect("function_params requires a JSON object")
        .clone();
    FunctionParameters(map)
}

#[test]
fn test_provider_serialization() {
    let providers = vec![
        (Provider::Ollama, "ollama"),
        (Provider::OllamaCloud, "ollama_cloud"),
        (Provider::Groq, "groq"),
        (Provider::Openai, "openai"),
        (Provider::Cloudflare, "cloudflare"),
        (Provider::Cohere, "cohere"),
        (Provider::Anthropic, "anthropic"),
        (Provider::Deepseek, "deepseek"),
        (Provider::Google, "google"),
        (Provider::Mistral, "mistral"),
        (Provider::Minimax, "minimax"),
        (Provider::Moonshot, "moonshot"),
        (Provider::Nvidia, "nvidia"),
    ];

    for (provider, expected) in providers {
        let json = serde_json::to_string(&provider).unwrap();
        assert_eq!(json, format!("\"{}\"", expected));
    }
}

#[test]
fn test_provider_deserialization() {
    let test_cases = vec![
        ("\"ollama\"", Provider::Ollama),
        ("\"ollama_cloud\"", Provider::OllamaCloud),
        ("\"groq\"", Provider::Groq),
        ("\"llamacpp\"", Provider::Llamacpp),
        ("\"openai\"", Provider::Openai),
        ("\"cloudflare\"", Provider::Cloudflare),
        ("\"cohere\"", Provider::Cohere),
        ("\"anthropic\"", Provider::Anthropic),
        ("\"deepseek\"", Provider::Deepseek),
        ("\"google\"", Provider::Google),
        ("\"mistral\"", Provider::Mistral),
        ("\"minimax\"", Provider::Minimax),
        ("\"moonshot\"", Provider::Moonshot),
        ("\"nvidia\"", Provider::Nvidia),
    ];

    for (json, expected) in test_cases {
        let provider: Provider = serde_json::from_str(json).unwrap();
        assert_eq!(provider, expected);
    }
}

#[test]
fn test_provider_moonshot_present() {
    let provider: Provider = serde_json::from_str("\"moonshot\"").unwrap();
    assert_eq!(provider, Provider::Moonshot);
    assert_eq!(provider.to_string(), "moonshot");
    assert_eq!(Provider::try_from("moonshot").unwrap(), Provider::Moonshot);
}

#[test]
fn test_provider_llamacpp_present() {
    let provider: Provider = serde_json::from_str("\"llamacpp\"").unwrap();
    assert_eq!(provider, Provider::Llamacpp);
    assert_eq!(provider.to_string(), "llamacpp");
    assert_eq!(Provider::try_from("llamacpp").unwrap(), Provider::Llamacpp);
}

#[test]
fn test_message_serialization_with_tool_call_id() {
    let mut message_with_tool = system_message("ignored");
    message_with_tool.role = MessageRole::Tool;
    message_with_tool.content = MessageContent::String("The weather is sunny".to_string());
    message_with_tool.tool_call_id = Some("call_123".to_string());

    let serialized = serde_json::to_string(&message_with_tool).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"{"role":"tool","content":"The weather is sunny","tool_call_id":"call_123"}"#,
    )
    .unwrap();
    assert_eq!(actual, expected);

    let message_without_tool = user_message("What's the weather?");
    let serialized = serde_json::to_string(&message_without_tool).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    let expected: serde_json::Value =
        serde_json::from_str(r#"{"role":"user","content":"What's the weather?"}"#).unwrap();
    assert_eq!(actual, expected);

    let deserialized: Message = serde_json::from_str(
        r#"{"role":"tool","content":"The weather is sunny","tool_call_id":"call_123"}"#,
    )
    .unwrap();
    assert_eq!(deserialized.role, MessageRole::Tool);
    match deserialized.content {
        MessageContent::String(ref s) => assert_eq!(s, "The weather is sunny"),
        _ => panic!("expected string content"),
    }
    assert_eq!(deserialized.tool_call_id, Some("call_123".to_string()));
}

#[test]
fn test_provider_display() {
    let providers = vec![
        (Provider::Ollama, "ollama"),
        (Provider::OllamaCloud, "ollama_cloud"),
        (Provider::Groq, "groq"),
        (Provider::Openai, "openai"),
        (Provider::Cloudflare, "cloudflare"),
        (Provider::Cohere, "cohere"),
        (Provider::Anthropic, "anthropic"),
        (Provider::Deepseek, "deepseek"),
        (Provider::Google, "google"),
        (Provider::Mistral, "mistral"),
        (Provider::Minimax, "minimax"),
        (Provider::Moonshot, "moonshot"),
        (Provider::Nvidia, "nvidia"),
    ];

    for (provider, expected) in providers {
        assert_eq!(provider.to_string(), expected);
    }
}

#[test]
fn test_provider_try_from_lowercase() {
    let test_cases = vec![
        "ollama",
        "openai",
        "google",
        "moonshot",
        "ollama_cloud",
        "nvidia",
    ];
    for case in test_cases {
        let provider: Provider = case.try_into().unwrap_or_else(|_| panic!("parse {case}"));
        assert_eq!(provider.to_string(), case);
    }
    assert!(Provider::try_from("Google").is_err());
}

#[test]
fn test_generate_request_serialization() {
    let payload = CreateChatCompletionRequest {
        model: "deepseek-v4-flash".to_string(),
        messages: vec![
            system_message("You are a helpful assistant."),
            user_message("What is the current weather in Toronto?"),
        ],
        stream: false,
        stream_options: None,
        tools: vec![ChatCompletionTool {
            type_: ChatCompletionToolType::Function,
            function: FunctionObject {
                name: "get_current_weather".to_string(),
                description: Some("Get the current weather of a city".to_string()),
                parameters: Some(function_params(json!({
                    "type": "object",
                    "properties": {
                        "city": {
                            "type": "string",
                            "description": "The name of the city"
                        }
                    },
                    "required": ["city"]
                }))),
                strict: false,
            },
        }],
        ..Default::default()
    };

    let serialized: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&payload).unwrap()).unwrap();

    assert_eq!(serialized["model"], "deepseek-v4-flash");
    assert_eq!(serialized["stream"], false);
    assert_eq!(serialized["messages"][0]["role"], "system");
    assert_eq!(serialized["messages"][1]["role"], "user");
    assert_eq!(serialized["tools"][0]["type"], "function");
    assert_eq!(
        serialized["tools"][0]["function"]["name"],
        "get_current_weather"
    );
    // Fields with a schema `default:` are non-`Option` and always serialize.
    assert_eq!(serialized["temperature"], 1.0);
    assert_eq!(serialized["top_p"], 1.0);
    assert_eq!(serialized["n"], 1);
    assert_eq!(serialized["frequency_penalty"], 0.0);
    assert_eq!(serialized["presence_penalty"], 0.0);
    assert_eq!(serialized["logprobs"], false);
    assert_eq!(serialized["parallel_tool_calls"], true);
}

// The `stop`, `tool_choice`, and `response_format` request fields are `oneOf`
// unions in openapi.yaml. typify emits them as `#[serde(untagged)]` enums, so
// every variant must serialize to its bare wire shape and deserialize back to
// the same variant (the untagged discriminator relies on the `type` enums).

#[test]
fn test_stop_oneof_round_trip() {
    let string = CreateChatCompletionRequestStop::String("STOP".to_string());
    let value = serde_json::to_value(&string).unwrap();
    assert_eq!(value, json!("STOP"));
    match serde_json::from_value(value).unwrap() {
        CreateChatCompletionRequestStop::String(s) => assert_eq!(s, "STOP"),
        other => panic!("expected String, got {other:?}"),
    }

    let array = CreateChatCompletionRequestStop::Array(vec!["\n\n".to_string(), "END".to_string()]);
    let value = serde_json::to_value(&array).unwrap();
    assert_eq!(value, json!(["\n\n", "END"]));
    match serde_json::from_value(value).unwrap() {
        CreateChatCompletionRequestStop::Array(v) => assert_eq!(v, ["\n\n", "END"]),
        other => panic!("expected Array, got {other:?}"),
    }
}

#[test]
fn test_tool_choice_oneof_round_trip() {
    let auto = ChatCompletionToolChoiceOption::String(ChatCompletionToolChoiceOptionString::Auto);
    let value = serde_json::to_value(&auto).unwrap();
    assert_eq!(value, json!("auto"));
    match serde_json::from_value(value).unwrap() {
        ChatCompletionToolChoiceOption::String(s) => {
            assert_eq!(s, ChatCompletionToolChoiceOptionString::Auto)
        }
        other => panic!("expected String, got {other:?}"),
    }

    let named = ChatCompletionToolChoiceOption::ChatCompletionNamedToolChoice(
        ChatCompletionNamedToolChoice {
            type_: ChatCompletionToolType::Function,
            function: ChatCompletionNamedToolChoiceFunction {
                name: "get_current_weather".to_string(),
            },
        },
    );
    let value = serde_json::to_value(&named).unwrap();
    assert_eq!(
        value,
        json!({"type": "function", "function": {"name": "get_current_weather"}})
    );
    match serde_json::from_value(value).unwrap() {
        ChatCompletionToolChoiceOption::ChatCompletionNamedToolChoice(c) => {
            assert_eq!(c.function.name, "get_current_weather")
        }
        other => panic!("expected named tool choice, got {other:?}"),
    }
}

#[test]
fn test_response_format_oneof_round_trip() {
    let text = CreateChatCompletionRequestResponseFormat::Text(ResponseFormatText {
        type_: ResponseFormatTextType::Text,
    });
    let value = serde_json::to_value(&text).unwrap();
    assert_eq!(value, json!({"type": "text"}));
    assert!(matches!(
        serde_json::from_value(value).unwrap(),
        CreateChatCompletionRequestResponseFormat::Text(_)
    ));

    let json_object =
        CreateChatCompletionRequestResponseFormat::JsonObject(ResponseFormatJsonObject {
            type_: ResponseFormatJsonObjectType::JsonObject,
        });
    let value = serde_json::to_value(&json_object).unwrap();
    assert_eq!(value, json!({"type": "json_object"}));
    // The `type` enum is what keeps `json_object` from matching the `Text`
    // variant first under `#[serde(untagged)]`.
    assert!(matches!(
        serde_json::from_value(value).unwrap(),
        CreateChatCompletionRequestResponseFormat::JsonObject(_)
    ));

    let json_schema =
        CreateChatCompletionRequestResponseFormat::JsonSchema(ResponseFormatJsonSchema {
            type_: ResponseFormatJsonSchemaType::JsonSchema,
            json_schema: ResponseFormatJsonSchemaJsonSchema {
                name: "weather".to_string(),
                description: None,
                schema: None,
                strict: false,
            },
        });
    let value = serde_json::to_value(&json_schema).unwrap();
    assert_eq!(value["type"], "json_schema");
    assert_eq!(value["json_schema"]["name"], "weather");
    assert!(matches!(
        serde_json::from_value(value).unwrap(),
        CreateChatCompletionRequestResponseFormat::JsonSchema(_)
    ));
}

#[test]
fn test_reasoning_effort_round_trip() {
    let cases = [
        (
            CreateChatCompletionRequestReasoningEffort::Minimal,
            "minimal",
        ),
        (CreateChatCompletionRequestReasoningEffort::Low, "low"),
        (CreateChatCompletionRequestReasoningEffort::Medium, "medium"),
        (CreateChatCompletionRequestReasoningEffort::High, "high"),
    ];
    for (variant, wire) in cases {
        let value = serde_json::to_value(variant).unwrap();
        assert_eq!(value, json!(wire));
        let back: CreateChatCompletionRequestReasoningEffort =
            serde_json::from_value(value).unwrap();
        assert_eq!(back, variant);
    }
}

#[test]
fn test_request_with_new_optional_fields_round_trips() {
    let request = CreateChatCompletionRequest {
        model: "gpt-test".to_string(),
        messages: vec![user_message("hi")],
        stop: Some(CreateChatCompletionRequestStop::Array(vec![
            "STOP".to_string(),
        ])),
        tool_choice: Some(ChatCompletionToolChoiceOption::String(
            ChatCompletionToolChoiceOptionString::Required,
        )),
        response_format: Some(CreateChatCompletionRequestResponseFormat::JsonObject(
            ResponseFormatJsonObject {
                type_: ResponseFormatJsonObjectType::JsonObject,
            },
        )),
        reasoning_effort: Some(CreateChatCompletionRequestReasoningEffort::Low),
        seed: Some(42),
        top_logprobs: Some(5),
        user: Some("user-123".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&request).unwrap();
    let back: CreateChatCompletionRequest = serde_json::from_str(&json).unwrap();

    assert!(matches!(
        back.stop,
        Some(CreateChatCompletionRequestStop::Array(ref v)) if *v == ["STOP"]
    ));
    assert!(matches!(
        back.tool_choice,
        Some(ChatCompletionToolChoiceOption::String(
            ChatCompletionToolChoiceOptionString::Required
        ))
    ));
    assert!(matches!(
        back.response_format,
        Some(CreateChatCompletionRequestResponseFormat::JsonObject(_))
    ));
    assert_eq!(
        back.reasoning_effort,
        Some(CreateChatCompletionRequestReasoningEffort::Low)
    );
    assert_eq!(back.seed, Some(42));
    assert_eq!(back.top_logprobs, Some(5));
    assert_eq!(back.user.as_deref(), Some("user-123"));
}

#[tokio::test]
async fn test_authentication_header() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock_response = r#"{ "object": "list", "data": [] }"#;

    let mock_with_auth = server
        .mock("GET", "/v1/models")
        .match_header("authorization", "Bearer test-token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .expect(1)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url).with_token("test-token");
    client.list_models().await?;
    mock_with_auth.assert();

    let mock_without_auth = server
        .mock("GET", "/v1/models")
        .match_header("authorization", Matcher::Missing)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .expect(1)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    client.list_models().await?;
    mock_without_auth.assert();

    Ok(())
}

#[tokio::test]
async fn test_unauthorized_error() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/models")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{ "error": "Invalid token" }"#)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let error = client.list_models().await.unwrap_err();

    assert!(matches!(error, GatewayError::Unauthorized(_)));
    if let GatewayError::Unauthorized(msg) = error {
        assert_eq!(msg, "Invalid token");
    }
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_list_models() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_response_json = r#"{
        "object": "list",
        "data": [
            {
                "id": "llama2",
                "object": "model",
                "created": 1630000001,
                "owned_by": "ollama",
                "served_by": "ollama"
            }
        ]
    }"#;

    let mock = server
        .mock("GET", "/v1/models")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_response_json)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let response = client.list_models().await?;

    assert!(response.provider.is_none());
    assert_eq!(response.object, "list");
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, "llama2");
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_list_models_by_provider() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_json_response = r#"{
        "provider":"ollama",
        "object":"list",
        "data": [
            {
                "id": "llama2",
                "object": "model",
                "created": 1630000001,
                "owned_by": "ollama",
                "served_by": "ollama"
            }
        ]
    }"#;

    let mock = server
        .mock("GET", "/v1/models?provider=ollama")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_json_response)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let response = client.list_models_by_provider(Provider::Ollama).await?;

    assert_eq!(response.provider, Some(Provider::Ollama));
    assert_eq!(response.data[0].id, "llama2");
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_json_response = r#"{
        "id": "chatcmpl-456",
        "object": "chat.completion",
        "created": 1630000001,
        "model": "deepseek-v4-flash",
        "choices": [
            {
                "index": 0,
                "finish_reason": "stop",
                "logprobs": null,
                "message": {
                    "role": "assistant",
                    "content": "Hellloooo"
                }
            }
        ]
    }"#;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_json_response)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);

    let messages = vec![user_message("Hello")];
    let response = client
        .generate_content(Provider::Deepseek, "deepseek-v4-flash", messages)
        .await?;

    assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
    assert!(matches!(
        response.choices[0].message.content,
        MessageContent::String(ref s) if s == "Hellloooo"
    ));
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content_serialization() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_json = r#"{
        "id": "chatcmpl-456",
        "object": "chat.completion",
        "created": 1630000001,
        "model": "deepseek-v4-flash",
        "choices": [
            {
                "index": 0,
                "finish_reason": "stop",
                "logprobs": null,
                "message": {
                    "role": "assistant",
                    "content": "Hello"
                }
            }
        ]
    }"#;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_json)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);

    let direct_parse: Result<CreateChatCompletionResponse, _> = serde_json::from_str(raw_json);
    assert!(
        direct_parse.is_ok(),
        "direct parse: {:?}",
        direct_parse.err()
    );

    let messages = vec![user_message("Hello")];
    let response = client
        .generate_content(Provider::Deepseek, "deepseek-v4-flash", messages)
        .await?;

    assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
    assert!(matches!(
        response.choices[0].message.content,
        MessageContent::String(ref s) if s == "Hello"
    ));
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content_error_response() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{ "error": "Invalid request" }"#)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let messages = vec![user_message("Hello")];
    let error = client
        .generate_content(Provider::Deepseek, "deepseek-v4-flash", messages)
        .await
        .unwrap_err();

    assert!(matches!(error, GatewayError::BadRequest(_)));
    if let GatewayError::BadRequest(msg) = error {
        assert_eq!(msg, "Invalid request");
    }
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_gateway_errors() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let unauthorized_mock = server
        .mock("GET", "/v1/models")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"Invalid token"}"#)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    match client.list_models().await {
        Err(GatewayError::Unauthorized(msg)) => assert_eq!(msg, "Invalid token"),
        _ => panic!("Expected Unauthorized error"),
    }
    unauthorized_mock.assert();

    let bad_request_mock = server
        .mock("GET", "/v1/models")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"Invalid provider"}"#)
        .create();

    match client.list_models().await {
        Err(GatewayError::BadRequest(msg)) => assert_eq!(msg, "Invalid provider"),
        _ => panic!("Expected BadRequest error"),
    }
    bad_request_mock.assert();

    let internal_error_mock = server
        .mock("GET", "/v1/models")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"Internal server error occurred"}"#)
        .create();

    match client.list_models().await {
        Err(GatewayError::InternalError(msg)) => {
            assert_eq!(msg, "Internal server error occurred")
        }
        _ => panic!("Expected InternalError error"),
    }
    internal_error_mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content_stream() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(200)
        .with_header("content-type", "text/event-stream")
        .with_chunked_body(move |writer| -> std::io::Result<()> {
            let events = vec![
                format!("data: {}\n\n", r#"{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"deepseek-v4-flash","system_fingerprint":"fp_","choices":[{"index":0,"delta":{"role":"assistant","content":"Hello"},"finish_reason":null}]}"#),
                format!("data: {}\n\n", r#"{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268191,"model":"deepseek-v4-flash","system_fingerprint":"fp_","choices":[{"index":0,"delta":{"role":"assistant","content":" World"},"finish_reason":null}]}"#),
                format!("data: {}\n\n", r#"{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268192,"model":"deepseek-v4-flash","system_fingerprint":"fp_","choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"usage":{"prompt_tokens":17,"completion_tokens":40,"total_tokens":57}}"#),
                format!("data: [DONE]\n\n")
            ];
            for event in events {
                writer.write_all(event.as_bytes())?;
            }
            Ok(())
        })
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);

    let messages = vec![user_message("Test message")];

    let stream = client.generate_content_stream(Provider::Deepseek, "deepseek-v4-flash", messages);
    pin_mut!(stream);
    while let Some(result) = stream.next().await {
        let result = result?;
        if result.data == "[DONE]" {
            break;
        }
        let generate_response: CreateChatCompletionStreamResponse =
            serde_json::from_str(&result.data).expect("parse stream chunk");

        if let Some(reason) = &generate_response.choices[0].finish_reason {
            assert_eq!(reason, &FinishReason::Stop);
            break;
        }

        let content = &generate_response.choices[0].delta.content;
        if let Some(content) = content {
            assert!(matches!(content.as_str(), "Hello" | " World"));
        }
        if let Some(role) = &generate_response.choices[0].delta.role {
            assert_eq!(role, &MessageRole::Assistant);
        }
    }

    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content_stream_error() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_chunked_body(move |writer| -> std::io::Result<()> {
            let event = "event: error\ndata: {\"error\":\"Invalid request\"}\nretry: 1000\n\n";
            writer.write_all(event.as_bytes())?;
            Ok(())
        })
        .expect_at_least(1)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);

    let messages = vec![user_message("Test message")];
    let stream = client.generate_content_stream(Provider::Deepseek, "deepseek-v4-flash", messages);

    pin_mut!(stream);
    while let Some(result) = stream.next().await {
        let result = result?;
        assert_eq!(result.event.as_deref(), Some("error"));
        assert!(result.data.contains("Invalid request"));
        assert!(result.retry.is_none());
    }

    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content_with_tools() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_json_response = r#"{
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1630000000,
        "model": "deepseek-v4-flash",
        "choices": [
            {
                "index": 0,
                "finish_reason": "tool_calls",
                "logprobs": null,
                "message": {
                    "role": "assistant",
                    "content": "Let me check the weather for you.",
                    "tool_calls": [
                        {
                            "id": "1234",
                            "type": "function",
                            "function": {
                                "name": "get_weather",
                                "arguments": "{\"location\": \"London\"}"
                            }
                        }
                    ]
                }
            }
        ]
    }"#;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_json_response)
        .create();

    let tools = vec![ChatCompletionTool {
        type_: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "get_weather".to_string(),
            description: Some("Get the weather for a location".to_string()),
            parameters: Some(function_params(json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city name"
                    }
                },
                "required": ["location"]
            }))),
            strict: false,
        },
    }];

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url).with_tools(Some(tools));

    let messages = vec![user_message("What's the weather in London?")];

    let response = client
        .generate_content(Provider::Deepseek, "deepseek-v4-flash", messages)
        .await?;

    assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
    assert!(matches!(
        response.choices[0].message.content,
        MessageContent::String(ref s) if s == "Let me check the weather for you."
    ));

    let tool_calls = &response.choices[0].message.tool_calls;
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].function.name, "get_weather");

    let params: serde_json::Value = tool_calls[0]
        .function
        .parse_arguments()
        .expect("parse function arguments");
    assert_eq!(params["location"].as_str().unwrap(), "London");

    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_generate_content_without_tools() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_json_response = r#"{
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1630000000,
        "model": "deepseek-v4-flash",
        "choices": [
            {
                "index": 0,
                "finish_reason": "stop",
                "logprobs": null,
                "message": {
                    "role": "assistant",
                    "content": "Hello!"
                }
            }
        ]
    }"#;

    let mock = server
        .mock("POST", "/v1/chat/completions?provider=deepseek")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_json_response)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);

    let messages = vec![user_message("Hi")];

    let response = client
        .generate_content(Provider::Deepseek, "deepseek-v4-flash", messages)
        .await?;

    assert_eq!(response.model, "deepseek-v4-flash");
    assert!(matches!(
        response.choices[0].message.content,
        MessageContent::String(ref s) if s == "Hello!"
    ));
    assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
    assert!(response.choices[0].message.tool_calls.is_empty());

    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_health_check() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/health").with_status(200).create();

    let client = InferenceGatewayClient::new(&server.url());
    let is_healthy = client.health_check().await?;

    assert!(is_healthy);
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_health_check_strips_versioned_prefix() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/health").with_status(200).create();
    let v1_unmatched = server
        .mock("GET", "/v1/health")
        .with_status(404)
        .expect(0)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let is_healthy = client.health_check().await?;

    assert!(is_healthy);
    mock.assert();
    v1_unmatched.assert();
    Ok(())
}

#[tokio::test]
async fn test_client_base_url_configuration() -> Result<(), GatewayError> {
    let mut custom_url_server = Server::new_async().await;

    let custom_url_mock = custom_url_server
        .mock("GET", "/health")
        .with_status(200)
        .create();

    let custom_client = InferenceGatewayClient::new(&custom_url_server.url());
    let is_healthy = custom_client.health_check().await?;
    assert!(is_healthy);
    custom_url_mock.assert();

    let default_client = InferenceGatewayClient::new_default();
    assert_eq!(default_client.base_url(), "http://localhost:8080/v1");

    Ok(())
}

#[tokio::test]
async fn test_list_tools() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let raw_response_json = r#"{
        "object": "list",
        "data": [
            {
                "name": "read_file",
                "description": "Read content from a file",
                "server": "http://mcp-filesystem-server:8083/mcp",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["file_path"]
                }
            },
            {
                "name": "write_file",
                "description": "Write content to a file",
                "server": "http://mcp-filesystem-server:8083/mcp"
            }
        ]
    }"#;

    let mock = server
        .mock("GET", "/v1/mcp/tools")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(raw_response_json)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);
    let response = client.list_tools().await?;

    assert_eq!(response.object, "list");
    assert_eq!(response.data.len(), 2);

    assert_eq!(response.data[0].name, "read_file");
    assert_eq!(response.data[0].description, "Read content from a file");
    assert_eq!(
        response.data[0].server,
        "http://mcp-filesystem-server:8083/mcp"
    );
    assert!(!response.data[0].input_schema.is_empty());

    assert_eq!(response.data[1].name, "write_file");
    assert_eq!(response.data[1].description, "Write content to a file");
    assert!(response.data[1].input_schema.is_empty());

    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_list_tools_with_authentication() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/mcp/tools")
        .match_header("authorization", "Bearer test-token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{ "object": "list", "data": [] }"#)
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url).with_token("test-token");
    let response = client.list_tools().await?;

    assert_eq!(response.object, "list");
    assert_eq!(response.data.len(), 0);
    mock.assert();
    Ok(())
}

#[tokio::test]
async fn test_list_tools_mcp_not_exposed() -> Result<(), GatewayError> {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/v1/mcp/tools")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"error":"MCP tools endpoint is not exposed. Set EXPOSE_MCP=true to enable."}"#,
        )
        .create();

    let base_url = format!("{}/v1", server.url());
    let client = InferenceGatewayClient::new(&base_url);

    match client.list_tools().await {
        Err(GatewayError::Forbidden(msg)) => {
            assert_eq!(
                msg,
                "MCP tools endpoint is not exposed. Set EXPOSE_MCP=true to enable."
            );
        }
        _ => panic!("Expected Forbidden error for MCP not exposed"),
    }

    mock.assert();
    Ok(())
}
