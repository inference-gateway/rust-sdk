//! Inference Gateway SDK for Rust
//!
//! This crate provides a Rust client for the Inference Gateway API, allowing interaction
//! with various LLM providers through a unified interface.

use core::fmt;
use std::future::Future;

use futures_util::{Stream, StreamExt};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use serde_json::Value;
use thiserror::Error;

/// Stream of Server-Sent Events (SSE) from the Inference Gateway API
#[derive(Debug, Serialize, Deserialize)]
pub struct SSEvents {
    pub data: String,
    pub event: Option<String>,
    pub retry: Option<u64>,
}

/// Custom error types for the Inference Gateway SDK
#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Stream error: {0}")]
    StreamError(reqwest::Error),

    #[error("Decoding error: {0}")]
    DecodingError(std::string::FromUtf8Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    DeserializationError(serde_json::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

/// Common model information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    /// The model identifier
    pub id: String,
    /// The object type, usually "model"
    pub object: Option<String>,
    /// The Unix timestamp (in seconds) of when the model was created
    pub created: Option<i64>,
    /// The organization that owns the model
    pub owned_by: Option<String>,
    /// The provider that serves the model
    pub served_by: Option<String>,
}

/// Response structure for listing models
#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// The provider identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// Response object type, usually "list"
    pub object: String,
    /// List of available models
    pub data: Vec<Model>,
}

/// Supported LLM providers
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[serde(alias = "Ollama", alias = "OLLAMA")]
    Ollama,
    #[serde(alias = "Groq", alias = "GROQ")]
    Groq,
    #[serde(alias = "OpenAI", alias = "OPENAI")]
    OpenAI,
    #[serde(alias = "Cloudflare", alias = "CLOUDFLARE")]
    Cloudflare,
    #[serde(alias = "Cohere", alias = "COHERE")]
    Cohere,
    #[serde(alias = "Anthropic", alias = "ANTHROPIC")]
    Anthropic,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Provider::Ollama => write!(f, "ollama"),
            Provider::Groq => write!(f, "groq"),
            Provider::OpenAI => write!(f, "openai"),
            Provider::Cloudflare => write!(f, "cloudflare"),
            Provider::Cohere => write!(f, "cohere"),
            Provider::Anthropic => write!(f, "anthropic"),
        }
    }
}

impl TryFrom<&str> for Provider {
    type Error = GatewayError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "ollama" => Ok(Self::Ollama),
            "groq" => Ok(Self::Groq),
            "openai" => Ok(Self::OpenAI),
            "cloudflare" => Ok(Self::Cloudflare),
            "cohere" => Ok(Self::Cohere),
            "anthropic" => Ok(Self::Anthropic),
            _ => Err(GatewayError::BadRequest(format!("Unknown provider: {}", s))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    #[default]
    User,
    Assistant,
    Tool,
}

impl fmt::Display for MessageRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageRole::System => write!(f, "system"),
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
            MessageRole::Tool => write!(f, "tool"),
        }
    }
}

/// A message in a conversation with an LLM
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Message {
    /// Role of the message sender ("system", "user", "assistant" or "tool")
    pub role: MessageRole,
    /// Content of the message
    pub content: String,
    /// The tools an LLM wants to invoke
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
    /// Unique identifier of the tool call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Reasoning behind the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

/// A tool call in a message response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessageToolCall {
    /// Unique identifier of the tool call
    pub id: String,
    /// Type of the tool being called
    #[serde(rename = "type")]
    pub r#type: ChatCompletionToolType,
    /// Function that was called
    pub function: ChatCompletionMessageToolCallFunction,
}

/// Type of tool that can be called
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ChatCompletionToolType {
    /// Function tool type
    #[serde(rename = "function")]
    Function,
}

/// Function details in a tool call
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletionMessageToolCallFunction {
    /// Name of the function to call
    pub name: String,
    /// Arguments to the function in JSON string format
    pub arguments: String,
}

// Add this helper method to make argument access more convenient
impl ChatCompletionMessageToolCallFunction {
    pub fn parse_arguments(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }
}

/// Tool function to call
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionObject {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

/// Type of tool that can be used by the model
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Function,
}

