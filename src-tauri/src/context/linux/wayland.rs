// src-tauri/src/context/linux/wayland.r// src-tauri/src/context/linux/wayland.rs
//! Wayland-native application context detection.
//!
//! This module retrieves the active window's title and application class on
//! Linux by interfacing with the [`active-win-pos-rs`](https://crates.io/crates/active-win-pos-rs) crate.
//!
//! ## Security & Performance Notes
//!
//! - This relies on the underlying mechanisms of `active-win-pos-rs`, which abstract
//!   away the compositor-specific APIs used to fetch window context.
//! - Fails gracefully: Returns [`AppInfo::unknown()`] on any error (e.g., unsupported
//!   compositor, missing permissions, or invalid data).
//!
//! ## Limitations
//!
//! - Wayland support across different Linux desktop environments (GNOME, KDE, wlroots)
//!   can be highly fragmented. Success depends heavily on the compatibility of the
//!   `active-win-pos-rs` crate with the user's specific compositor.
//! - Application class names are raw identifiers and are normalized via [`normalize_app_class`].

use crate::context::{app_info::AppInfo, normalize_app_name::normalize_app_class};
use active_win_pos_rs::get_active_window;
use tracing::error;

/// Retrieves metadata about the currently focused application.
///
/// This function:
/// 1. Queries the active window using `active-win-pos-rs`.
/// 2. Extracts the window's title and raw application name.
/// 3. Normalizes the application class name for consistent display.
/// 4. Validates that the returned title and class are meaningful.
///
/// On any failure (unsupported environment, missing window, empty response), returns
/// a placeholder [`AppInfo`] representing an unknown state.
pub fn get_active_app_info_linux_wayland() -> AppInfo {
    match get_active_window() {
        Ok(active_window) => {
            let window_title = active_window.title;
            let app_class = normalize_app_class(&active_window.app_name);

            // Validate that we didn't just get empty strings back
            if is_invalid_title_and_class(&window_title, &app_class) {
                return AppInfo {
                    window_title: "Unknown".to_string(),
                    app_class: "Unknown".to_string(),
                };
            }

            AppInfo {
                window_title,
                app_class,
            }
        }
        Err(e) => {
            error!("Failed to get active window via active-win-pos-rs: {:?}", e);

            AppInfo {
                window_title: "Unknown".to_string(),
                app_class: "Unknown".to_string(),
            }
        }
    }
}

/// Validates that both window title and app class are non-empty.
///
/// Prevents storing meaningless context like `("", "Unknown")`. Uses `.trim()`
/// to ensure strings with only whitespace are also caught as invalid.
fn is_invalid_title_and_class(title: &str, class: &str) -> bool {
    title.trim().is_empty() || class.trim().is_empty()
}
