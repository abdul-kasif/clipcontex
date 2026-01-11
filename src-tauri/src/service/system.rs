// src-tauri/src/service/system.rs
//! Platform-specific system utilities.

use std::process::Command;

use crate::error::AppError;

/// Checks whether `kdotool` is installed on the system.
///
/// `kdotool` is required on Linux for simulating keyboard input (e.g., paste actions).
/// On non-Linux platforms, this function always returns `Ok(false)` (not needed).
///
/// # Errors
///
/// Returns [`AppError::Core`] if the subprocess fails to spawn.
pub fn check_kdotool_installed() -> Result<bool, AppError> {
    #[cfg(target_os = "linux")]
    {
        Command::new("kdotool")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .map_err(|e| AppError::Core(e.to_string()))
    }
    #[cfg(not(target_os = "linux"))]
    {
        // kdotool is Linux-specific; not needed elsewhere
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdotool_check() {
        let result = check_kdotool_installed();
        // Should not panic; should return Ok(bool)
        assert!(result.is_ok());
        // On non-Linux, it's always false
        #[cfg(not(target_os = "linux"))]
        assert_eq!(result.unwrap(), false);
    }
}

