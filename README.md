# lscmd - Shell Command Visualization Tool

A Rust-based shell command listing tool for scanning and displaying `alias` and `function` definitions in shell scripts.

## Features

- 🔍 **Auto Scanning**: Recursively scans all `.sh` files in specified directory
- 🎨 **Color Output**: File names (orange uppercase bold), aliases (green), functions (blue) with distinct colors
- 📊 **Multi-Column Layout**: Auto-adapts to terminal width with default 5-column layout
- 🔤 **Smart Sorting**: Files sorted alphabetically, commands sorted by name
- ⚡ **High Performance**: Built with Rust for fast processing of large file sets

## Installation

### Homebrew (Recommended)

```bash
brew install monshakeys/tap/lscmd
```

### Alternative Methods

If you prefer not to use Homebrew, you can install using one of these methods:

#### Build from Source

```bash
# Clone the repository
git clone <repo-url>
cd lscmd

# For development - fast compilation
cargo build
./target/debug/lscmd

# For production - optimized performance
cargo build --release
./target/release/lscmd

# Or run directly without building executable
cargo run              # debug mode
cargo run --release    # release mode

# Install to system (optional)
cargo install --path .
```

#### Using Cargo

If you have Rust installed:

```bash
cargo install --git <repo-url>
```

#### Download Binary

Download the latest binary from the [releases page](https://github.com/monshakeys/lscmd/releases) and place it in your `$PATH`.

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

## Development

### Building and Running

```bash
# Development build (fast compilation, includes debug info)
cargo build
./target/debug/lscmd

# Release build (optimized, smaller binary)
cargo build --release
./target/release/lscmd

# Run without building (convenient for development)
cargo run -- --help              # debug mode with arguments
cargo run --release -- /path     # release mode with arguments
```

### Testing

```bash
# Run unit tests
cargo test

# Run specific tests
cargo test parser::tests

# Run tests with output
cargo test -- --nocapture
```

