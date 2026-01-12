// =// src-tauri/src/context/linux.rs
//! Linux-specific application context detection.
//!
//! Detects session type (wayland) and delegates accordingly.

use crate::context::app_info::AppInfo;

pub mod wayland;

/// Detects active app info on Linux by session type.
pub fn get_active_app_info_linux() -> AppInfo {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();

    if session_type.eq_ignore_ascii_case("wayland") {
        wayland::get_active_app_info_linux_wayland()
    } else {
        AppInfo::unknown()
    }
}
