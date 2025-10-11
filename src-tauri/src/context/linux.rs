use std::process::Command;

use super::app_info::AppInfo;

/// Gets the active window's title and class on Linux using `xprop`.
pub fn get_active_app_info() -> AppInfo {
    // Step 1: Get active window ID
    let output = Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok());

    let window_id_hex = if let Some(line) = output {
        // Parse: _NET_ACTIVE_WINDOW(WINDOW): 0x1200003
        if let Some(pos) = line.find("0x") {
            line[pos..].trim().to_string()
        } else {
            return AppInfo::unknown();
        }
    } else {
        return AppInfo::unknown();
    };

    // Step 2: Get window title and class
    let output = Command::new("xprop")
        .args(["-id", &window_id_hex, "WM_NAME", "WM_CLASS"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok());

    if let Some(output) = output {
        parse_xprop_output(&output)
    } else {
        AppInfo::unknown()
    }
}

/// Parses xprop output into AppInfo (pure function for testing).
pub fn parse_xprop_output(output: &str) -> AppInfo {
    let mut title = "Unknown".to_string();
    let mut class = "unknown".to_string();

    for line in output.lines() {
        if line.starts_with("WM_NAME(STRING)") || line.starts_with("WM_NAME(UTF8_STRING)") {
            if let Some(value) = line.split_once(" = ") {
                title = value.1.trim_matches('"').to_string();
            }
        } else if line.starts_with("WM_CLASS(STRING)") {
            if let Some(value) = line.split_once(" = ") {
                let parts: Vec<&str> = value.1.split(',').collect();
                if !parts.is_empty() {
                    class = parts[0].trim_matches('"').to_lowercase();
                }
            }
        }
    }

    AppInfo {
        window_title: title,
        app_class: class,
    }
}
