//! Inference Gateway SDK for Rust
//!
//! This crate provides a Rust client for the Inference Gateway API, allowing interaction
//! with various LLM providers through a unified interface.
//!
//! Data types in [`crate::generated::schemas`] are generated from the upstream
//! `openapi.yaml` and re-exported at the crate root. Run `task generate-types`
//! to regenerate them after a spec bump.

mod ext;
mod generated;

pub use generated::schemas::*;

use std::future::Future;

use futures_util::{Stream, StreamExt};
use reqwest::{Client, StatusCode};
use thiserror::Error;

/// Stream of Server-Sent Events (SSE) yielded by [`InferenceGatewayAPI::generate_content_stream`].
///
/// This is the SDK's own SSE wrapper used by the streaming function. It is distinct
/// from the spec's [`SsEvent`] (which constrains `event` to a fixed enum) - the
/// streaming function may surface arbitrary event names produced by upstream providers.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

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

/// Client for interacting with the Inference Gateway API
pub struct InferenceGatewayClient {
    base_url: String,
    client: Client,
    token: Option<String>,
    tools: Option<Vec<ChatCompletionTool>>,
    max_tokens: Option<i64>,
}

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
    fn list_models(&self) -> impl Future<Output = Result<ListModelsResponse, GatewayError>> + Send;

    /// Lists available models filtered by a specific provider
    fn list_models_by_provider(
        &self,
        provider: Provider,
    ) -> impl Future<Output = Result<ListModelsResponse, GatewayError>> + Send;

    /// Lists available models with additional metadata included, optionally
    /// filtered by a provider. Supported `include` values: `"pricing"`,
    /// `"context_window"` - they populate [`Model::pricing`] and
    /// [`Model::context_window`] respectively.
    fn list_models_with_include(
        &self,
        provider: Option<Provider>,
        include: &[&str],
    ) -> impl Future<Output = Result<ListModelsResponse, GatewayError>> + Send;

    /// Generates content using a specified model
    fn generate_content(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Future<Output = Result<CreateChatCompletionResponse, GatewayError>> + Send;

    /// Streams content generation as SSE events from the gateway.
    fn generate_content_stream(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send;

    /// Creates a message via the Anthropic-compatible Messages API.
    ///
    /// Providers without Messages support return [`GatewayError::BadRequest`];
    /// use [`InferenceGatewayAPI::generate_content`] for those providers.
    fn create_message(
        &self,
        provider: Option<Provider>,
        request: CreateMessagesRequest,
    ) -> impl Future<Output = Result<MessagesResponse, GatewayError>> + Send;

    /// Streams a message via the Messages API as SSE events. Each event's
    /// `data` field holds a JSON-serialized [`MessagesStreamEvent`].
    fn create_message_stream(
        &self,
        provider: Option<Provider>,
        request: CreateMessagesRequest,
    ) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send;

    /// Lists available MCP tools (only when `EXPOSE_MCP=true` server-side)
    fn list_tools(&self) -> impl Future<Output = Result<ListToolsResponse, GatewayError>> + Send;

    /// Health probe - returns true on HTTP 200, false otherwise.
    fn health_check(&self) -> impl Future<Output = Result<bool, GatewayError>> + Send;
}

impl InferenceGatewayClient {
    /// Creates a new client targeting `base_url`.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
            token: None,
            tools: None,
            max_tokens: None,
        }
    }

    /// Creates a client using `INFERENCE_GATEWAY_URL` (or `http://localhost:8080/v1`).
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

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Sets the tools used for subsequent generations.
    pub fn with_tools(mut self, tools: Option<Vec<ChatCompletionTool>>) -> Self {
        self.tools = tools;
        self
    }

    /// Sets the bearer token used for authentication.
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets an upper bound for tokens generated per request.
    pub fn with_max_tokens(mut self, max_tokens: Option<i64>) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// The gateway serves `/health` from the root server, not under the
    /// versioned API prefix, so this strips a trailing `/v<digits>` segment
    /// from the configured base URL before appending `/health`.
    fn health_url(&self) -> String {
        let trimmed = self.base_url.trim_end_matches('/');
        let root = match trimmed.rsplit_once('/') {
            Some((prefix, last))
                if last.len() >= 2
                    && last.starts_with('v')
                    && last[1..].chars().all(|c| c.is_ascii_digit()) =>
            {
                prefix
            }
            _ => trimmed,
        };
        format!("{root}/health")
    }

    fn messages_url(&self, provider: Option<Provider>) -> String {
        match provider {
            Some(provider) => format!("{}/messages?provider={provider}", self.base_url),
            None => format!("{}/messages", self.base_url),
        }
    }

    fn build_chat_request(
        &self,
        model: &str,
        messages: Vec<Message>,
        stream: bool,
    ) -> CreateChatCompletionRequest {
        // `tools` and `max_tokens` are deliberately omitted from streaming
        // requests; every other field falls back to the schema defaults via
        // `Default`. See CLAUDE.md for the streaming asymmetry.
        CreateChatCompletionRequest {
            model: model.to_string(),
            messages,
            stream,
            tools: if stream {
                Vec::new()
            } else {
                self.tools.clone().unwrap_or_default()
            },
            max_tokens: if stream { None } else { self.max_tokens },
            ..Default::default()
        }
    }
}

