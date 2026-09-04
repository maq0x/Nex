pub mod file;
pub mod git;
pub mod shell;

use crate::types::ToolDefinition;
use anyhow::Result;
use serde_json::json;

/// Returns the list of tools available to the agent
pub fn available_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            kind: "function".to_string(),
            function: crate::types::FunctionDefinition {
                name: "read_file".to_string(),
                description: "Read the contents of a file".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        ToolDefinition {
            kind: "function".to_string(),
            function: crate::types::FunctionDefinition {
                name: "write_file".to_string(),
                description: "Write content to a file (creates or overwrites)".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content to write"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        },
        ToolDefinition {
            kind: "function".to_string(),
            function: crate::types::FunctionDefinition {
                name: "list_dir".to_string(),
                description: "List files and directories in a path".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directory path (default: current directory)"
                        }
                    }
                }),
            },
        },
        ToolDefinition {
            kind: "function".to_string(),
            function: crate::types::FunctionDefinition {
                name: "run_command".to_string(),
                description: "Run a shell command and return the output".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The command to run"
                        }
                    },
                    "required": ["command"]
                }),
            },
        },
        ToolDefinition {
            kind: "function".to_string(),
            function: crate::types::FunctionDefinition {
                name: "git_status".to_string(),
                description: "Get the current git status".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        },
    ]
}

/// Execute a tool by name with the given arguments (as JSON string)
pub async fn execute_tool(name: &str, arguments: &str) -> Result<String> {
    let args: serde_json::Value = serde_json::from_str(arguments)?;

    match name {
        "read_file" => {
            let path = args["path"].as_str().unwrap_or("");
            file::read_file(path)
        }
        "write_file" => {
            let path = args["path"].as_str().unwrap_or("");
            let content = args["content"].as_str().unwrap_or("");
            file::write_file(path, content)
        }
        "list_dir" => {
            let path = args["path"].as_str().unwrap_or(".");
            file::list_dir(path)
        }
        "run_command" => {
            let command = args["command"].as_str().unwrap_or("");
            shell::run_command(command)
        }
        "git_status" => git::git_status(),
        _ => Ok(format!("Unknown tool: {}", name)),
    }
}