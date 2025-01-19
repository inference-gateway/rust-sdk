use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub owned_by: String,
    pub created: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderModels {
    pub provider: Provider,
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Ollama,
    Groq,
    OpenAI,
    Google,
    Cloudflare,
    Cohere,
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
        }
    }
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub provider: String,
    pub response: String,
}

pub struct InferenceGatewayClient {
    base_url: String,
    client: Client,
}

impl InferenceGatewayClient {
    pub fn new(base_url: &str) -> Self {
        InferenceGatewayClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub fn list_models(&self) -> Result<Vec<ProviderModels>, Box<dyn Error>> {
        let url = format!("{}/llms", self.base_url);
        let response = self.client.get(&url).send()?;
        let models = response.json()?;
        Ok(models)
    }

    pub fn generate_content(
        &self,
        provider: Provider,
        model: &str,
        prompt: &str,
    ) -> Result<GenerateResponse, Box<dyn Error>> {
        let url = format!("{}/llms/{}/generate", self.base_url, provider);
        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
        };

        let response = self.client.post(&url).json(&request).send()?.json()?;

        Ok(response)
    }

    pub fn health_check(&self) -> Result<bool, Box<dyn Error>> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send()?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

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
            .with_body(r#"{"provider":"ollama","response":"Generated text"}"#)
            .create();

        let client = InferenceGatewayClient::new(&server.url());
        let response = client
            .generate_content(Provider::Ollama, "llama2", "Hello")
            .unwrap();

        assert_eq!(response.provider, "ollama");
        assert_eq!(response.response, "Generated text");
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
