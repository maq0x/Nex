mod agent;
mod cli;
mod config;
mod providers;
mod tools;
mod types;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConfigAction};
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle config subcommands
    if let Some(Commands::Config { action }) = &cli.command {
        return handle_config(action);
    }

    // Main agent task
    let task = cli
        .task
        .as_ref()
        .map(|parts| parts.join(" "))
        .filter(|s| !s.trim().is_empty());

    match task {
        Some(task) => {
            agent::run(task, &cli).await?;
        }
        None => {
            println!("Nex — AI Coding Agent\n");
            println!("Usage:");
            println!("  nex \"your task here\"");
            println!("  nex \"refactor the auth module\" --model gpt-4o");
            println!("  nex config set openai_api_key sk-...");
            println!("  nex config list");
        }
    }

    Ok(())
}

fn handle_config(action: &ConfigAction) -> Result<()> {
    let mut config = Config::load()?;

    match action {
        ConfigAction::Set { key, value } => {
            match key.as_str() {
                "openai_api_key" => config.openai_api_key = Some(value.clone()),
                "anthropic_api_key" => config.anthropic_api_key = Some(value.clone()),
                "openrouter_api_key" => config.openrouter_api_key = Some(value.clone()),
                "default_model" => config.default_model = Some(value.clone()),
                "default_provider" => config.default_provider = Some(value.clone()),
                "ollama_base_url" => config.ollama_base_url = Some(value.clone()),
                _ => {
                    println!("Unknown config key: {}", key);
                    return Ok(());
                }
            }
            config.save()?;
            println!("✓ Set {} successfully", key);
        }
        ConfigAction::Get { key } => {
            let value = match key.as_str() {
                "openai_api_key" => config.openai_api_key,
                "anthropic_api_key" => config.anthropic_api_key,
                "openrouter_api_key" => config.openrouter_api_key,
                "default_model" => config.default_model,
                "default_provider" => config.default_provider,
                "ollama_base_url" => config.ollama_base_url,
                _ => None,
            };
            match value {
                Some(v) => println!("{}", v),
                None => println!("(not set)"),
            }
        }
        ConfigAction::List => {
            println!("{:#?}", config);
        }
    }

    Ok(())
}
