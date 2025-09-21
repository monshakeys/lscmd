pub mod operations;
pub mod schema;

use crate::error::{LscmdError, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a command (alias or function) found in a shell script.
/// This struct is mapped directly to the `commands` table in the database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Command {
    /// The name of the alias or function (PRIMARY KEY).
    pub name: String,
    /// The type of the command, either 'alias' or 'function'.
    #[sqlx(rename = "type")]
    pub cmd_type: String,
    /// The absolute path to the file where the command is defined.
    pub path: String,
    /// The full code/body of the command.
    pub code: String,
    /// The last modification time of the source file (Unix timestamp).
    pub file_mtime: i64,
    /// The timestamp when the command was first added to the database.
    /// This is handled by the database DEFAULT trigger.
    pub created_at: Option<i64>,
}

impl Command {
    /// Validates the command's fields.
    ///
    /// # Errors
    ///
    /// Returns `LscmdError::Validation` if any field fails the validation rules.
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() || self.name.len() > 256 {
            return Err(LscmdError::Validation(format!(
                "Invalid command name length for: {}",
                self.name
            )));
        }
        if self.cmd_type != "alias" && self.cmd_type != "function" {
            return Err(LscmdError::Validation(format!(
                "Invalid command type: {}",
                self.cmd_type
            )));
        }
        if self.path.is_empty() {
            return Err(LscmdError::Validation(
                "Command path cannot be empty".to_string(),
            ));
        }
        if self.code.is_empty() {
            return Err(LscmdError::Validation(
                "Command code cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
