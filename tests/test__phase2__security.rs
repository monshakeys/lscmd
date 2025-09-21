//! Tests for security-related components from Phase 2.4.

use lscmd::error::Result;
use lscmd::security::{InputValidator, QuoteParser};
use std::path::Path;

// --- InputValidator Tests ---

#[test]
fn test_validator_valid_inputs() -> Result<()> {
    InputValidator::validate_file_path(Path::new("/home/user/file.sh"))?;
    InputValidator::validate_command_name("my-command_1")?;
    InputValidator::validate_search_query("some query")?;
    Ok(())
}

#[test]
fn test_validator_path_traversal() {
    assert!(InputValidator::validate_file_path(Path::new("../secret.txt")).is_err());
    assert!(InputValidator::validate_file_path(Path::new("/home/user/../../etc/passwd")).is_err());
}

#[test]
fn test_validator_command_name_too_long() {
    let long_name = "a".repeat(300);
    assert!(InputValidator::validate_command_name(&long_name).is_err());
}

#[test]
fn test_validator_command_name_control_chars() {
    assert!(InputValidator::validate_command_name("my\ncommand").is_err());
    assert!(InputValidator::validate_command_name("my\tcmd").is_err());
}

#[test]
fn test_validator_search_query_too_long() {
    let long_query = "q".repeat(2000);
    assert!(InputValidator::validate_search_query(&long_query).is_err());
}

// --- QuoteParser Tests ---

#[test]
fn test_quote_parser_extract_content() -> Result<()> {
    let parser = QuoteParser::new();

    // Double quotes
    let line1 = "alias gco=\"git checkout\" # comment";
    assert_eq!(
        parser.extract_quoted_content(line1)?,
        Some("git checkout".to_string())
    );

    // Single quotes
    let line2 = "alias ls='ls -la'";
    assert_eq!(
        parser.extract_quoted_content(line2)?,
        Some("ls -la".to_string())
    );

    // No quotes
    let line3 = "not_an_alias";
    assert_eq!(parser.extract_quoted_content(line3)?, None);

    // Unterminated quotes
    let line4 = "alias bad=\"something";
    assert_eq!(parser.extract_quoted_content(line4)?, None);

    // Mixed quotes (finds first pair)
    let line5 = "alias x='echo \"hello\"'";
    assert_eq!(
        parser.extract_quoted_content(line5)?,
        Some("echo \"hello\"".to_string())
    );

    Ok(())
}
