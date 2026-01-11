// src-tauri/src/context/windows/win32.rs
//! Low-level Win32 API implementation for active window detection.
//!
//! This module uses raw Win32 calls to:
//! 1. Get the foreground window handle.
//! 2. Read its window title.
//! 3. Resolve the owning process's executable name.
//!
//! All unsafe blocks are contained and validated.
//! Errors are gracefully handled by returning placeholder values.

use crate::context::app_info::AppInfo;
use std::path::Path;
use windows::{
    core::*,
    Win32::{Foundation::*, System::Threading::*, UI::WindowsAndMessaging::*},
};

/// Retrieves active application info using Win32 APIs.
///
/// Returns [`AppInfo::unknown()`] if:
/// - No foreground window exists.
/// - Window title cannot be read.
/// - Process ID or executable path is inaccessible.
pub fn get_active_app_info_windows_win32() -> AppInfo {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return AppInfo::unknown();
        }

        let window_title = get_windows_window_title(hwnd);
        let app_class = get_windows_application_name(hwnd);

        AppInfo {
            window_title,
            app_class,
        }
    }
}

// ===== Helper Functions =====

/// Reads the window title from a HWND.
///
/// Uses a fixed-size buffer (512 UTF-16 chars). Truncation is not handled,
/// but this is sufficient for typical window titles.
unsafe fn get_windows_window_title(hwnd: HWND) -> String {
    let mut title_buffer = [0u16; 512];
    let title_len = GetWindowTextW(hwnd, &mut title_buffer);
    if title_len == 0 {
        return AppInfo::unknown_window_title();
    }

    String::from_utf16_lossy(&title_buffer[..title_len as usize])
}

/// Resolves the application executable name from a window handle.
///
/// Steps:
/// 1. Get process ID from window.
/// 2. Open process with query permissions.
/// 3. Query full image path.
/// 4. Extract filename stem (e.g., "Code" from "Code.exe").
///
/// Uses a 1024-char buffer for the path — sufficient for most cases.
/// Returns "unknown" on any failure.
unsafe fn get_windows_application_name(hwnd: HWND) -> String {
    let mut process_id: u32 = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));

    let process_handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
        Ok(handle) => handle,
        Err(_) => return AppInfo::unknown_app_class(),
    };

    let mut class_buffer = [0u16; 1024];
    let mut class_len = class_buffer.len() as u32;
    let result = QueryFullProcessImageNameW(
        process_handle,
        PROCESS_NAME_FORMAT(0),
        PWSTR(class_buffer.as_mut_ptr()),
        &mut class_len,
    );

    // Ensure handle is closed even if query fails
    let _ = CloseHandle(process_handle);

    if result.is_ok() {
        let full_class_path = String::from_utf16_lossy(&class_buffer[..class_len as usize]);

        let raw_class_name = Path::new(&full_class_path)
            .file_name()
            .and_then(|r| r.to_str())
            .unwrap_or("unknown.exe")
            .to_string();

        Path::new(&raw_class_name)
            .file_stem()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown")
            .to_string()
    } else {
        AppInfo::unknown_app_class()
    }
}
