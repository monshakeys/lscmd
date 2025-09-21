use lscmd::cli::{Cli, Commands};
use lscmd::error::Result;
use clap::Parser;

mod tui; // TUI is not part of the library yet

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Help => {
            // Clap handles help, but we might want a custom one.
            // For now, let's rely on clap's default.
            println!("Use 'lscmd --help' for more information.");
        },
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
