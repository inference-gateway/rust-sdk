//! Inference Gateway SDK for Rust
//!
//! This crate provides a Rust client for the Inference Gateway API, allowing interaction
//! with various LLM providers through a unified interface.

use core::fmt;
use std::future::Future;

use futures_util::{Stream, StreamExt};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use thiserror::Error;

/// Stream of Server-Sent Events (SSE) from the Inference Gateway API
#[derive(Debug, Serialize, Deserialize)]
pub struct SSEvents {
    pub data: String,
    pub event: Option<String>,
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

/// Represents a model available through a provider
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// Unique identifier of the model
    pub name: String,
}

/// Collection of models available from a specific provider
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderModels {
    /// The LLM provider
    pub provider: Provider,
    /// List of available models
    pub models: Vec<Model>,
}

/// Supported LLM providers
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[serde(alias = "Ollama", alias = "OLLAMA")]
    Ollama,
    #[serde(alias = "Groq", alias = "GROQ")]
    Groq,
    #[serde(alias = "OpenAI", alias = "OPENAI")]
    OpenAI,
    #[serde(alias = "Google", alias = "GOOGLE")]
    Google,
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
            Provider::Google => write!(f, "google"),
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
            "google" => Ok(Self::Google),
            "cloudflare" => Ok(Self::Cloudflare),
            "cohere" => Ok(Self::Cohere),
            "anthropic" => Ok(Self::Anthropic),
            _ => Err(GatewayError::BadRequest(format!("Unknown provider: {}", s))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl fmt::Display for MessageRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageRole::System => write!(f, "system"),
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "assistant"),
        }
    }
}

/// A message in a conversation with an LLM
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender ("system", "user" or "assistant")
    pub role: MessageRole,
    /// Content of the message
    pub content: String,
}

/// Request payload for generating content
#[derive(Debug, Serialize)]
struct GenerateRequest {
    /// Name of the model
    model: String,
    /// Conversation history and prompt
    messages: Vec<Message>,
    /// Enable Server-Sent Events (SSE) streaming
    ssevents: bool,
    /// Enable streaming of responses
    stream: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GenerateResponse {
    /// Provider that generated the response
    pub provider: Provider,
    /// Content of the response
    pub response: ResponseContent,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseContent {
    /// Role of the responder (typically "assistant")
    pub role: MessageRole,
    /// Model that generated the response
    pub model: String,
    /// Generated content
    pub content: String,
}

/// Client for interacting with the Inference Gateway API
pub struct InferenceGatewayClient {
    base_url: String,
    client: Client,
    token: Option<String>,
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
    fn list_models(&self)
        -> impl Future<Output = Result<Vec<ProviderModels>, GatewayError>> + Send;

    /// Lists available models by a specific provider
    ///
    /// # Arguments
    /// * `provider` - The LLM provider to list models for
    ///
    /// # Errors
    /// - Returns [`GatewayError::Unauthorized`] if authentication fails
    /// - Returns [`GatewayError::BadRequest`] if the request is malformed
    /// - Returns [`GatewayError::InternalError`] if the server has an error
    fn list_models_by_provider(
        &self,
        provider: Provider,
    ) -> impl Future<Output = Result<ProviderModels, GatewayError>> + Send;

    /// Generates content using a specified model
    ///
    /// # Arguments
    /// * `provider` - The LLM provider to use
    /// * `model` - Name of the model
    /// * `messages` - Conversation history and prompt
    fn generate_content(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Future<Output = Result<GenerateResponse, GatewayError>> + Send;

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
        }
    }

    /// Sets an authentication token for the client
    ///
    /// # Arguments
    /// * `token` - JWT token for authentication
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }
}

impl InferenceGatewayAPI for InferenceGatewayClient {
    async fn list_models(&self) -> Result<Vec<ProviderModels>, GatewayError> {
        let url = format!("{}/llms", self.base_url);
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
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
    ) -> Result<ProviderModels, GatewayError> {
        let url = format!("{}/llms/{}", self.base_url, provider);
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = self.client.get(&url).bearer_auth(token);
        }

