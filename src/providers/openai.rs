use super::{Provider, ProviderResponse};
use crate::config::Config;
use crate::types::{Message, ToolDefinition};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(config: &Config, is_openrouter: bool) -> Result<Self> {
        let (api_key, base_url) = if is_openrouter {
            let key = config
                .openrouter_api_key
                .clone()
                .context("OpenRouter API key not set. Run: nex config set openrouter_api_key sk-or-...")?;
            (key, "https://openrouter.ai/api/v1".to_string())
        } else {
            let key = config
                .openai_api_key
                .clone()
                .context("OpenAI API key not set. Run: nex config set openai_api_key sk-...")?;
            (key, "https://api.openai.com/v1".to_string())
        };

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
        })
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<ProviderResponse> {
        let mut body = json!({
    "model": model,
    "messages": messages,
    "max_tokens": 2048,
    });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
            body["tool_choice"] = json!("auto");
        }

        let res = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error ({}): {}", status, error_text);
        }

        let data: Value = res.json().await?;
        let message = &data["choices"][0]["message"];

        let content = message["content"].as_str().map(|s| s.to_string());
        
        let tool_calls = if let Some(tcs) = message.get("tool_calls") {
            serde_json::from_value(tcs.clone()).ok()
        } else {
            None
        };

        Ok(ProviderResponse {
            content,
            tool_calls,
        })
    }
}
