pub mod ollama;
pub mod openai;

use crate::config::Config;
use crate::types::{Message, ToolDefinition};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Provider {
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<ProviderResponse>;
}

pub struct ProviderResponse {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<crate::types::ToolCall>>,
}

pub fn get_provider(name: &str, config: &Config) -> Result<Box<dyn Provider>> {
    match name.to_lowercase().as_str() {
        "openai" | "openrouter" => Ok(Box::new(openai::OpenAIProvider::new(config, name == "openrouter")?)),
        "ollama" => Ok(Box::new(ollama::OllamaProvider::new(config))),
        _ => anyhow::bail!("Unknown provider: {}. Supported: openai, openrouter, ollama", name),
    }
}
