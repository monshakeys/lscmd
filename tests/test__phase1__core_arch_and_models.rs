// tests/phase1.rs

use lscmd::config::{Config, XdgPaths};
use lscmd::database::Command;
use lscmd::error::LscmdError;
use std::env;
use std::fs;
use std::path::PathBuf;

// Helper to create a temporary directory for tests
fn setup_test_env() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    let home = temp_dir.path().to_path_buf();
    let appdata = temp_dir.path().join("AppData/Roaming");
    fs::create_dir_all(&appdata).unwrap();

    env::set_var("HOME", &home);
    env::set_var("XDG_DATA_HOME", temp_dir.path().join(".local/share"));
    env::set_var("XDG_CONFIG_HOME", temp_dir.path().join(".config"));
    env::set_var("APPDATA", &appdata);
    
    (temp_dir, home, appdata)
}

#[test]
fn test_command_validation_success() {
    let cmd = Command {
        name: "my_func".to_string(),
        cmd_type: "function".to_string(),
        path: "/path/to/file.sh".to_string(),
        code: "echo hello".to_string(),
        file_mtime: 12345,
        created_at: None,
    };
    assert!(cmd.validate().is_ok());
}

#[test]
fn test_command_validation_failures() {
    // Empty name
    let mut cmd = Command {
        name: "".to_string(),
        cmd_type: "function".to_string(),
        path: "/path/to/file.sh".to_string(),
        code: "echo hello".to_string(),
        file_mtime: 12345,
        created_at: None,
    };
    assert!(matches!(cmd.validate(), Err(LscmdError::Validation(_))));

    // Invalid type
    cmd.name = "my_func".to_string();
    cmd.cmd_type = "invalid_type".to_string();
    assert!(matches!(cmd.validate(), Err(LscmdError::Validation(_))));

    // Empty path
    cmd.cmd_type = "alias".to_string();
    cmd.path = "".to_string();
    assert!(matches!(cmd.validate(), Err(LscmdError::Validation(_))));

    // Empty code
    cmd.path = "/path/to/file.sh".to_string();
    cmd.code = "".to_string();
    assert!(matches!(cmd.validate(), Err(LscmdError::Validation(_))));
}

#[test]
#[cfg(unix)]
fn test_xdg_paths_unix() {
    let _td = setup_test_env();
    let home = dirs::home_dir().unwrap();

    // Test default paths
    env::remove_var("XDG_DATA_HOME");
    env::remove_var("XDG_CONFIG_HOME");
    let paths = XdgPaths::new().unwrap();
    assert_eq!(paths.data_home, home.join(".local/share"));
    assert_eq!(paths.config_home, home.join(".config"));

    // Test custom paths
    env::set_var("XDG_DATA_HOME", home.join("custom_data"));
    env::set_var("XDG_CONFIG_HOME", home.join("custom_config"));
    let paths = XdgPaths::new().unwrap();
    assert_eq!(paths.data_home, home.join("custom_data"));
    assert_eq!(paths.config_home, home.join("custom_config"));
    
    assert!(paths.database_path().unwrap().ends_with("lscmd/commands.db"));
    assert!(paths.config_file_path().unwrap().ends_with("lscmd/config.json"));
}

#[test]
#[cfg(windows)]
fn test_xdg_paths_windows() {
    let (_td, _home, appdata) = setup_test_env();

    let paths = XdgPaths::new().unwrap();
    assert_eq!(paths.data_home, appdata);
    assert_eq!(paths.config_home, appdata);

    assert!(paths.database_path().unwrap().ends_with("lscmd/commands.db"));
    assert!(paths.config_file_path().unwrap().ends_with("lscmd/config.json"));
}

#[test]
fn test_config_load_and_save() {
    let _td = setup_test_env();
    
    // Test creating a default config
    let config = Config::load().unwrap();
    let home = dirs::home_dir().unwrap();
    assert_eq!(config.alias_path, home.join(".aliases"));

    // Test saving and loading
    let mut new_config = config;
    new_config.alias_path = PathBuf::from("/tmp/new_aliases");
    new_config.save().unwrap();

    let loaded_config = Config::load().unwrap();
    assert_eq!(loaded_config.alias_path, new_config.alias_path);
    assert_eq!(loaded_config.version, new_config.version);
}

#[test]
fn test_error_conversions() {
    // Test std::io::Error -> LscmdError
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let lscmd_err: LscmdError = io_error.into();
    assert!(matches!(lscmd_err, LscmdError::Io(_)));
    assert_eq!(lscmd_err.to_string(), "IO error: file not found");

    // Test serde_json::Error -> LscmdError
    let json_error: serde_json::Error = serde_json::from_str::<Command>("{}").unwrap_err();
    let lscmd_err: LscmdError = json_error.into();
    assert!(matches!(lscmd_err, LscmdError::Config(_)));
    assert!(lscmd_err.to_string().starts_with("Config error: missing field"));
}
