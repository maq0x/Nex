use super::{Provider, ProviderResponse};
use crate::config::Config;
use crate::types::{Message, ToolDefinition};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

impl OllamaProvider {
    pub fn new(config: &Config) -> Self {
        let base_url = config
            .ollama_base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:11434".to_string());

        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        _tools: &[ToolDefinition],
    ) -> Result<ProviderResponse> {
        // Note: Tool calling support in Ollama varies by model.
        // For MVP we send a simplified request without tools.

        let model_name = model.strip_prefix("ollama/").unwrap_or(model);

        let body = json!({
            "model": model_name,
            "messages": messages,
            "stream": false,
        });

        let res = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Ollama. Is Ollama running?")?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            anyhow::bail!("Ollama error ({}): {}", status, error_text);
        }

        let data: Value = res.json().await?;
        let content = data["message"]["content"]
            .as_str()
            .map(|s| s.to_string());

        Ok(ProviderResponse {
            content,
            tool_calls: None, // Tool calling for Ollama will be improved later
        })
    }
}
