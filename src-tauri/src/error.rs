use serde::{Serialize, Serializer};
use std::fmt;

/// Custom error type for the application
#[derive(Debug)]
pub enum AppError {
    /// Backend API errors
    Backend(String),
    /// Network-related errors
    Network(String),
    /// Authentication errors
    Auth(String),
    /// Validation errors
    Validation(String),
    /// Resource not found
    NotFound(String),
    /// Internal errors
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Backend(msg) => write!(f, "Backend error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<tauri_plugin_http::reqwest::Error> for AppError {
    fn from(err: tauri_plugin_http::reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::Network("Request timeout".to_string())
        } else if err.is_connect() {
            AppError::Network("Connection failed".to_string())
        } else {
            AppError::Network(err.to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<tauri_plugin_musickit::Error> for AppError {
    fn from(err: tauri_plugin_musickit::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// Alias for Result with AppError
pub type AppResult<T> = Result<T, AppError>;
