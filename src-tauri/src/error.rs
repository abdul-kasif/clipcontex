// src-tauri/src/error.rs
//! Application-specific error types.
//!
//! This module defines a unified error enum [`AppError`] that categorizes failures
//! by subsystem (storage, config, clipboard, etc.). It enables consistent error
//! handling and user-friendly logging across the application.

// ===== Imports =====

use std::fmt::Display;

// ===== Domain Types =====

/// A categorized error type representing failures in different layers of the application.
///
/// Each variant corresponds to a major subsystem:
/// - `Storage`: Database or persistence issues (e.g., SQLite errors).
/// - `Config`: Problems loading or saving user settings.
/// - `Core`: Application lifecycle or setup failures.
/// - `Shortcut`: Invalid or unregistrable global hotkeys.
/// - `Clipboard`: Issues interacting with the system clipboard.
///
/// This design avoids generic strings and enables structured logging or UI feedback
/// based on error origin.
#[derive(Debug)]
pub enum AppError {
    /// An error originating from the storage layer (e.g., SQLite operations).
    Storage(String),
    /// An error related to user configuration (loading, parsing, or saving).
    Config(String),
    /// A failure in core application logic (e.g., window creation, setup).
    Core(String),
    /// An issue with global shortcut registration or parsing.
    Shortcut(String),
    /// A problem accessing or modifying the system clipboard.
    Clipboard(String),
}

// ===== Implementations =====

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Storage(e) => write!(f, "Storage Error: {}", e),
            AppError::Config(e) => write!(f, "Config Error: {}", e),
            AppError::Core(e) => write!(f, "Core Error: {}", e),
            AppError::Shortcut(e) => write!(f, "Shortcut Error: {}", e),
            AppError::Clipboard(e) => write!(f, "Clipboard Error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
