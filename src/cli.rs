use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nex")]
#[command(about = "Fast, lightweight AI coding agent for the terminal", long_about = None)]
pub struct Cli {
    /// Natural language task for the agent
    pub task: Option<Vec<String>>,

    /// Model to use (e.g. gpt-4o, claude-sonnet, llama3.2)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Provider (openai, anthropic, openrouter, ollama)
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Auto-approve all tool calls (use with caution)
    #[arg(long)]
    pub yolo: bool,

    /// Plan only — do not execute any tools
    #[arg(long)]
    pub plan: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Set a config value
    Set {
        /// Config key (e.g. openai_api_key, default_model)
        key: String,
        /// Value to set
        value: String,
    },
    /// Get a config value
    Get {
        /// Config key
        key: String,
    },
    /// List all config
    List,
}
