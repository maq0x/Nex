use anyhow::{Context, Result};
use std::process::Command;

pub fn run_command(command: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .with_context(|| format!("Failed to run command: {}", command))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        if stdout.trim().is_empty() {
            Ok("(no output)".to_string())
        } else {
            Ok(stdout.to_string())
        }
    } else {
        Ok(format!(
            "Exit code: {}\n{}\n{}",
            output.status.code().unwrap_or(-1),
            stdout,
            stderr
        ))
    }
}