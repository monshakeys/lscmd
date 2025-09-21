mod cli;
mod error;
mod config;
mod database;
mod parser;
mod tui;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan => {
            println!("Full scan - TODO: implement in Phase 4");
        },
        Commands::Update => {
            println!("Incremental update - TODO: implement in Phase 4");
        },
        Commands::Path { new_path } => {
            println!("Change path to: {:?} - TODO: implement in Phase 4", new_path);
        },
        Commands::List { type_filter } => {
            println!("List commands (filter: {:?}) - TODO: implement TUI in Phase 6", type_filter);
        },
        Commands::Search { query, regex, type_filter } => {
            println!("Search: '{}' (regex: {}, filter: {:?}) - TODO: implement in Phase 4", 
                    query, regex, type_filter);
        },
        Commands::Show { name } => {
            println!("Show command: '{}' - TODO: implement in Phase 4", name);
        },
        Commands::Init { path } => {
            println!("Initialize with path: {:?} - TODO: implement in Phase 7", path);
        },
    }

    Ok(())
}

fn print_help() {
    println!("lscmd - High-performance CLI tool for shell aliases and functions");
    println!();
    println!("USAGE:");
    println!("    lscmd [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    help        Show this help message");
    println!("    scan        Full rescan - clear database and rebuild from all .alias files");
    println!("    update      Incremental update - only reparse modified files");
    println!("    path <PATH> Change alias directory path (WARNING: triggers full rescan)");
    println!("    list        List all commands (opens TUI interface)");
    println!("    search      Search commands by name/pattern (opens TUI interface)");
    println!("    show        Show specific command details");
    println!("    init        Initialize lscmd with interactive setup");
    println!();
    println!("For more information on a specific command, use: lscmd <command> --help");
}
