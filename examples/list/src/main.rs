use std::collections::BTreeMap;
use std::env;

use inference_gateway_sdk::{GatewayError, InferenceGatewayAPI, InferenceGatewayClient, Provider};

#[tokio::main]
async fn main() -> Result<(), GatewayError> {
    let base_url = env::var("INFERENCE_GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    let client = InferenceGatewayClient::new(&base_url);

    println!("🔍 Inference Gateway SDK - List Examples");
    println!("=========================================\n");

    // Example 1: List all models
    println!("📋 Example 1: List All Models");
    let all_models = client.list_models().await?;
    println!(
        "Found {} total models across all providers:",
        all_models.data.len()
    );

    let mut models_by_provider: BTreeMap<String, Vec<&_>> = BTreeMap::new();
    for model in &all_models.data {
        models_by_provider
            .entry(model.served_by.to_string())
            .or_default()
            .push(model);
    }

    for (provider_name, models) in &models_by_provider {
        println!(
            "\n  {}: {} models",
            provider_name.to_uppercase(),
            models.len()
        );
        for model in models.iter().take(3) {
            println!("    • {} (created: {})", model.id, model.created);
        }
        if models.len() > 3 {
            println!("    ... and {} more", models.len() - 3);
        }
    }

    println!("\n---\n");

    // Example 2: List models from a specific provider (if PROVIDER is set)
    match env::var("PROVIDER") {
        Ok(provider_str) => {
            let provider: Provider = provider_str.parse().map_err(|e| {
                GatewayError::Other(Box::new(std::io::Error::other(format!(
                    "invalid PROVIDER '{provider_str}': {e}"
                ))))
            })?;

            println!(
                "📋 Example 2: List Models from {}",
                provider_str.to_uppercase()
            );
            let provider_models = client.list_models_by_provider(provider).await?;
            println!(
                "Found {} models from {}:",
                provider_models.data.len(),
                provider
            );

            for model in &provider_models.data {
                println!("  • {}", model.id);
                println!("    Owner: {}", model.owned_by);
                println!("    Created: {}", model.created);
                println!("    Served by: {}\n", model.served_by);
            }
        }
        Err(_) => {
            println!("📋 Example 2: Skipped (no PROVIDER specified)");
            println!("Set PROVIDER environment variable to see provider-specific models");
        }
    }

    println!("---\n");

    // Example 3: List MCP tools (if available)
    println!("🛠️  Example 3: List MCP Tools");
    match client.list_tools().await {
        Ok(tools) => {
            println!("Found {} MCP tools:", tools.data.len());

            if tools.data.is_empty() {
                println!("  No MCP tools available. Make sure MCP_EXPOSE=true on the gateway.");
            } else {
                for tool in &tools.data {
                    println!("\n  🔧 {}", tool.name);
                    println!("     Description: {}", tool.description);
                    println!("     Server: {}", tool.server);

                    if !tool.input_schema.is_empty() {
                        let schema_str =
                            serde_json::to_string_pretty(&tool.input_schema).unwrap_or_default();
                        let preview: String = schema_str.chars().take(100).collect();
                        println!("     Input schema: {preview}...");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ MCP tools not available (MCP_EXPOSE might be disabled)");
            println!("     Error: {e}");
        }
    }

    println!("\n---\n");

    // Example 4: Health check
    println!("❤️  Example 4: Health Check");
    let is_healthy = client.health_check().await?;
    println!(
        "Gateway health status: {}",
        if is_healthy {
            "✅ Healthy"
        } else {
            "❌ Unhealthy"
        }
    );

    println!("\n---\n");
    println!("✅ All examples completed successfully!");

    Ok(())
}
