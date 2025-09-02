pub mod command;
pub mod parser;
pub mod scanner;

use anyhow::Result;
use std::path::Path;

pub use command::{Command, CommandType};
pub use scanner::scan_directory;

#[derive(Debug)]
pub struct FileCommands {
    pub file_name: String,
    pub commands: Vec<Command>,
}

pub fn list_commands<P: AsRef<Path>>(root_path: P) -> Result<Vec<FileCommands>> {
    let shell_files = scan_directory(root_path)?;
    let mut file_commands = Vec::new();

    for file_path in shell_files {
        let file_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content = std::fs::read_to_string(&file_path)?;
        let commands = parser::parse_shell_commands(&content);

        if !commands.is_empty() {
            file_commands.push(FileCommands { file_name, commands });
        }
    }

    file_commands.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    for file_cmd in &mut file_commands {
        file_cmd.commands.sort_by(|a, b| a.name.cmp(&b.name));
    }

    Ok(file_commands)
}