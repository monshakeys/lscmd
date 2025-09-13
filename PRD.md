# lscmd Rust CLI - Product Requirements Document

## Overview
A high-performance Rust CLI tool to parse and index shell aliases and functions from `.alias/*.sh` files, storing them in SQLite3 for fast querying with a beautiful TUI interface.

## Current State Analysis
Based on the existing `lscmd.sh` script, the tool currently:
- Scans `$HOME/.alias/*.sh` files for aliases and functions
- Uses awk to parse shell script syntax
- Displays results in a 5-column colored layout optimized for dark terminals
- Supports both `alias name="command"` and `function name() { ... }` patterns

## Requirements

### Core Functionality
1. **File Discovery**: Recursively scan `.alias/*` directory for `*.sh` files
2. **Shell Script Parsing**: Extract aliases and functions using regex patterns
3. **Database Storage**: Store parsed data in SQLite3 with proper indexing
4. **Full Rebuild**: `scan` command clears database and rebuilds from scratch
5. **Incremental Update**: `update` command only processes modified files (by mtime)
6. **Performance**: Handle ~20 shell files with parallel processing
7. **TUI Interface**: Beautiful terminal interface for browsing and searching
8. **CLI Arguments**: Comprehensive command-line interface

### Database Schema
```sql
CREATE TABLE commands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,           -- alias/function name (indexed)
    type TEXT NOT NULL,           -- 'alias' or 'function'
    path TEXT NOT NULL,           -- source .sh file path
    code TEXT NOT NULL,           -- command content/function body
    file_mtime INTEGER NOT NULL, -- last modified time of source file
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    UNIQUE(name, path)            -- prevent duplicates
);

CREATE INDEX idx_name ON commands(name);
CREATE INDEX idx_type ON commands(type);
CREATE INDEX idx_path ON commands(path);
```

### Parsing Requirements
- **Alias Pattern**: `alias name="command"` or `alias name='command'`
- **Function Patterns**:
  - `function name() { ... }`
  - `name() { ... }`
- **Code Extraction**:
  - Aliases: content within quotes
  - Functions: entire function body including braces
- **Multi-line Support**: Handle functions spanning multiple lines

### Technology Stack
- **CLI Framework**: `clap` for argument parsing
- **Database**: `sqlx` for SQLite3 operations
- **TUI**: `ratatui` (modern TUI library)
- **Parsing**: `regex` for detailed pattern extraction, `aho-corasick` for fast multi-pattern scanning
- **Parallel Processing**: `rayon` for concurrent file processing
- **File Operations**: `walkdir` for directory traversal

### CLI Interface
```
lscmd [OPTIONS] [COMMAND]

Commands:
    scan        Full rescan - clear database and rebuild from all .alias files
    list        List all commands
    search      Search commands by name/pattern
    show        Show specific command details
    update      Incremental update - only reparse modified files
    
Options:
    -p, --path <PATH>       Custom alias directory path [default: ~/.alias]
```

### Performance Considerations
1. **Parallel Processing**: Use `rayon` to process multiple .sh files concurrently
2. **Full Rebuild (scan)**: Clear entire database, then rebuild from all files in parallel
3. **Incremental Updates (update)**: Only reparse files that have changed (compare mtime)
4. **Database Indexing**: Proper indices on name, type, and path columns
5. **Fast Pattern Scanning**: Use `aho-corasick` to simultaneously search for `alias`, `function`, and `()` patterns in one pass
6. **Memory Efficiency**: Stream processing for large files
7. **Caching**: Cache database connections and prepared statements

### TUI Features
- **Interactive Search**: Real-time filtering as you type
- **Syntax Highlighting**: Color-coded display for aliases vs functions
- **File Navigation**: Browse by source file
- **Details View**: Show full command/function body
- **Export Options**: Copy to clipboard, save to file

### Error Handling
- Graceful handling of malformed shell scripts
- Clear error messages for database issues
- Fallback behavior when TUI cannot be displayed
- Logging for debugging parsing issues

### File Structure
```
src/
├── main.rs              # CLI entry point
├── cli.rs               # Clap command definitions
├── parser.rs            # Shell script parsing logic
├── database/
│   ├── mod.rs
│   ├── schema.rs        # Database schema and migrations
│   └── operations.rs    # CRUD operations
├── tui/
│   ├── mod.rs
│   ├── app.rs          # TUI application state
│   ├── ui.rs           # UI rendering
│   └── events.rs       # Event handling
└── utils/
    ├── mod.rs
    ├── file_scanner.rs  # File discovery and monitoring
    └── parallel.rs      # Parallel processing utilities
```

### Migration Strategy
1. **Phase 1**: Basic CLI with scan and list functionality
2. **Phase 2**: Add TUI interface and search capabilities  
3. **Phase 3**: Performance optimization and advanced features
4. **Phase 4**: Configuration and customization options

### Success Metrics
- Parse 20+ .sh files in under 1 second
- Sub-100ms query response time for searches
- Support for 1000+ aliases/functions without performance degradation
- Zero data loss during incremental updates
- Intuitive TUI navigation with keyboard shortcuts

## Technical Notes
- Use prepared statements for all database operations
- Implement proper SQL escaping for shell command content
- Handle edge cases like nested quotes and escaped characters
- Support both zsh and bash syntax variations
- Maintain backward compatibility with existing .alias structure
- Use case-insensitive matching by default for searches
- Use Aho-Corasick to scan for multiple parsing patterns (`alias `, `function `, `name()`) in single pass
- Fall back to regex for detailed extraction after Aho-Corasick identifies candidate lines