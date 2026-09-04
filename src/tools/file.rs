use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn read_file(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;

    // Limit very large files
    if content.len() > 100_000 {
        Ok(format!(
            "{}...\n\n[File truncated - too large]",
            &content[..100_000]
        ))
    } else {
        Ok(content)
    }
}

pub fn write_file(path: &str, content: &str) -> Result<String> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path))?;
    Ok(format!("Successfully wrote {} bytes to {}", content.len(), path))
}

pub fn list_dir(path: &str) -> Result<String> {
    let entries = fs::read_dir(path)
        .with_context(|| format!("Failed to list directory: {}", path))?;

    let mut lines = Vec::new();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let file_type = if entry.file_type()?.is_dir() {
            "dir "
        } else {
            "file"
        };
        lines.push(format!("{}  {}", file_type, name));
    }

    if lines.is_empty() {
        Ok("(empty directory)".to_string())
    } else {
        lines.sort();
        Ok(lines.join("\n"))
    }
}