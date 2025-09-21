use crate::error::{LscmdError, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

/// Represents the XDG Base Directory paths.
/// This struct provides a cross-platform way to locate user-specific
/// data and configuration files.
#[derive(Debug, Clone)]
pub struct XdgPaths {
    pub data_home: PathBuf,
    pub config_home: PathBuf,
}

impl XdgPaths {
    /// Creates a new `XdgPaths` instance, automatically detecting the correct
    /// paths based on the operating system and environment variables.
    ///
    /// It follows the XDG Base Directory Specification.
    ///
    /// # Errors
    ///
    /// Returns `LscmdError::XdgError` if the home directory cannot be determined.
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().ok_or_else(|| LscmdError::XdgError("Home directory not found".to_string()))?;

        let data_home = if cfg!(unix) {
            env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join(".local/share"))
        } else if cfg!(windows) {
            env::var("APPDATA")
                .map(PathBuf::from)
                .map_err(|_| LscmdError::XdgError("APPDATA environment variable not found".to_string()))?
        } else {
            // Fallback for other OSes
            home_dir.join(".local/share")
        };

        let config_home = if cfg!(unix) {
            env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join(".config"))
        } else if cfg!(windows) {
            // On Windows, AppData is often used for both
            env::var("APPDATA")
                .map(PathBuf::from)
                .map_err(|_| LscmdError::XdgError("APPDATA environment variable not found".to_string()))?
        } else {
            // Fallback for other OSes
            home_dir.join(".config")
        };

        Ok(Self { data_home, config_home })
    }

    /// Returns the path to the lscmd-specific data directory.
    /// It ensures the directory exists.
    pub fn get_lscmd_data_dir(&self) -> Result<PathBuf> {
        let path = self.data_home.join("lscmd");
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    /// Returns the path to the lscmd-specific config directory.
    /// It ensures the directory exists.
    pub fn get_lscmd_config_dir(&self) -> Result<PathBuf> {
        let path = self.config_home.join("lscmd");
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    /// Returns the full path to the SQLite database file.
    pub fn database_path(&self) -> Result<PathBuf> {
        let data_dir = self.get_lscmd_data_dir()?;
        Ok(data_dir.join("commands.db"))
    }

    /// Returns the full path to the JSON configuration file.
    pub fn config_file_path(&self) -> Result<PathBuf> {
        let config_dir = self.get_lscmd_config_dir()?;
        Ok(config_dir.join("config.json"))
    }
}

/// Represents the application's configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub alias_path: PathBuf,
    pub version: String,
}

impl Config {
    /// Loads the configuration from the file specified by XDG paths.
    /// If the file doesn't exist, it creates a default configuration.
    pub fn load() -> Result<Self> {
        let xdg_paths = XdgPaths::new()?;
        let config_path = xdg_paths.config_file_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            // TODO: Implement version migration logic here
            Ok(config)
        } else {
            // Create a default configuration
            let home_dir = dirs::home_dir().ok_or_else(|| LscmdError::XdgError("Home directory not found".to_string()))?;
            let default_config = Config {
                alias_path: home_dir.join(".aliases"), // Default path
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Saves the current configuration to the file specified by XDG paths.
    pub fn save(&self) -> Result<()> {
        let xdg_paths = XdgPaths::new()?;
        let config_path = xdg_paths.config_file_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}
