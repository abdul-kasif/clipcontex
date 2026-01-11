// src-tauri/src/context/linux/wayland.rs
//! Wayland-native application context detection using `kdotool`.
//!
//! This module retrieves the active window's title and application class on
//! Wayland compositors by interfacing with [`kdotool`](https://github.com/jinliu/kdotool),
//! a command-line utility that communicates with the KDE KWin compositor via D-Bus.
//!
//! ## Requirements
//!
//! - **Compositor**: Only works with **KWin** (KDE Plasma's compositor).
//!   Other Wayland compositors (e.g., Sway, GNOME Shell) are **not supported**.
//! - **Dependency**: `kdotool` must be installed and accessible in `$PATH`.
//!   Install via: `sudo apt install kdotool` (Debian/Ubuntu) or equivalent.
//!
//! ## Security & Performance Notes
//!
//! - Each call spawns a new `kdotool` process — **avoid frequent polling**.
//! - No sandboxing: `kdotool` requires D-Bus access to the session bus.
//! - Fails gracefully: Returns [`AppInfo::unknown()`] on any error (missing binary,
//!   permission denied, empty output, etc.).
//!
//! ## Limitations
//!
//! - Does not work in headless, nested, or non-KWin Wayland sessions.
//! - May return stale data if windows change rapidly between calls.
//! - Application class names are raw KWin identifiers; normalization is applied
//!   via [`normalize_app_class`].

use crate::{
    context::{app_info::AppInfo, normalize_app_name::normalize_app_class},
    service,
};
use std::process::Command;
use tracing::error;

/// Retrieves metadata about the currently focused application on KWin-based Wayland sessions.
///
/// This function:
/// 1. Verifies `kdotool` is installed.
/// 2. Fetches the active window ID.
/// 3. Queries the window's title and class name.
/// 4. Normalizes the class name for consistent display.
///
/// On any failure (missing tool, invalid window, empty response), returns
/// a placeholder [`AppInfo`] with `"Unknown"` fields.
///
/// # Platform Assumptions
///
/// - Running under a **KWin Wayland session**.
/// - User has permission to query window properties via D-Bus.
///
pub fn get_active_app_info_linux_wayland() -> AppInfo {
    // Early exit if kdotool is not available
    if let Err(e) = service::system::check_kdotool_installed() {
        error!("kdotool not available: {}", e);
        return AppInfo::unknown();
    }

    // Step 1: Get active window ID
    let window_id = match run_kdotool_command(&["getactivewindow"]) {
        Some(id) => {
            if id.is_empty() || id == "0" {
                return AppInfo::unknown();
            }
            id
        }
        None => return AppInfo::unknown(),
    };

    // Step 2: Get window title
    let window_title = run_kdotool_command(&["getwindowname", &window_id])
        .unwrap_or_else(|| "Unknown".to_string());

    // Step 3: Get and normalize application class
    let app_class = run_kdotool_command(&["getwindowclassname", &window_id])
        .map(|class| normalize_app_class(&class))
        .unwrap_or_else(|| "Unknown".to_string());

    // Step 4: Validate and return
    if is_invalid_title_and_class(&window_title, &app_class) {
        return AppInfo::unknown();
    }

    AppInfo {
        window_title,
        app_class,
    }
}

/// Executes a `kdotool` command and returns trimmed stdout on success.
///
/// Logs stderr on failure and returns `None`.
///
/// # Arguments
///
/// - `args`: Command-line arguments to pass to `kdotool` (e.g., `["getactivewindow"]`).
///
/// # Returns
///
/// - `Some(String)`: Trimmed stdout if exit status is 0.
/// - `None`: If process fails to spawn, exits non-zero, or output is not valid UTF-8.
fn run_kdotool_command(args: &[&str]) -> Option<String> {
    let output = Command::new("kdotool").args(args).output().ok()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("kdotool {:?} failed: {}", args, stderr);
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Validates that both window title and app class are non-empty.
///
/// Prevents storing meaningless context like `("", "Unknown")`.
fn is_invalid_title_and_class(title: &str, class: &str) -> bool {
    title.is_empty() || class.is_empty()
}
