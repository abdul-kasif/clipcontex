// src-sauri/src/context.rs
use std::process::Command;
use tracing::{info, error, warn}; // Added warn

use crate::context::{app_info::AppInfo, project::normalize_app_class};

/// Main entry: detects environment and picks the right backend.
/// This function should be called as quickly as possible after a clipboard change is detected.
pub fn get_active_app_info() -> AppInfo {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    // Log only in debug builds or if needed for troubleshooting
    info!("Detected session type: {}, desktop: {}", session_type, desktop_env);

    if session_type.eq_ignore_ascii_case("wayland") {
        return get_active_app_info_wayland();
    }

    get_active_app_info_x11()
}

/// Detect active window on Wayland using kdotool (DE-agnostic)
fn get_active_app_info_wayland() -> AppInfo {
    info!("Using kdotool for Wayland active window detection");

    if !is_kdotool_installed() {
        error!("kdotool is not installed. Please install it: https://github.com/adi1090x/kdotool  ");
        return AppInfo::unknown();
    }

    // Get active window ID - This is the critical call that must happen immediately after the clipboard change
    let win_id_output = Command::new("kdotool")
        .args(&["getactivewindow"])
        .output();

    let win_id = match win_id_output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                info!("kdotool getactivewindow failed or returned non-zero status");
                return AppInfo::unknown();
            }
        }
        Err(e) => {
            info!("kdotool getactivewindow command failed: {}", e);
            return AppInfo::unknown();
        }
    };

    if win_id.is_empty() || win_id == "0" {
        info!("No active window detected via kdotool (ID: {})", win_id);
        return AppInfo::unknown();
    }

    // Get window title
    let title_output = Command::new("kdotool")
        .args(&["getwindowname", &win_id])
        .output();

    let title = match title_output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                warn!("kdotool getwindowname failed for ID {}, using 'Unknown'", win_id);
                "Unknown".to_string()
            }
        }
        Err(e) => {
            warn!("kdotool getwindowname command failed for ID {}: {}", win_id, e);
            "Unknown".to_string()
        }
    };

    // Get window class
    let class_output = Command::new("kdotool")
        .args(&["getwindowclassname", &win_id])
        .output();

    let class = match class_output {
        Ok(output) => {
            if output.status.success() {
                let raw_class = String::from_utf8_lossy(&output.stdout).trim().to_string();
                normalize_app_class(&raw_class)
            } else {
                warn!("kdotool getwindowclassname failed for ID {}, using 'unknown'", win_id);
                "unknown".to_string()
            }
        }
        Err(e) => {
            warn!("kdotool getwindowclassname command failed for ID {}: {}", win_id, e);
            "unknown".to_string()
        }
    };

    let title = if title.is_empty() { "Unknown".into() } else { title };
    let class = if class.is_empty() { "unknown".into() } else { class };

    info!("Active window (Wayland): class='{}', title='{}'", class, title);
    AppInfo { window_title: title, app_class: class }
}


/// Check if kdotool is installed
fn is_kdotool_installed() -> bool {
    Command::new("kdotool")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// X11 implementation remains the same for now
fn get_active_app_info_x11() -> AppInfo {
    info!("Using X11 xprop fallback");

    let window_id_output = Command::new("xprop")
        .args(&["-root", "_NET_ACTIVE_WINDOW"])
        .output();

    let window_id = window_id_output.ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|s| s.split_whitespace().last().map(|v| v.to_string()))
        .unwrap_or_default();

    if window_id.is_empty() || window_id == "0x0" {
        info!("No active window found via xprop");
        return AppInfo::unknown();
    }

    let props = Command::new("xprop")
        .args(&["-id", &window_id, "WM_CLASS", "WM_NAME"])
        .output();

    if let Ok(out) = props {
        let text = String::from_utf8_lossy(&out.stdout);
        return parse_xprop_output(&text);
    }

    AppInfo::unknown()
}

pub fn parse_xprop_output(output: &str) -> AppInfo {
    let mut title = "Unknown".to_string();
    let mut class = "unknown".to_string();

    for line in output.lines() {
        if line.starts_with("WM_NAME") {
            if let Some(v) = line.splitn(2, " = ").nth(1) {
                title = v.trim().trim_matches('"').to_string();
            }
        } else if line.starts_with("WM_CLASS") {
            if let Some(v) = line.splitn(2, " = ").nth(1) {
                let parts: Vec<_> = v.split(',').collect();
                if let Some(c) = parts.last() {
                    class = c.trim().trim_matches('"').to_lowercase();
                }
            }
        }
    }

    AppInfo { window_title: title, app_class: class }
}