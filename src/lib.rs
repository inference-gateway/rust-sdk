//! Inference Gateway SDK for Rust
//!
//! This crate provides a Rust client for the Inference Gateway API, allowing interaction
//! with various LLM providers through a unified interface.

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// Represents a model available through a provider
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    /// Unique identifier of the model
    pub id: String,
    /// Type of the object (always "model")
    pub object: String,
    /// Organization that owns the model
    pub owned_by: String,
    /// Unix timestamp of when the model was created
    pub created: i64,
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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Ollama,
    Groq,
    OpenAI,
    Google,
    Cloudflare,
    Cohere,
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

/// A message in a conversation with an LLM
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender ("system", "user" or "assistant")
    pub role: String,
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
}

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    /// Provider that generated the response
    pub provider: String,
    /// Content of the response
    pub response: ResponseContent,
}

#[derive(Debug, Deserialize)]
pub struct ResponseContent {
    /// Role of the responder (typically "assistant")
    pub role: String,
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

/// Core API interface for the Inference Gateway
pub trait InferenceGatewayAPI {
    /// Lists available models from all providers
    fn list_models(&self) -> Result<Vec<ProviderModels>, Box<dyn Error>>;

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
    ) -> Result<GenerateResponse, Box<dyn Error>>;

    /// Checks if the API is available
    fn health_check(&self) -> Result<bool, Box<dyn Error>>;
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
    fn list_models(&self) -> Result<Vec<ProviderModels>, Box<dyn Error>> {
        let url = format!("{}/llms", self.base_url);
        let mut request = self.client.get(&url);
        if let Some(token) = &self.token {
            request = self.client.get(&url).bearer_auth(token);
        }

        let response = request.send()?;
        let models = response.json()?;
        Ok(models)
    }

    fn generate_content(
        &self,
        provider: Provider,
        model: &str,
        messages: Vec<Message>,
    ) -> Result<GenerateResponse, Box<dyn Error>> {
        let url = format!("{}/llms/{}/generate", self.base_url, provider);
        let mut request = self.client.post(&url);
        if let Some(token) = &self.token {
            request = self.client.post(&url).bearer_auth(token);
        }

        let request_payload = GenerateRequest {
            model: model.to_string(),
            messages,
        };

        let response = request.json(&request_payload).send()?.json()?;
        Ok(response)
    }

    fn health_check(&self) -> Result<bool, Box<dyn Error>> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send()?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};

    #[test]
    fn test_authentication_header() {
        let mut server = Server::new();

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
        client.list_models().unwrap();
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
        client.list_models().unwrap();
        mock_without_auth.assert();
    }

    #[test]
    fn test_list_models() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/llms")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"provider":"ollama","models":[{"id":"llama2","object":"model","owned_by":"meta","created":1600000000}]}]"#)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let models = client.list_models().unwrap();

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].models[0].id, "llama2");
        mock.assert();
    }

    #[test]
    fn test_generate_content() {
        let mut server = Server::new();
        let mock = server
            .mock("POST", "/llms/ollama/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"provider":"ollama","response":{"role":"assistant","model":"llama2","content":"Hellloooo"}}"#)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }];
        let response = client
            .generate_content(Provider::Ollama, "llama2", messages)
            .unwrap();

        assert_eq!(response.provider, "ollama");
        assert_eq!(response.response.role, "assistant");
        assert_eq!(response.response.model, "llama2");
        assert_eq!(response.response.content, "Hellloooo");
        mock.assert();
    }

    #[test]
    fn test_health_check() {
        let mut server = Server::new();
        let mock = server.mock("GET", "/health").with_status(200).create();

        let client = InferenceGatewayClient::new(&server.url());
        let is_healthy = client.health_check().unwrap();

        assert!(is_healthy);
        mock.assert();
    }
}
