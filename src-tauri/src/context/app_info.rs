// src-tauri/src/context/app_info.rs
//! Active application context representation.

#[cfg(target_os = "linux")]
use crate::context::linux;

#[cfg(target_os = "windows")]
use crate::context::windows;

/// Represents the active application context at the time of clipboard capture.
#[derive(Debug, Clone, PartialEq)]
pub struct AppInfo {
    /// Title of the active window (e.g., "main.rs - my-project").
    pub window_title: String,
    /// Normalized application class/name (e.g., "Visual Studio Code").
    pub app_class: String,
}

impl AppInfo {
    /// Returns a placeholder `AppInfo` when detection fails.
    pub fn unknown() -> Self {
        Self {
            window_title: "Unknown".to_string(),
            app_class: "Unknown".to_string(),
        }
    }

    /// Returns the unknown app class string.
    pub fn unknown_app_class() -> String {
        "Unknown".to_string()
    }

    /// Returns the unknown window title string.
    pub fn unknown_window_title() -> String {
        "Unknown".to_string()
    }
}

/// Retrieves the active application context based on the current OS.
///
/// # Platform Support
///
/// - **Linux**: Requires `kdotool` (Wayland) or `xdotool`/`xprop` (X11).
/// - **Windows**: Uses native Win32 APIs.
/// - **macOS**: Not implemented (returns `unknown`).
///
/// On failure, returns [`AppInfo::unknown()`].
pub fn get_active_app_info() -> AppInfo {
    #[cfg(target_os = "linux")]
    {
        linux::get_active_app_info_linux()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_active_app_info_windows()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        AppInfo::unknown()
    }
}

