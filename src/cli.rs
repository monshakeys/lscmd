use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lscmd")]
#[command(about = "A high-performance CLI tool to manage shell aliases and functions")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    
    /// Full rescan - clear database and rebuild from all .alias files
    Scan,
    
    /// Incremental update - only reparse modified files
    Update,
    
    /// Change alias directory path (WARNING: triggers full rescan)
    Path { 
        /// New alias directory path
        new_path: PathBuf 
    },
    
    /// List all commands (opens TUI interface with case-insensitive filtering)
    List {
        /// Filter by command type ('alias' or 'function')
        #[arg(long)]
        type_filter: Option<String>,
    },
    
    /// Search commands by name/pattern (opens TUI interface with fixed case-insensitive matching)
    Search {
        /// Search query string
        query: String,
        
        /// Enable regex mode
        #[arg(long)]
        regex: bool,
        
        /// Filter by command type ('alias' or 'function')
        #[arg(long)]
        type_filter: Option<String>,
    },
    
    /// Show specific command details
    Show { 
        /// Command name to display
        name: String 
    },
    
    /// Initialize lscmd with interactive setup
    Init {
        /// Optional alias directory path
        #[arg(long)]
        path: Option<PathBuf>,
    },
}