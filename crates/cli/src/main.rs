use anyhow::Result;
use clap::Parser;
use lscmd_core::{list_commands, CommandType, FileCommands};
use owo_colors::OwoColorize;
use crossterm::terminal;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lscmd", version, about = "List shell commands (aliases and functions)")]
struct Cli {
    #[arg(help = "Root directory to scan (default: ~/.alias/)")]
    path: Option<PathBuf>,
    
    #[arg(short, long, default_value_t = 5, help = "Number of columns to display")]
    columns: usize,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    
    let root_path = cli.path.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".alias")
    });

    match list_commands(root_path) {
        Ok(file_commands) => {
            display_commands(&file_commands, cli.columns);
        }
        Err(e) => {
            eprintln!("{}: {}", "lscmd error".red(), e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn display_commands(file_commands: &[FileCommands], num_cols: usize) {
    for file_cmd in file_commands {
        println!("\n{}", format!("## {}", file_cmd.file_name.to_uppercase()).truecolor(255, 171, 0).bold());

        if file_cmd.commands.is_empty() {
            continue;
        }

        display_commands_grid(&file_cmd.commands, num_cols);
    }
}

fn display_commands_grid(commands: &[lscmd_core::Command], num_cols: usize) {
    if commands.is_empty() {
        return;
    }

    let term_width = terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(90);
    
    let col_width = (term_width.saturating_sub(num_cols * 2)) / num_cols;
    let total = commands.len();
    let rows = (total + num_cols - 1) / num_cols;

    for r in 0..rows {
        let mut line = String::new();
        
        for c in 0..num_cols {
            let index = r + c * rows;
            if index < total {
                let cmd = &commands[index];
                let colored_name = match cmd.command_type {
                    CommandType::Alias => cmd.name.bright_green().to_string(),
                    CommandType::Function => cmd.name.bright_blue().to_string(),
                };
                
                let padding = col_width.saturating_sub(cmd.name.len()).max(2);
                line.push_str(&colored_name);
                line.push_str(&" ".repeat(padding));
            }
        }
        
        println!("{}", line);
    }
}