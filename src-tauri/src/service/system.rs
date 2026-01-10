// ===== Imports =====
use std::process::Command;

// ===== Crates =====
use crate::error::AppError;

// ===== Public API =====
pub fn check_kdotool_installed() -> Result<bool, AppError> {
    #[cfg(target_os = "linux")]
    Command::new("kdotool")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .map_err(|e| AppError::Core(e.to_string()))
}