/// Tool to use for the LLM toolbox
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub r#type: ToolType,
    pub function: FunctionObject,
}

/// Request payload for generating content
#[derive(Debug, Serialize)]
struct CreateChatCompletionRequest {
    /// Name of the model
    model: String,
    /// Conversation history and prompt
    messages: Vec<Message>,
    /// Enable streaming of responses
    stream: bool,
    /// Optional tools to use for generation
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
}

/// Function details in a tool call response
#[derive(Debug, Deserialize, Clone)]
pub struct ToolFunctionResponse {
    /// Name of the function that the LLM wants to call
    pub name: String,
    /// Description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The arguments that the LLM wants to pass to the function
    pub arguments: Value,
}

/// A tool call in the response
#[derive(Debug, Deserialize, Clone)]
pub struct ToolCallResponse {
    /// Unique identifier of the tool call
    pub id: String,
    /// Type of tool that was called
    #[serde(rename = "type")]
    pub r#type: ToolType,
    /// Function that the LLM wants to call
    pub function: ToolFunctionResponse,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionChoice {
    pub finish_reason: String,
    pub message: Message,
    pub index: i32,
}

/// The response from generating content
#[derive(Debug, Deserialize, Clone)]
pub struct CreateChatCompletionResponse {
    pub id: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub created: i64,
    pub model: String,
    pub object: String,
}

/// The response from streaming content generation
#[derive(Debug, Deserialize, Clone)]
pub struct CreateChatCompletionStreamResponse {
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: String,
    /// A list of chat completion choices. Can contain more than one element if `n` is greater than 1.
    pub choices: Vec<ChatCompletionStreamChoice>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: i64,
    /// The model used to generate the completion.
    pub model: String,
    /// This fingerprint represents the backend configuration that the model runs with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// The object type, which is always "chat.completion.chunk".
    pub object: String,
    /// Usage statistics for the completion request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,
}

/// Choice in a streaming completion response
#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionStreamChoice {
    /// The delta content for this streaming chunk
    pub delta: ChatCompletionStreamDelta,
    /// Index of the choice in the choices array
    pub index: i32,
    /// The reason the model stopped generating tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Delta content for streaming responses
#[derive(Debug, Deserialize, Clone)]
pub struct ChatCompletionStreamDelta {
    /// Role of the message sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageRole>,
    /// Content of the message delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool calls for this delta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallResponse>>,
}

/// Usage statistics for the completion request
#[derive(Debug, Deserialize, Clone)]
pub struct CompletionUsage {
    /// Number of tokens in the generated completion
    pub completion_tokens: i64,
    /// Number of tokens in the prompt
    pub prompt_tokens: i64,
    /// Total number of tokens used in the request (prompt + completion)
    pub total_tokens: i64,
}

/// Client for interacting with the Inference Gateway API
pub struct InferenceGatewayClient {
    base_url: String,
    client: Client,
    token: Option<String>,
    tools: Option<Vec<Tool>>,
    max_tokens: Option<i32>,
}

/// Implement Debug for InferenceGatewayClient
impl std::fmt::Debug for InferenceGatewayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InferenceGatewayClient")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_ref().map(|_| "*****"))
            .finish()
    }
}

/// Core API interface for the Inference Gateway
pub trait InferenceGatewayAPI {
    /// Lists available models from all providers
    ///
    /// # Errors
    /// - Returns [`GatewayError::Unauthorized`] if authentication fails
    /// - Returns [`GatewayError::BadRequest`] if the request is malformed
    /// - Returns [`GatewayError::InternalError`] if the server has an error
    /// - Returns [`GatewayError::Other`] for other errors
    ///
    /// # Returns
    /// A list of models available from all providers
    fn list_models(&self) -> impl Future<Output = Result<ListModelsResponse, GatewayError>> + Send;

    /// Lists available models by a specific provider
    ///
    /// # Arguments
    /// * `provider` - The LLM provider to list models for
    ///
    /// # Errors
    /// - Returns [`GatewayError::Unauthorized`] if authentication fails
    /// - Returns [`GatewayError::BadRequest`] if the request is malformed
    /// - Returns [`GatewayError::InternalError`] if the server has an error
    /// - Returns [`GatewayError::Other`] for other errors
    ///
    /// # Returns
    /// A list of models available from the specified provider
    fn list_models_by_provider(
        &self,
        provider: Provider,
    ) -> impl Future<Output = Result<ListModelsResponse, GatewayError>> + Send;

