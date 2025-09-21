use crate::database::Command;
use crate::error::{LscmdError, Result};
use regex::Regex;

/// Defines the search engine for finding commands.
///
/// The search behavior is determined by the `regex_mode` flag. All searches
/// are performed case-insensitively, as per the project's requirements.
#[derive(Debug)]
pub struct SearchEngine {
    regex_mode: bool,
}

impl SearchEngine {
    /// Creates a new `SearchEngine`.
    ///
    /// # Arguments
    ///
    /// * `regex_mode` - If true, the engine will use regular expressions for searching.
    ///                  Otherwise, it will perform a simple word-based search.
    pub fn new(regex_mode: bool) -> Self {
        Self { regex_mode }
    }

    /// Searches a slice of commands based on the configured mode.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string.
    /// * `commands` - A slice of `Command` structs to search through.
    ///
    /// # Returns
    ///
    /// A `Vec<Command>` containing the matched commands.
    ///
    /// # Errors
    ///
    /// Returns `LscmdError::SearchPattern` if the regex is invalid.
    pub fn search<'a>(&self, query: &str, commands: &'a [Command]) -> Result<Vec<&'a Command>> {
        if query.is_empty() {
            return Ok(commands.iter().collect());
        }

        match self.regex_mode {
            true => self.regex_search(query, commands),
            false => self.word_search(query, commands),
        }
    }

    /// Performs a case-insensitive regex search on command names and code.
    fn regex_search<'a>(&self, query: &str, commands: &'a [Command]) -> Result<Vec<&'a Command>> {
        // Prepend with `(?i)` for case-insensitivity.
        let pattern = format!("(?i){}", query);
        let re = Regex::new(&pattern)
            .map_err(|e| LscmdError::SearchPattern(format!("Invalid regex: {}", e)))?;

        let results = commands
            .iter()
            .filter(|cmd| re.is_match(&cmd.name) || re.is_match(&cmd.code))
            .collect();

        Ok(results)
    }

    /// Performs a case-insensitive word-based OR search.
    ///
    /// A command matches if its name or code contains any of the whitespace-separated
    /// words in the query.
    fn word_search<'a>(&self, query: &str, commands: &'a [Command]) -> Result<Vec<&'a Command>> {
        let words: Vec<String> = query.split_whitespace().map(|s| s.to_lowercase()).collect();
        if words.is_empty() {
            return Ok(commands.iter().collect());
        }

        let results = commands
            .iter()
            .filter(|cmd| {
                let name_lower = cmd.name.to_lowercase();
                let code_lower = cmd.code.to_lowercase();
                words
                    .iter()
                    .any(|word| name_lower.contains(word) || code_lower.contains(word))
            })
            .collect();

        Ok(results)
    }
}
