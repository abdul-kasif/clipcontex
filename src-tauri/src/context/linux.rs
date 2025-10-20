use std::process::Command;
use tracing::{debug, error, info};

use crate::context::{app_info::AppInfo, project::normalize_app_class};

/// Main entry: detects environment and picks the right backend.
pub fn get_active_app_info() -> AppInfo {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    info!("Detected session type: {}", session_type);
    info!("Detected desktop environment: {}", desktop_env);

    if session_type.eq_ignore_ascii_case("wayland") {
        return get_active_app_info_wayland();
    }

    get_active_app_info_x11()
}

/// Detect active window on Wayland using kdotool (DE-agnostic)
fn get_active_app_info_wayland() -> AppInfo {
    info!("Using kdotool for Wayland active window detection");

    if !is_kdotool_installed() {
        error!("kdotool is not installed. Please install it: https://github.com/adi1090x/kdotool");
        return AppInfo::unknown();
    }

    // Get active window ID
    let win_id = run_cmd("kdotool", &["getactivewindow"]);
    if win_id.is_empty() || win_id == "0" {
        debug!("No active window detected via kdotool");
        return AppInfo::unknown();
    }

    // Get window title
    let title = run_cmd("kdotool", &["getwindowname", &win_id]);
    // Get window class
    let class = run_cmd("kdotool", &["getwindowclassname", &win_id]);

    let title = if title.is_empty() { "Unknown".into() } else { title };
    let class = normalize_app_class(&class);

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

/// Run a command and return trimmed stdout
fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// Run xprop-based X11 fallback
fn get_active_app_info_x11() -> AppInfo {
    debug!("Using X11 xprop fallback");

    let window_id_output = Command::new("xprop")
        .args(&["-root", "_NET_ACTIVE_WINDOW"])
        .output();

    let window_id = window_id_output.ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|s| s.split_whitespace().last().map(|v| v.to_string()))
        .unwrap_or_default();

    if window_id.is_empty() || window_id == "0x0" {
        debug!("No active window found via xprop");
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

/// Parse WM_CLASS and WM_NAME for X11 fallback
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
