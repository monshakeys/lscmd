use thiserror::Error;

#[derive(Error, Debug)]
pub enum LscmdError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error in {file}:{line} - {message}")]
    Parse { 
        file: String, 
        line: usize, 
        message: String 
    },
    
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("XDG directory error: {0}")]
    XdgError(String),
    
    #[error("Search pattern error: {0}")]
    SearchPattern(String),
    
    #[error("Data integrity error: {0}")]
    DataIntegrity(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<rusqlite::Error> for LscmdError {
    fn from(err: rusqlite::Error) -> Self {
        LscmdError::Database(err.to_string())
    }
}

impl From<regex::Error> for LscmdError {
    fn from(err: regex::Error) -> Self {
        LscmdError::SearchPattern(err.to_string())
    }
}

impl From<serde_json::Error> for LscmdError {
    fn from(err: serde_json::Error) -> Self {
        LscmdError::Config(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LscmdError>;