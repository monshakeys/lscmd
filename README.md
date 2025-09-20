# lscmd - List My Custom Commands

A Rust-based shell command listing tool for scanning and displaying `alias` and `function` definitions in shell scripts.

## Features

- 🧠 **Instant Command Recall**: Never forget the aliases and functions you've written
  > *"Dauthen, a DevOps engineer, had accumulated 150+ custom shell shortcuts over 3 years. She often found herself rewriting commands she knew she'd created before, wasting precious time. With lscmd, she instantly recalls any command with a simple search."*

- ⚡ **Lightning Search**: Find any command in milliseconds, no matter how many you have
  > *"When Dauthen needed that Docker cleanup function she wrote months ago, instead of grep-ing through dozens of files for 15 minutes, she simply typed 'docker clean' and found it instantly."*

- 🔍 **Fuzzy Memory Recovery**: Can't remember the exact name? No problem - partial matches work perfectly
  > *"Dauthen vaguely remembered writing something about 'git branch cleanup' but couldn't recall if it was called 'cleanup', 'clean', or 'prune'. Fuzzy search found it immediately with just 'git clean'."*

- 🚀 **Zero Maintenance**: Automatically discovers and syncs changes without any manual work
  > *"Dauthen creates new aliases weekly. The tool automatically detects when she adds new commands or modifies existing ones, keeping everything perfectly synchronized without her lifting a finger."*

- 🎯 **Smart Discovery**: Uncover forgotten gems in your command collection
  > *"Dauthen spent 2 hours debugging a complex deployment script, only to discover she had already written the perfect solution 6 months ago but completely forgot about it. The deadline pressure and repeated work could have been avoided if she knew what tools she already possessed."*

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
