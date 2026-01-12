// src-tauri/src/context/windows.rs
//! Windows-specific active application context detection.
//!
//! Uses the Win32 API to retrieve:
//! - The title of the foreground window.
//! - The executable name of the process owning that window.
//!
//! All operations are safe-fallback: any error results in [`AppInfo::unknown()`].

use crate::context::app_info::AppInfo;

pub mod win32;

/// Retrieves active application context on Windows.
///
/// Delegates to the Win32 implementation.
pub fn get_active_app_info_windows() -> AppInfo {
    win32::get_active_app_info_windows_win32()
}
