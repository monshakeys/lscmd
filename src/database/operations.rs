use super::{Command, Result};
use crate::error::LscmdError;
use crate::search::SearchEngine;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// Trait defining the interface for command persistence.
///
/// All operations are synchronous to align with the project's architecture requirements.
pub trait CommandRepository: Send + Sync {
    fn insert_command(&self, command: &Command) -> Result<()>;
    fn batch_insert(&self, commands: &[Command]) -> Result<()>;
    fn search_commands(&self, engine: &SearchEngine, query: &str) -> Result<Vec<Command>>;
    fn get_command_by_name(&self, name: &str) -> Result<Option<Command>>;
    fn delete_by_path(&self, path: &str) -> Result<u64>;
    fn clear_all(&self) -> Result<u64>;
    fn get_file_mtime(&self, path: &str) -> Result<Option<i64>>;
    fn health_check(&self) -> Result<()>;
}

/// An implementation of `CommandRepository` using `rusqlite` with a SQLite backend.
///
/// Uses Arc<Mutex<Connection>> to allow safe concurrent access while maintaining
/// the synchronous interface required by the project architecture.
#[derive(Debug)]
pub struct SqliteCommandRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteCommandRepository {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl CommandRepository for SqliteCommandRepository {
    /// Inserts a single command, replacing it if it already exists.
    fn insert_command(&self, command: &Command) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO commands (name, cmd_type, path, code, file_mtime) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&command.name, &command.cmd_type, &command.path, &command.code, command.file_mtime],
        )
        .map_err(|e| LscmdError::Database(e.to_string()))?;
        Ok(())
    }

    /// Inserts a batch of commands in a single transaction.
    fn batch_insert(&self, commands: &[Command]) -> Result<()> {
        if commands.is_empty() {
            return Ok(());
        }

        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction().map_err(|e| LscmdError::Database(e.to_string()))?;

        for command in commands {
            tx.execute(
                "INSERT OR REPLACE INTO commands (name, cmd_type, path, code, file_mtime) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![&command.name, &command.cmd_type, &command.path, &command.code, command.file_mtime],
            )
            .map_err(|e| LscmdError::Database(e.to_string()))?;
        }

        tx.commit().map_err(|e| LscmdError::Database(e.to_string()))?;
        Ok(())
    }

    /// Searches commands by first fetching all and then using the search engine.
    fn search_commands(&self, engine: &SearchEngine, query: &str) -> Result<Vec<Command>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name, cmd_type, path, code, file_mtime, created_at FROM commands")
            .map_err(|e| LscmdError::Database(e.to_string()))?;

        let command_iter = stmt.query_map([], |row| {
            Ok(Command {
                name: row.get(0)?,
                cmd_type: row.get(1)?,
                path: row.get(2)?,
                code: row.get(3)?,
                file_mtime: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| LscmdError::Database(e.to_string()))?;

        let all_commands: Vec<Command> = command_iter
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| LscmdError::Database(e.to_string()))?;

        // The search engine returns references, so we clone them to return owned Commands.
        let results = engine.search(query, &all_commands)?.into_iter().cloned().collect();
        Ok(results)
    }

    /// Retrieves a command by its exact name (case-insensitive).
    fn get_command_by_name(&self, name: &str) -> Result<Option<Command>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name, cmd_type, path, code, file_mtime, created_at FROM commands WHERE LOWER(name) = LOWER(?1)")
            .map_err(|e| LscmdError::Database(e.to_string()))?;

        let mut rows = stmt.query_map([name], |row| {
            Ok(Command {
                name: row.get(0)?,
                cmd_type: row.get(1)?,
                path: row.get(2)?,
                code: row.get(3)?,
                file_mtime: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| LscmdError::Database(e.to_string()))?;

        match rows.next() {
            Some(Ok(command)) => Ok(Some(command)),
            Some(Err(e)) => Err(LscmdError::Database(e.to_string())),
            None => Ok(None),
        }
    }

    /// Deletes all commands from a given file path.
    fn delete_by_path(&self, path: &str) -> Result<u64> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM commands WHERE path = ?1", [path])
            .map_err(|e| LscmdError::Database(e.to_string()))?;
        Ok(affected as u64)
    }

    /// Clears the entire commands table.
    fn clear_all(&self) -> Result<u64> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM commands", [])
            .map_err(|e| LscmdError::Database(e.to_string()))?;
        Ok(affected as u64)
    }

    /// Gets the most recent modification time for a given file path.
    fn get_file_mtime(&self, path: &str) -> Result<Option<i64>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT MAX(file_mtime) as max_mtime FROM commands WHERE path = ?1")
            .map_err(|e| LscmdError::Database(e.to_string()))?;

        let result: Option<i64> = stmt.query_row([path], |row| {
            Ok(row.get::<_, Option<i64>>(0)?)
        })
        .map_err(|e| LscmdError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Pings the database to check for a live connection.
    fn health_check(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.prepare("SELECT 1")
            .map_err(|e| LscmdError::Database(e.to_string()))?;
        Ok(())
    }
}