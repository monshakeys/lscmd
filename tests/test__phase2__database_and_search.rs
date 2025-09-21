//! Integration tests for Phase 2: Database and Search.

use lscmd::database::operations::{CommandRepository, SqliteCommandRepository};
use lscmd::database::Command;
use lscmd::error::Result;
use lscmd::search::SearchEngine;

// --- Test Setup ---

fn setup_in_memory_db() -> Result<SqliteCommandRepository> {
    use rusqlite::Connection;
    // Use in-memory database for tests
    let conn = Connection::open(":memory:").map_err(|e| lscmd::error::LscmdError::Database(e.to_string()))?;

    // Initialize schema
    conn.execute_batch(lscmd::database::schema::get_schema())
        .map_err(|e| lscmd::error::LscmdError::Database(e.to_string()))?;

    Ok(SqliteCommandRepository::new(conn))
}

fn get_mock_commands() -> Vec<Command> {
    vec![
        Command {
            name: "GitCommit".to_string(),
            cmd_type: "alias".to_string(),
            path: "/files/g.sh".to_string(),
            code: "git commit -m".to_string(),
            file_mtime: 100,
            created_at: None,
        },
        Command {
            name: "DockerRun".to_string(),
            cmd_type: "function".to_string(),
            path: "/files/d.sh".to_string(),
            code: "docker run --rm -it".to_string(),
            file_mtime: 200,
            created_at: None,
        },
        Command {
            name: "rust_analyzer_status".to_string(),
            cmd_type: "function".to_string(),
            path: "/files/g.sh".to_string(), // Same path as GitCommit
            code: "echo 'Rust is running'".to_string(),
            file_mtime: 100,
            created_at: None,
        },
    ]
}

// --- Schema Tests ---

#[test]
fn test_phase2_schema_initialization() -> Result<()> {
    let repo = setup_in_memory_db()?;
    assert!(repo.health_check().is_ok());
    Ok(())
}

// --- SearchEngine Tests ---

#[test]
fn test_search_engine_logic() -> Result<()> {
    let commands = get_mock_commands();
    let engine_word = SearchEngine::new(false);
    let engine_regex = SearchEngine::new(true);

    // Word search, case-insensitive
    assert_eq!(engine_word.search("gitcommit", &commands)?.len(), 1);
    // Word search, OR logic
    assert_eq!(engine_word.search("git docker", &commands)?.len(), 2);
    // Regex search, case-insensitive
    assert_eq!(engine_regex.search("^git", &commands)?.len(), 1);
    // Empty query
    assert_eq!(engine_word.search("", &commands)?.len(), 3);
    // No results
    assert!(engine_word.search("nonexistent", &commands)?.is_empty());
    // Invalid regex
    assert!(engine_regex.search("git(", &commands).is_err());

    Ok(())
}

// --- CRUD and Repository Tests ---

#[test]
fn test_crud_insert_and_get() -> Result<()> {
    let repo = setup_in_memory_db()?;
    let mock_commands = get_mock_commands();

    // Insert one command
    repo.insert_command(&mock_commands[0])?;

    // Get it back (case-insensitively)
    let fetched = repo.get_command_by_name("gitcommit")?;
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().name, "GitCommit");

    // Get something that doesn't exist
    let fetched_none = repo.get_command_by_name("nonexistent")?;
    assert!(fetched_none.is_none());

    Ok(())
}

#[test]
fn test_crud_batch_insert() -> Result<()> {
    let repo = setup_in_memory_db()?;
    let mock_commands = get_mock_commands();

    repo.batch_insert(&mock_commands)?;

    let fetched = repo.get_command_by_name("DockerRun")?;
    assert!(fetched.is_some());
    let fetched = repo.get_command_by_name("rust_analyzer_status")?;
    assert!(fetched.is_some());

    Ok(())
}

#[test]
fn test_crud_delete_by_path_and_get_mtime() -> Result<()> {
    let repo = setup_in_memory_db()?;
    let mock_commands = get_mock_commands();
    repo.batch_insert(&mock_commands)?;

    // Check mtime for a path with multiple commands
    let mtime = repo.get_file_mtime("/files/g.sh")?;
    assert_eq!(mtime, Some(100));

    // Delete by path
    let deleted_count = repo.delete_by_path("/files/g.sh")?;
    assert_eq!(deleted_count, 2);

    // Verify they are gone
    let fetched = repo.get_command_by_name("GitCommit")?;
    assert!(fetched.is_none());
    let fetched = repo.get_command_by_name("rust_analyzer_status")?;
    assert!(fetched.is_none());

    // Verify other commands remain
    let fetched_docker = repo.get_command_by_name("DockerRun")?;
    assert!(fetched_docker.is_some());

    Ok(())
}

#[test]
fn test_crud_clear_all() -> Result<()> {
    let repo = setup_in_memory_db()?;
    let mock_commands = get_mock_commands();
    repo.batch_insert(&mock_commands)?;

    let cleared_count = repo.clear_all()?;
    assert_eq!(cleared_count, 3);

    let fetched = repo.get_command_by_name("DockerRun")?;
    assert!(fetched.is_none());

    Ok(())
}

#[test]
fn test_repo_search_integration() -> Result<()> {
    let repo = setup_in_memory_db()?;
    let mock_commands = get_mock_commands();
    repo.batch_insert(&mock_commands)?;

    // Search using the repo, which integrates the search engine
    let engine = SearchEngine::new(false); // Word mode
    let results = repo.search_commands(&engine, "RUST DOCKER")?;

    assert_eq!(results.len(), 2);
    let names: Vec<_> = results.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"DockerRun"));
    assert!(names.contains(&"rust_analyzer_status"));

    Ok(())
}