//! Error types for mdbook-htmx

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during the build process.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Invalid frontmatter in a chapter file
    #[error("Invalid frontmatter in {path}: {source}")]
    InvalidFrontmatter {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    /// Template rendering error
    #[error("Template error in {template}: {source}")]
    TemplateError {
        template: String,
        #[source]
        source: tera::Error,
    },

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl BuildError {
    /// Get the exit code for this error type.
    ///
    /// Exit codes per ADR-0017:
    /// - 1: Content/template errors (recoverable)
    /// - 2: Configuration errors
    /// - 3: I/O errors
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidFrontmatter { .. } => 1,
            Self::TemplateError { .. } => 1,
            Self::ConfigError(_) => 2,
            Self::IoError(_) => 3,
            Self::JsonError(_) => 1,
        }
    }
}
