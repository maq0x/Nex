use crate::cli::Cli;
use crate::config::Config;
use crate::providers::{get_provider, ProviderResponse};
use crate::tools::{available_tools, execute_tool};
use crate::types::{Message, Role};
use anyhow::Result;
use colored::Colorize;

pub async fn run(task: String, cli: &Cli) -> Result<()> {
    let config = Config::load()?;

    let provider_name = cli
        .provider
        .clone()
        .or(config.default_provider.clone())
        .unwrap_or_else(|| "openai".to_string());

    let model = cli
        .model
        .clone()
        .or(config.default_model.clone())
        .unwrap_or_else(|| "gpt-4o".to_string());

    println!("{}", "Nex".cyan().bold());
    println!("{} {}", "Provider:".dimmed(), provider_name);
    println!("{} {}", "Model:".dimmed(), model);
    println!("\n{} {}\n", "Task:".bold(), task);

    let provider = get_provider(&provider_name, &config)?;
    let tools = available_tools();

    let mut messages = vec![
        Message {
            role: Role::System,
            content: Some(build_system_prompt()),
            tool_calls: None,
            tool_call_id: None,
        },
        Message {
            role: Role::User,
            content: Some(task),
            tool_calls: None,
            tool_call_id: None,
        },
    ];

    let max_steps = 20;

    for step in 1..=max_steps {
        print!("{} ", format!("[step {}]", step).dimmed());
        println!("{}", "Thinking...".dimmed());

        let response: ProviderResponse = provider.chat(&model, &messages, &tools).await?;

        // Add assistant message
        messages.push(Message {
            role: Role::Assistant,
            content: response.content.clone(),
            tool_calls: response.tool_calls.clone(),
            tool_call_id: None,
        });

        if let Some(content) = &response.content {
            if !content.trim().is_empty() {
                println!("{}\n", content);
            }
        }

        // No tool calls → done
        let tool_calls = match &response.tool_calls {
            Some(tcs) if !tcs.is_empty() => tcs,
            _ => {
                println!("{}", "✓ Done".green().bold());
                return Ok(());
            }
        };

        if cli.plan {
            println!("{}", "Plan-only mode — stopping before tool execution.".yellow());
            for tc in tool_calls {
                println!("  → {}({})", tc.function.name, tc.function.arguments);
            }
            return Ok(());
        }

        // Execute tools
        for tc in tool_calls {
            let name = &tc.function.name;
            let args = &tc.function.arguments;

            println!("{} {}", "→".blue(), name.bold());

            let result = execute_tool(name, args).await?;

            // Show short preview of result
            let preview = if result.len() > 300 {
                format!("{}...", &result[..300])
            } else {
                result.clone()
            };
            println!("{}\n", preview.dimmed());

            messages.push(Message {
                role: Role::Tool,
                content: Some(result),
                tool_calls: None,
                tool_call_id: Some(tc.id.clone()),
            });
        }
    }

    println!("{}", "Reached maximum steps. Stopping.".yellow());
    Ok(())
}

fn build_system_prompt() -> String {
    format!(
        r#"You are Nex, a fast and precise AI coding agent that lives in the terminal.

You help users with coding tasks by reading files, editing code, running commands, and using git.

Guidelines:
- Be concise and action-oriented.
- Prefer making correct, minimal changes.
- Always read relevant files before editing them.
- Use tools to gather information instead of guessing.
- When you are done, respond with a short summary of what you did. Do not call tools in the final response.
- If a task is unclear, ask a clarifying question instead of guessing.

Current working directory: {}"#,
        std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| ".".to_string())
    )
}