    /// Generates content using a specified model
    ///
    /// # Arguments
    /// * `provider` - The LLM provider to use
    /// * `model` - Name of the model
    /// * `messages` - Conversation history and prompt
    /// * `tools` - Optional tools to use for generation
    ///
    /// # Errors
    /// - Returns [`GatewayError::Unauthorized`] if authentication fails
    /// - Returns [`GatewayError::BadRequest`] if the request is malformed
    /// - Returns [`GatewayError::InternalError`] if the server has an error
    /// - Returns [`GatewayError::Other`] for other errors
    ///
    /// # Returns
    /// The generated response
    fn generate_content(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Future<Output = Result<CreateChatCompletionResponse, GatewayError>> + Send;

    /// Stream content generation directly using the backend SSE stream.
    ///
    /// # Arguments
    /// * `provider` - The LLM provider to use
    /// * `model` - Name of the model
    /// * `messages` - Conversation history and prompt
    ///
    /// # Returns
    /// A stream of Server-Sent Events (SSE) from the Inference Gateway API
    fn generate_content_stream(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send;

    /// Checks if the API is available
    fn health_check(&self) -> impl Future<Output = Result<bool, GatewayError>> + Send;
}

impl InferenceGatewayClient {
    /// Creates a new client instance
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the Inference Gateway API
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
            token: None,
            tools: None,
            max_tokens: None,
        }
    }

    /// Creates a new client instance with default configuration
    /// pointing to http://localhost:8080/v1
    pub fn new_default() -> Self {
        let base_url = std::env::var("INFERENCE_GATEWAY_URL")
            .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

        Self {
            base_url,
            client: Client::new(),
            token: None,
            tools: None,
            max_tokens: None,
        }
    }

    /// Returns the base URL this client is configured to use
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Provides tools to use for generation
    ///
    /// # Arguments
    /// * `tools` - List of tools to use for generation
    ///
    /// # Returns
    /// Self with the tools set
    pub fn with_tools(mut self, tools: Option<Vec<Tool>>) -> Self {
        self.tools = tools;
        self
    }

