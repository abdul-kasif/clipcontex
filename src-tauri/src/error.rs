// ===== Imports =====
use std::fmt::Display;

// ===== Domain Types =====
#[derive(Debug)]
pub enum AppError {
    Storage(String),
    Config(String),
    Core(String),
    Shortcut(String),
    Clipboard(String),
}

// ===== Implementaion =====
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
