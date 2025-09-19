# lscmd - List My Custom Commands

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

- Download the latest binary from the [releases page](https://github.com/monshakeys/lscmd/releases) and place it in your `$PATH`.
- Or these methods:

```bash
# clone
git clone git@github.com:monshakeys/lscmd.git
cd lscmd

# for dev
cargo build
./target/debug/lscmd

# for prod
cargo build --locked --release
./target/release/lscmd

# run without building executable
cargo run              # debug mode
cargo run --release    # release mode

# install to path
cargo install --path .
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
