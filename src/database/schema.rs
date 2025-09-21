/// Defines the SQL schema for the commands database.
///
/// This function returns the SQL statements required to create the necessary
/// tables and indexes for the lscmd application. It is designed to be
/// executed during the database initialization process.
///
/// The schema includes:
/// - A `commands` table to store aliases and functions.
/// - Indexes to optimize search performance, including a case-insensitive
///   index on the command name.
pub fn get_schema() -> &'static str {
    r#"
-- Create the main table for storing shell commands.
-- The `name` is the PRIMARY KEY to ensure uniqueness.
CREATE TABLE IF NOT EXISTS commands (
    name TEXT PRIMARY KEY NOT NULL,
    cmd_type TEXT NOT NULL,      -- 'alias' or 'function'
    path TEXT NOT NULL,          -- Absolute path to the source file
    code TEXT NOT NULL,          -- The actual command or function body
    file_mtime INTEGER NOT NULL, -- Last modification time of the source file
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Create an index on the command type for faster filtering.
CREATE INDEX IF NOT EXISTS idx_type ON commands(cmd_type);

-- Create an index on the file path to efficiently find all commands from a specific file.
CREATE INDEX IF NOT EXISTS idx_path ON commands(path);

-- Create an index on the lowercased command name to ensure fast, case-insensitive searches.
-- This is a key performance optimization for the search functionality.
CREATE INDEX IF NOT EXISTS idx_name_lower ON commands(LOWER(name));
    "#
}
