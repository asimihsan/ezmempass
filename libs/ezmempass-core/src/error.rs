/*
 * Error types for the EzMemPass core library.
 */

use std::fmt;
use thiserror::Error;

/// Errors that can occur in the EzMemPass library
#[derive(Error, Debug)]
pub enum EzMemPassError {
    /// Error when generating a password
    #[error("Failed to generate password: {0}")]
    GenerationError(String),

    /// Error when validating a password
    #[error("Invalid password: {0}")]
    ValidationError(String),

    /// Error when calculating entropy
    #[error("Entropy calculation error: {0}")]
    EntropyError(String),

    /// Error related to language models
    #[error("Language model error: {0}")]
    ModelError(String),

    /// Error related to graph operations
    #[error("Graph error: {0}")]
    GraphError(String),

    /// Error when loading or saving data
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Type alias for Results with EzMemPassError
pub type Result<T> = std::result::Result<T, EzMemPassError>;

/// Log level for the library
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}