    /// Sets an authentication token for the client
    ///
    /// # Arguments
    /// * `token` - JWT token for authentication
    ///
    /// # Returns
    /// Self with the token set
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets the maximum number of tokens to generate
    ///
    /// # Arguments
    /// * `max_tokens` - Maximum number of tokens to generate
    ///
    /// # Returns
    /// Self with the maximum tokens set
    pub fn with_max_tokens(mut self, max_tokens: Option<i32>) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

impl InferenceGatewayAPI for InferenceGatewayClient {
    async fn list_models(&self) -> Result<ListModelsResponse, GatewayError> {
        let url = format!("{}/models", self.base_url);
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;
        match response.status() {
            StatusCode::OK => {
                let json_response: ListModelsResponse = response.json().await?;
                Ok(json_response)
            }
            StatusCode::UNAUTHORIZED => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::Unauthorized(error.error))
            }
            StatusCode::BAD_REQUEST => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::BadRequest(error.error))
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::InternalError(error.error))
            }
            _ => Err(GatewayError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unexpected status code: {}", response.status()),
            )))),
        }
    }

    async fn list_models_by_provider(
        &self,
        provider: Provider,
    ) -> Result<ListModelsResponse, GatewayError> {
        let url = format!("{}/list/models?provider={}", self.base_url, provider);
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = self.client.get(&url).bearer_auth(token);
        }

        let response = request.send().await?;
        match response.status() {
            StatusCode::OK => {
                let json_response: ListModelsResponse = response.json().await?;
                Ok(json_response)
            }
            StatusCode::UNAUTHORIZED => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::Unauthorized(error.error))
            }
            StatusCode::BAD_REQUEST => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::BadRequest(error.error))
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::InternalError(error.error))
            }
            _ => Err(GatewayError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unexpected status code: {}", response.status()),
            )))),
        }
    }

    async fn generate_content(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> Result<CreateChatCompletionResponse, GatewayError> {
        let url = format!("{}/chat/completions?provider={}", self.base_url, provider);
        let mut request = self.client.post(&url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let request_payload = CreateChatCompletionRequest {
            model: model.to_string(),
            messages,
            stream: false,
            tools: self.tools.clone(),
            max_tokens: self.max_tokens,
        };

        let response = request.json(&request_payload).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::BAD_REQUEST => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::BadRequest(error.error))
            }
            StatusCode::UNAUTHORIZED => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::Unauthorized(error.error))
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let error: ErrorResponse = response.json().await?;
                Err(GatewayError::InternalError(error.error))
            }
            status => Err(GatewayError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unexpected status code: {}", status),
            )))),
        }
    }

    /// Stream content generation directly using the backend SSE stream.
    fn generate_content_stream(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send {
        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let url = format!(
            "{}/chat/completions?provider={}",
            base_url,
            provider.to_string().to_lowercase()
        );

        let request = CreateChatCompletionRequest {
            model: model.to_string(),
            messages,
            stream: true,
            tools: None,
            max_tokens: None,
        };

        async_stream::try_stream! {
            let response = client.post(&url).json(&request).send().await?;
            let mut stream = response.bytes_stream();
            let mut current_event: Option<String> = None;
            let mut current_data: Option<String> = None;

            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                let chunk_str = String::from_utf8_lossy(&chunk);

                for line in chunk_str.lines() {
                    if line.is_empty() && current_data.is_some() {
                        yield SSEvents {
                            data: current_data.take().unwrap(),
                            event: current_event.take(),
                            retry: None, // TODO - implement this, for now it's not implemented in the backend
                        };
                        continue;
                    }

                    if let Some(event) = line.strip_prefix("event:") {
                        current_event = Some(event.trim().to_string());
                    } else if let Some(data) = line.strip_prefix("data:") {
                        let processed_data = data.strip_suffix('\n').unwrap_or(data);
                        current_data = Some(processed_data.trim().to_string());
                    }
                }
            }
        }
    }

    async fn health_check(&self) -> Result<bool, GatewayError> {
        let url = format!("{}/health", self.base_url);

        let response = self.client.get(&url).send().await?;
        match response.status() {
            StatusCode::OK => Ok(true),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        CreateChatCompletionRequest, CreateChatCompletionResponse,
        CreateChatCompletionStreamResponse, FunctionObject, GatewayError, InferenceGatewayAPI,
        InferenceGatewayClient, Message, MessageRole, Provider, Tool, ToolType,
    };
    use futures_util::{pin_mut, StreamExt};
    use mockito::{Matcher, Server};
    use serde_json::json;

    #[test]
    fn test_provider_serialization() {
        let providers = vec![
            (Provider::Ollama, "ollama"),
            (Provider::Groq, "groq"),
            (Provider::OpenAI, "openai"),
            (Provider::Cloudflare, "cloudflare"),
            (Provider::Cohere, "cohere"),
            (Provider::Anthropic, "anthropic"),
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
            ("\"groq\"", Provider::Groq),
            ("\"openai\"", Provider::OpenAI),
            ("\"cloudflare\"", Provider::Cloudflare),
            ("\"cohere\"", Provider::Cohere),
            ("\"anthropic\"", Provider::Anthropic),
        ];

        for (json, expected) in test_cases {
            let provider: Provider = serde_json::from_str(json).unwrap();
            assert_eq!(provider, expected);
        }
    }

    #[test]
    fn test_message_serialization_with_tool_call_id() {
        let message_with_tool = Message {
            role: MessageRole::Tool,
            content: "The weather is sunny".to_string(),
            tool_call_id: Some("call_123".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&message_with_tool).unwrap();
        let expected_with_tool =
            r#"{"role":"tool","content":"The weather is sunny","tool_call_id":"call_123"}"#;
        assert_eq!(serialized, expected_with_tool);

        let message_without_tool = Message {
            role: MessageRole::User,
            content: "What's the weather?".to_string(),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&message_without_tool).unwrap();
        let expected_without_tool = r#"{"role":"user","content":"What's the weather?"}"#;
        assert_eq!(serialized, expected_without_tool);

        let deserialized: Message = serde_json::from_str(expected_with_tool).unwrap();
        assert_eq!(deserialized.role, MessageRole::Tool);
        assert_eq!(deserialized.content, "The weather is sunny");
        assert_eq!(deserialized.tool_call_id, Some("call_123".to_string()));

        let deserialized: Message = serde_json::from_str(expected_without_tool).unwrap();
        assert_eq!(deserialized.role, MessageRole::User);
        assert_eq!(deserialized.content, "What's the weather?");
        assert_eq!(deserialized.tool_call_id, None);
    }

    #[test]
    fn test_provider_display() {
        let providers = vec![
            (Provider::Ollama, "ollama"),
            (Provider::Groq, "groq"),
            (Provider::OpenAI, "openai"),
            (Provider::Cloudflare, "cloudflare"),
            (Provider::Cohere, "cohere"),
            (Provider::Anthropic, "anthropic"),
        ];

        for (provider, expected) in providers {
            assert_eq!(provider.to_string(), expected);
        }
    }

    #[test]
    fn test_generate_request_serialization() {
        let request_payload = CreateChatCompletionRequest {
            model: "llama3.2:1b".to_string(),
            messages: vec![
                Message {
                    role: MessageRole::System,
                    content: "You are a helpful assistant.".to_string(),
                    ..Default::default()
                },
                Message {
                    role: MessageRole::User,
                    content: "What is the current weather in Toronto?".to_string(),
                    ..Default::default()
                },
            ],
            stream: false,
            tools: Some(vec![Tool {
                r#type: ToolType::Function,
                function: FunctionObject {
                    name: "get_current_weather".to_string(),
                    description: "Get the current weather of a city".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "city": {
                                "type": "string",
                                "description": "The name of the city"
                            }
                        },
                        "required": ["city"]
                    }),
                },
            }]),
            max_tokens: None,
        };

        let serialized = serde_json::to_string_pretty(&request_payload).unwrap();
        let expected = r#"{
      "model": "llama3.2:1b",
      "messages": [
        {
          "role": "system",
          "content": "You are a helpful assistant."
        },
        {
          "role": "user",
          "content": "What is the current weather in Toronto?"
        }
      ],
      "stream": false,
      "tools": [
        {
          "type": "function",
          "function": {
            "name": "get_current_weather",
            "description": "Get the current weather of a city",
            "parameters": {
              "type": "object",
              "properties": {
                "city": {
                  "type": "string",
                  "description": "The name of the city"
                }
              },
              "required": ["city"]
            }
          }
        }
      ]
    }"#;

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            serde_json::from_str::<serde_json::Value>(expected).unwrap()
        );
    }

    #[tokio::test]
    async fn test_authentication_header() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let mock_response = r#"{
            "object": "list",
            "data": []
        }"#;

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

        let raw_json_response = r#"{
            "error": "Invalid token"
        }"#;

        let mock = server
            .mock("GET", "/v1/models")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
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
            .mock("GET", "/v1/list/models?provider=ollama")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);
        let response = client.list_models_by_provider(Provider::Ollama).await?;

        assert!(response.provider.is_some());
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
            "model": "mixtral-8x7b",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {
                        "role": "assistant",
                        "content": "Hellloooo"
                    }
                }
            ]
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=ollama")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
            ..Default::default()
        }];
        let response = client
            .generate_content(Provider::Ollama, "llama2", messages)
            .await?;

        assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
        assert_eq!(response.choices[0].message.content, "Hellloooo");
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
            "model": "mixtral-8x7b",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {
                        "role": "assistant",
                        "content": "Hello"
                    }
                }
            ]
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);

        let direct_parse: Result<CreateChatCompletionResponse, _> = serde_json::from_str(raw_json);
        assert!(
            direct_parse.is_ok(),
            "Direct JSON parse failed: {:?}",
            direct_parse.err()
        );

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
            ..Default::default()
        }];

        let response = client
            .generate_content(Provider::Groq, "mixtral-8x7b", messages)
            .await?;

        assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
        assert_eq!(response.choices[0].message.content, "Hello");

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content_error_response() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let raw_json_response = r#"{
            "error":"Invalid request"
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);
        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
            ..Default::default()
        }];
        let error = client
            .generate_content(Provider::Groq, "mixtral-8x7b", messages)
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
        let mut server: mockito::ServerGuard = Server::new_async().await;

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
    async fn test_generate_content_case_insensitive() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let raw_json = r#"{
            "id": "chatcmpl-456",
            "object": "chat.completion",
            "created": 1630000001,
            "model": "mixtral-8x7b",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {
                        "role": "assistant",
                        "content": "Hello"
                    }
                }
            ]
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
            ..Default::default()
        }];

        let response = client
            .generate_content(Provider::Groq, "mixtral-8x7b", messages)
            .await?;

        assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
        assert_eq!(response.choices[0].message.content, "Hello");
        assert_eq!(response.model, "mixtral-8x7b");
        assert_eq!(response.object, "chat.completion");
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content_stream() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_chunked_body(move |writer| -> std::io::Result<()> {
                let events = vec![
                    format!("data: {}\n\n", r#"{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268190,"model":"mixtral-8x7b","system_fingerprint":"fp_","choices":[{"index":0,"delta":{"role":"assistant","content":"Hello"},"finish_reason":null}]}"#),
                    format!("data: {}\n\n", r#"{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268191,"model":"mixtral-8x7b","system_fingerprint":"fp_","choices":[{"index":0,"delta":{"role":"assistant","content":" World"},"finish_reason":null}]}"#),
                    format!("data: {}\n\n", r#"{"id":"chatcmpl-123","object":"chat.completion.chunk","created":1694268192,"model":"mixtral-8x7b","system_fingerprint":"fp_","choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"usage":{"prompt_tokens":17,"completion_tokens":40,"total_tokens":57}}"#),
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

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Test message".to_string(),
            ..Default::default()
        }];

        let stream = client.generate_content_stream(Provider::Groq, "mixtral-8x7b", messages);
        pin_mut!(stream);
        while let Some(result) = stream.next().await {
            let result = result?;
            let generate_response: CreateChatCompletionStreamResponse =
                serde_json::from_str(&result.data)
                    .expect("Failed to parse CreateChatCompletionResponse");

            if generate_response.choices[0].finish_reason.is_some() {
                assert_eq!(
                    generate_response.choices[0].finish_reason.as_ref().unwrap(),
                    "stop"
                );
                break;
            }

            if let Some(content) = &generate_response.choices[0].delta.content {
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
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_chunked_body(move |writer| -> std::io::Result<()> {
                let events = vec![format!(
                    "event: {}\ndata: {}\nretry: {}\n\n",
                    r#"error"#, r#"{"error":"Invalid request"}"#, r#"1000"#,
                )];
                for event in events {
                    writer.write_all(event.as_bytes())?;
                }
                Ok(())
            })
            .expect_at_least(1)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Test message".to_string(),
            ..Default::default()
        }];

        let stream = client.generate_content_stream(Provider::Groq, "mixtral-8x7b", messages);

        pin_mut!(stream);
        while let Some(result) = stream.next().await {
            let result = result?;
            assert!(result.event.is_some());
            assert_eq!(result.event.unwrap(), "error");
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
            "model": "deepseek-r1-distill-llama-70b",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "tool_calls",
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
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let tools = vec![Tool {
            r#type: ToolType::Function,
            function: FunctionObject {
                name: "get_weather".to_string(),
                description: "Get the weather for a location".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city name"
                        }
                    },
                    "required": ["location"]
                }),
            },
        }];

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url).with_tools(Some(tools));

        let messages = vec![Message {
            role: MessageRole::User,
            content: "What's the weather in London?".to_string(),
            ..Default::default()
        }];

        let response = client
            .generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", messages)
            .await?;

        assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
        assert_eq!(
            response.choices[0].message.content,
            "Let me check the weather for you."
        );

        let tool_calls = response.choices[0].message.tool_calls.as_ref().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "get_weather");

        let params = tool_calls[0]
            .function
            .parse_arguments()
            .expect("Failed to parse function arguments");
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
            "model": "gpt-4",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {
                        "role": "assistant",
                        "content": "Hello!"
                    }
                }
            ]
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=openai")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hi".to_string(),
            ..Default::default()
        }];

        let response = client
            .generate_content(Provider::OpenAI, "gpt-4", messages)
            .await?;

        assert_eq!(response.model, "gpt-4");
        assert_eq!(response.choices[0].message.content, "Hello!");
        assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
        assert!(response.choices[0].message.tool_calls.is_none());

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content_with_tools_payload() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let raw_request_body = r#"{
            "model": "deepseek-r1-distill-llama-70b",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant."
                },
                {
                    "role": "user",
                    "content": "What is the current weather in Toronto?"
                }
            ],
            "stream": false,
            "tools": [
                {
                    "type": "function",
                    "function": {
                        "name": "get_current_weather",
                        "description": "Get the current weather of a city",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "city": {
                                    "type": "string",
                                    "description": "The name of the city"
                                }
                            },
                            "required": ["city"]
                        }
                    }
                }
            ]
        }"#;

        let raw_json_response = r#"{
            "id": "1234",
            "object": "chat.completion",
            "created": 1630000000,
            "model": "deepseek-r1-distill-llama-70b",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {
                        "role": "assistant",
                        "content": "Let me check the weather for you",
                        "tool_calls": [
                            {
                                "id": "1234",
                                "type": "function",
                                "function": {
                                    "name": "get_current_weather",
                                    "arguments": "{\"city\": \"Toronto\"}"
                                }
                            }
                        ]
                    }
                }
            ]
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(200)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::JsonString(raw_request_body.to_string()))
            .with_body(raw_json_response)
            .create();

        let tools = vec![Tool {
            r#type: ToolType::Function,
            function: FunctionObject {
                name: "get_current_weather".to_string(),
                description: "Get the current weather of a city".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "city": {
                            "type": "string",
                            "description": "The name of the city"
                        }
                    },
                    "required": ["city"]
                }),
            },
        }];

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url);

        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a helpful assistant.".to_string(),
                ..Default::default()
            },
            Message {
                role: MessageRole::User,
                content: "What is the current weather in Toronto?".to_string(),
                ..Default::default()
            },
        ];

        let response = client
            .with_tools(Some(tools))
            .generate_content(Provider::Groq, "deepseek-r1-distill-llama-70b", messages)
            .await?;

        assert_eq!(response.choices[0].message.role, MessageRole::Assistant);
        assert_eq!(
            response.choices[0].message.content,
            "Let me check the weather for you"
        );
        assert_eq!(
            response.choices[0]
                .message
                .tool_calls
                .as_ref()
                .unwrap()
                .len(),
            1
        );

        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content_with_max_tokens() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let raw_json_response = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1630000000,
            "model": "mixtral-8x7b",
            "choices": [
                {
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {
                        "role": "assistant",
                        "content": "Here's a poem with 100 tokens..."
                    }
                }
            ]
        }"#;

        let mock = server
            .mock("POST", "/v1/chat/completions?provider=groq")
            .with_status(200)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::JsonString(
                r#"{
                "model": "mixtral-8x7b",
                "messages": [{"role":"user","content":"Write a poem"}],
                "stream": false,
                "max_tokens": 100
            }"#
                .to_string(),
            ))
            .with_body(raw_json_response)
            .create();

        let base_url = format!("{}/v1", server.url());
        let client = InferenceGatewayClient::new(&base_url).with_max_tokens(Some(100));

        let messages = vec![Message {
            role: MessageRole::User,
            content: "Write a poem".to_string(),
            ..Default::default()
        }];

        let response = client
            .generate_content(Provider::Groq, "mixtral-8x7b", messages)
            .await?;

        assert_eq!(
            response.choices[0].message.content,
            "Here's a poem with 100 tokens..."
        );
        assert_eq!(response.model, "mixtral-8x7b");
        assert_eq!(response.created, 1630000000);
        assert_eq!(response.object, "chat.completion");

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

        let default_url = "http://localhost:8080/v1";
        assert_eq!(default_client.base_url(), default_url);

        Ok(())
    }
}
