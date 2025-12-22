// ===== Imports =====
use crate::context::{normalize_app_class, AppInfo};
use std::process::Command;
use tracing::error;

// ===== Public API =====
pub fn get_active_app_info_linux_wayland() -> AppInfo {
    if !is_kdotool_installed() {
        error!("Kdotool is not installed.");
        return AppInfo::unknown();
    }

    let window_id = match run_kdotool_commands(&["getactivewindow"]) {
        Some(id) => {
            if id.is_empty() || id == "0" {
                return AppInfo::unknown();
            }
            id
        }
        None => return AppInfo::unknown(),
    };

    let window_title = match run_kdotool_commands(&["getwindowname", &window_id]) {
        Some(title) => title,
        None => "Unknown".to_string(),
    };

    let app_class = match run_kdotool_commands(&["getwindowclassname", &window_id]) {
        Some(class) => normalize_app_class(&class),
        None => "Unknown".to_string(),
    };

    if is_invalid_title_and_class(&window_title, &app_class) {
        return AppInfo::unknown();
    }

    AppInfo {
        window_title,
        app_class,
    }
}

// ===== Helper functions =====
fn is_kdotool_installed() -> bool {
    Command::new("kdotool")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_kdotool_commands(args: &[&str]) -> Option<String> {
    let output = Command::new("kdotool").args(args).output().ok()?;

    if !output.status.success() {
        error!("Kdotool {:?}, failed", args);
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn is_invalid_title_and_class(title: &str, class: &str) -> bool {
    title.is_empty() || class.is_empty()
}
