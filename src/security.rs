use crate::error::{LscmdError, Result};
use std::path::{Component, Path};

const MAX_COMMAND_NAME_LEN: usize = 256;
const MAX_SEARCH_QUERY_LEN: usize = 1024;

/// Provides validation for various user inputs to prevent security vulnerabilities.
pub struct InputValidator;

impl InputValidator {
    /// Validates a file path to prevent directory traversal attacks.
    pub fn validate_file_path(path: &Path) -> Result<()> {
        for component in path.components() {
            match component {
                Component::ParentDir => {
                    return Err(LscmdError::Validation(
                        "Path traversal detected: '..' is not allowed".to_string(),
                    ));
                }
                Component::RootDir | Component::CurDir | Component::Normal(_) => {}
                Component::Prefix(prefix) => {
                    if prefix.as_os_str().to_string_lossy().contains("\\") {
                        return Err(LscmdError::Validation(
                            "UNC paths are not supported".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Validates the name of a command.
    pub fn validate_command_name(name: &str) -> Result<()> {
        if name.len() > MAX_COMMAND_NAME_LEN {
            return Err(LscmdError::Validation(format!(
                "Command name exceeds maximum length of {}",
                MAX_COMMAND_NAME_LEN
            )));
        }
        if name.chars().any(|c| c.is_control()) {
            return Err(LscmdError::Validation(
                "Command name contains invalid control characters".to_string(),
            ));
        }
        Ok(())
    }

    /// Validates a search query to prevent overly complex or long queries.
    pub fn validate_search_query(query: &str) -> Result<()> {
        if query.len() > MAX_SEARCH_QUERY_LEN {
            return Err(LscmdError::Validation(format!(
                "Search query exceeds maximum length of {}",
                MAX_SEARCH_QUERY_LEN
            )));
        }
        Ok(())
    }
}

/// A parser for safely handling shell-style quoted strings.
#[derive(Default)]
pub struct QuoteParser;

impl QuoteParser {
    pub fn new() -> Self {
        Self
    }

    /// Extracts the content from the first pair of single or double quotes.
    ///
    /// This function is designed to safely extract the content of an `alias`
    /// command, removing the outer quotes.
    ///
    /// # Example
    /// `alias my_alias="echo hello"` -> `echo hello`
    pub fn extract_quoted_content(&self, line: &str) -> Result<Option<String>> {
        let first_quote = match line.find('"') {
            Some(dq) => {
                let sq = line.find('\'');
                if let Some(sq_pos) = sq {
                    if dq < sq_pos { Some(('"', dq)) } else { Some(('\'', sq_pos)) }
                } else {
                    Some(('"', dq))
                }
            }
            None => line.find('\'').map(|sq_pos| ('\'', sq_pos)),
        };

        if let Some((quote_char, start)) = first_quote {
            if let Some(end) = line[start + 1..].find(quote_char) {
                let end_pos = start + 1 + end;
                return Ok(Some(line[start + 1..end_pos].to_string()));
            }
        }

        Ok(None)
    }
}