async fn map_error_status(status: StatusCode, response: reqwest::Response) -> GatewayError {
    // Gateway errors are `{"error": "..."}`; Messages endpoints use the
    // Anthropic shape `{"type": "error", "error": {"type": ..., "message": ...}}`.
    let fallback = || status.canonical_reason().unwrap_or("unknown").to_string();
    let message = match response.json::<serde_json::Value>().await {
        Ok(body) => match body.get("error") {
            Some(serde_json::Value::String(error)) => error.clone(),
            Some(error) => error
                .get("message")
                .and_then(|m| m.as_str())
                .map(str::to_string)
                .unwrap_or_else(fallback),
            None => fallback(),
        },
        Err(_) => fallback(),
    };
    match status {
        StatusCode::UNAUTHORIZED => GatewayError::Unauthorized(message),
        StatusCode::FORBIDDEN => GatewayError::Forbidden(message),
        StatusCode::NOT_FOUND => GatewayError::NotFound(message),
        StatusCode::BAD_REQUEST => GatewayError::BadRequest(message),
        StatusCode::INTERNAL_SERVER_ERROR => GatewayError::InternalError(message),
        other => GatewayError::Other(Box::new(std::io::Error::other(format!(
            "Unexpected status code: {other}"
        )))),
    }
}

fn sse_stream<B>(
    client: Client,
    token: Option<String>,
    url: String,
    body: B,
) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send
where
    B: serde::Serialize + Send + 'static,
{
    async_stream::try_stream! {
        let mut request = client.post(&url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        let response = request.json(&body).send().await?;
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
                        retry: None,
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

impl InferenceGatewayClient {
    async fn fetch_models(&self, query: &str) -> Result<ListModelsResponse, GatewayError> {
        let url = if query.is_empty() {
            format!("{}/models", self.base_url)
        } else {
            format!("{}/models?{}", self.base_url, query)
        };
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(map_error_status(status, response).await),
        }
    }
}

impl InferenceGatewayAPI for InferenceGatewayClient {
    async fn list_models(&self) -> Result<ListModelsResponse, GatewayError> {
        self.fetch_models("").await
    }

    async fn list_models_by_provider(
        &self,
        provider: Provider,
    ) -> Result<ListModelsResponse, GatewayError> {
        self.fetch_models(&format!("provider={provider}")).await
    }

    async fn list_models_with_include(
        &self,
        provider: Option<Provider>,
        include: &[&str],
    ) -> Result<ListModelsResponse, GatewayError> {
        let mut query = Vec::new();
        if let Some(provider) = provider {
            query.push(format!("provider={provider}"));
        }
        if !include.is_empty() {
            query.push(format!("include={}", include.join(",")));
        }
        self.fetch_models(&query.join("&")).await
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

        let payload = self.build_chat_request(model, messages, false);
        let response = request.json(&payload).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(map_error_status(status, response).await),
        }
    }

    fn generate_content_stream(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send {
        let url = format!("{}/chat/completions?provider={}", self.base_url, provider);
        let request_body = self.build_chat_request(model, messages, true);
        sse_stream(self.client.clone(), self.token.clone(), url, request_body)
    }

    async fn create_message(
        &self,
        provider: Option<Provider>,
        mut request: CreateMessagesRequest,
    ) -> Result<MessagesResponse, GatewayError> {
        request.stream = false;
        let mut req = self.client.post(self.messages_url(provider));
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let response = req.json(&request).send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(map_error_status(status, response).await),
        }
    }

    fn create_message_stream(
        &self,
        provider: Option<Provider>,
        mut request: CreateMessagesRequest,
    ) -> impl Stream<Item = Result<SSEvents, GatewayError>> + Send {
        request.stream = true;
        sse_stream(
            self.client.clone(),
            self.token.clone(),
            self.messages_url(provider),
            request,
        )
    }

    async fn list_tools(&self) -> Result<ListToolsResponse, GatewayError> {
        let url = format!("{}/mcp/tools", self.base_url);
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;
        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            status => Err(map_error_status(status, response).await),
        }
    }

    async fn health_check(&self) -> Result<bool, GatewayError> {
        let response = self.client.get(self.health_url()).send().await?;
        Ok(response.status() == StatusCode::OK)
    }
}

#[cfg(test)]
mod tests;
