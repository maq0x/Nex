use anyhow::Result;
use super::shell::run_command;

pub fn git_status() -> Result<String> {
    run_command("git status --short")
}