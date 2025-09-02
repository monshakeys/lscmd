# lscmd - Shell Command Visualization Tool

A Rust-based shell command listing tool for scanning and displaying `alias` and `function` definitions in shell scripts.

## Features

- 🔍 **Auto Scanning**: Recursively scans all `.sh` files in specified directory
- 🎨 **Color Output**: File names (orange uppercase bold), aliases (green), functions (blue) with distinct colors
- 📊 **Multi-Column Layout**: Auto-adapts to terminal width with default 5-column layout
- 🔤 **Smart Sorting**: Files sorted alphabetically, commands sorted by name
- ⚡ **High Performance**: Built with Rust for fast processing of large file sets

## Installation

### Build from Source

```bash
# Clone the repository
git clone <repo-url>
cd lscmd

# Build
cargo build --release

# Run
./target/release/lscmd
```

## Usage

```bash
# Scan default directory ~/.alias/
lscmd

# Specify directory to scan
lscmd /path/to/shell/scripts

# Custom column count
lscmd -c 3 /path/to/scripts

# Show help
lscmd --help
```

## Output Example

```
## GIT-ALIASES
ga               gco              gp               gs               
gb               gd               gr               gst              

## DOCKER-ALIASES
dc               dps              drmi             
dexec            dr               dstop            
```

## Supported Syntax

### Aliases
```bash
alias ll='ls -la'
alias gs='git status'
```

### Functions
```bash
# function syntax
function my_func() {
    echo "hello"
}

# () syntax
another_func() {
    echo "world"
}
```

## Project Structure

```
lscmd/
├── Cargo.toml          # Workspace configuration
├── crates/
│   ├── core/           # Core library
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── command.rs
│   │   │   ├── parser.rs
│   │   │   └── scanner.rs
│   │   └── Cargo.toml
│   └── cli/            # Command line interface
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
└── README.md
```

## Technical Implementation

- **Language**: Rust 2021 Edition
- **Dependencies**: 
  - `clap` - CLI argument parsing
  - `regex` - Regular expression matching
  - `owo-colors` - Colored terminal output
  - `crossterm` - Cross-platform terminal control
  - `walkdir` - Recursive directory traversal

## Testing

```bash
# Run unit tests
cargo test

# Run specific tests
cargo test parser::tests
```