        let response = request.send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
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
    ) -> Result<GenerateResponse, GatewayError> {
        let url = format!("{}/llms/{}/generate", self.base_url, provider);
        let mut request = self.client.post(&url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let request_payload = GenerateRequest {
            model: model.to_string(),
            messages,
            stream: false,
            ssevents: false,
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
            "{}/llms/{}/generate",
            base_url,
            provider.to_string().to_lowercase()
        );

        let request = GenerateRequest {
            model: model.to_string(),
            messages,
            stream: true,
            ssevents: true,
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
                        // Empty line marks end of event, yield if we have data
                        yield SSEvents {
                            data: current_data.take().unwrap(),
                            event: current_event.take(),
                        };
                        continue;
                    }

                    if let Some(event) = line.strip_prefix("event:") {
                        current_event = Some(event.trim().to_string());
                    } else if let Some(data) = line.strip_prefix("data:") {
                        current_data = Some(data.trim().to_string());
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
    use super::*;
    use futures_util::pin_mut;
    use mockito::{Matcher, Server};

    #[test]
    fn test_provider_serialization() {
        let providers = vec![
            (Provider::Ollama, "ollama"),
            (Provider::Groq, "groq"),
            (Provider::OpenAI, "openai"),
            (Provider::Google, "google"),
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
            ("\"google\"", Provider::Google),
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
    fn test_provider_display() {
        let providers = vec![
            (Provider::Ollama, "ollama"),
            (Provider::Groq, "groq"),
            (Provider::OpenAI, "openai"),
            (Provider::Google, "google"),
            (Provider::Cloudflare, "cloudflare"),
            (Provider::Cohere, "cohere"),
            (Provider::Anthropic, "anthropic"),
        ];

        for (provider, expected) in providers {
            assert_eq!(provider.to_string(), expected);
        }
    }

    #[tokio::test]
    async fn test_authentication_header() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        // Test with token
        let mock_with_auth = server
            .mock("GET", "/llms")
            .match_header("authorization", "Bearer test-token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[]")
            .expect(1)
            .create();

        let client = InferenceGatewayClient::new(&server.url()).with_token("test-token");
        client.list_models().await?;
        mock_with_auth.assert();

        // Test without token
        let mock_without_auth = server
            .mock("GET", "/llms")
            .match_header("authorization", Matcher::Missing)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[]")
            .expect(1)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
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
            .mock("GET", "/llms")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
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

        let raw_response_json = r#"[
            {
                "provider": "ollama",
                "models": [
                    {"name": "llama2"}
                ]
            }
        ]"#;

        let mock = server
            .mock("GET", "/llms")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_response_json)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let models = client.list_models().await?;

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].models[0].name, "llama2");
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_list_models_by_provider() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let raw_json_response = r#"{
            "provider":"ollama",
            "models": [{
                "name": "llama2"
            }]
        }"#;

        let mock = server
            .mock("GET", "/llms/ollama")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let models = client.list_models_by_provider(Provider::Ollama).await?;

        assert_eq!(models.provider, Provider::Ollama);
        assert_eq!(models.models[0].name, "llama2");
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let raw_json_response = r#"{
            "provider":"ollama",
            "response":{
                "role":"assistant",
                "model":"llama2",
                "content":"Hellloooo"
            }
        }"#;

        let mock = server
            .mock("POST", "/llms/ollama/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
        }];
        let response = client
            .generate_content(Provider::Ollama, "llama2", messages)
            .await?;

        assert_eq!(response.provider, Provider::Ollama);
        assert_eq!(response.response.role, MessageRole::Assistant);
        assert_eq!(response.response.model, "llama2");
        assert_eq!(response.response.content, "Hellloooo");
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content_serialization() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        // Raw JSON response from API for debugging
        let raw_json = r#"{
        "provider": "groq",
        "response": {
            "role": "assistant",
            "model": "mixtral-8x7b",
            "content": "Hello"
        }
        }"#;

        // Create mock with exact JSON structure
        let mock = server
            .mock("POST", "/llms/groq/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json)
            .create();

        let client = InferenceGatewayClient::new(&server.url());

        // Test direct JSON deserialization first
        let direct_parse: Result<GenerateResponse, _> = serde_json::from_str(raw_json);
        assert!(
            direct_parse.is_ok(),
            "Direct JSON parse failed: {:?}",
            direct_parse.err()
        );

        // Test through client
        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
        }];

        let response = client
            .generate_content(Provider::Groq, "mixtral-8x7b", messages)
            .await?;

        // Verify structure matches
        assert_eq!(response.provider, Provider::Groq);
        assert_eq!(response.response.role, MessageRole::Assistant);
        assert_eq!(response.response.model, "mixtral-8x7b");
        assert_eq!(response.response.content, "Hello");

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
            .mock("POST", "/llms/groq/generate")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(raw_json_response)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
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

        // Test unauthorized error
        let unauthorized_mock = server
            .mock("GET", "/llms")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"Invalid token"}"#)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        match client.list_models().await {
            Err(GatewayError::Unauthorized(msg)) => assert_eq!(msg, "Invalid token"),
            _ => panic!("Expected Unauthorized error"),
        }
        unauthorized_mock.assert();

        // Test bad request error
        let bad_request_mock = server
            .mock("GET", "/llms")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"Invalid provider"}"#)
            .create();

        match client.list_models().await {
            Err(GatewayError::BadRequest(msg)) => assert_eq!(msg, "Invalid provider"),
            _ => panic!("Expected BadRequest error"),
        }
        bad_request_mock.assert();

        // Test internal server error
        let internal_error_mock = server
            .mock("GET", "/llms")
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
            "provider": "Groq",
            "response": {
                "role": "assistant",
                "model": "mixtral-8x7b",
                "content": "Hello"
            }
        }"#;

        let mock = server
            .mock("POST", "/llms/groq/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(raw_json)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let messages = vec![Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
        }];

        let response = client
            .generate_content(Provider::Groq, "mixtral-8x7b", messages)
            .await?;

        assert_eq!(response.provider, Provider::Groq);
        assert_eq!(response.response.content, "Hello");
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_content_stream() -> Result<(), GatewayError> {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/llms/groq/generate")
            .with_status(200)
            .with_header("content-type", "text/event-stream")
            .with_chunked_body(move |writer| -> std::io::Result<()> {
                let events = vec![
                    format!("event: {}\ndata: {}\n\n", r#"message"#, r#"{"provider":"groq","response":{"role":"assistant","model":"mixtral-8x7b","content":"Hello"}}"#),
                    format!("event: {}\ndata: {}\n\n", r#"message"#, r#"{"provider":"groq","response":{"role":"assistant","model":"mixtral-8x7b","content":"World"}}"#),
                ];
                for event in events {
                    writer.write_all(event.as_bytes())?;
                }
                Ok(())
            })
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let messages = vec![Message {
            role: MessageRole::User,
            content: "Test message".to_string(),
        }];

        let stream = client.generate_content_stream(Provider::Groq, "mixtral-8x7b", messages);

        pin_mut!(stream);
        while let Some(result) = stream.next().await {
            let result = result?;
            let generate_response: GenerateResponse =
                serde_json::from_str(&result.data).expect("Failed to parse GenerateResponse");

            assert_eq!(result.event, Some("message".to_string()));
            assert_eq!(generate_response.provider, Provider::Groq);
            assert!(matches!(
                generate_response.response.role,
                MessageRole::Assistant
            ));
            assert!(matches!(
                generate_response.response.model.as_str(),
                "mixtral-8x7b"
            ));
            assert!(matches!(
                generate_response.response.content.as_str(),
                "Hello" | "World"
            ));
        }

        mock.assert();
        Ok(())
    }

    // #[tokio::test]
    // async fn test_generate_content_stream_error() -> Result<(), GatewayError> {
    //     let mut server = Server::new_async().await;

    //     let mock = server
    //         .mock("POST", "/llms/groq/generate")
    //         .with_status(400)
    //         .with_header("content-type", "application/json")
    //         .with_chunked_body(move |writer| -> std::io::Result<()> {
    //             let events = vec!["event: error".to_string(), "retry: 1000".to_string()];
    //             for event in events {
    //                 writer.write_all(event.as_bytes())?;
    //             }
    //             Ok(())
    //         })
    //         .expect_at_least(1)
    //         .create();

    //     let client = InferenceGatewayClient::new(&server.url());
    //     let messages = vec![Message {
    //         role: MessageRole::User,
    //         content: "Test message".to_string(),
    //     }];

    //     let stream =
    //         client.generate_content_stream(Provider::Groq, "mixtral-8x7b", messages, false);

    //     pin_mut!(stream);
    //     // Get first item from stream which should be the error
    //     let result = stream.next().await;

    //     // Verify we got an error result
    //     assert!(result.is_some(), "Expected error result from stream");

    //     match result.unwrap() {
    //         Err(err) => {
    //             assert!(
    //                 matches!(err, GatewayError::RequestError(_)),
    //                 "Expected RequestError, got {:?}",
    //                 err
    //             );
    //         }
    //         Ok(item) => panic!("Expected error, got successful response: {:?}", item),
    //     }

    //     // Verify mock was called
    //     mock.assert();
    //     Ok(())
    // }

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
}
