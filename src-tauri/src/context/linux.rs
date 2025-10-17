use std::process::Command;
use crate::context::app_info::AppInfo;

/// Gets the active window's title and class on X11 (Linux)
pub fn get_active_app_info() -> AppInfo {
    // Step 1: Get the active window ID
    let window_id_hex = Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|s| {
            // Parse line like: _NET_ACTIVE_WINDOW(WINDOW): 0x2e00007
            s.split_whitespace().last().map(|w| w.trim().to_string())
        });

    let window_id = if let Some(id) = window_id_hex {
        id
    } else {
        return AppInfo::unknown();
    };

    // Step 2: Try to get WM_CLASS and WM_NAME
    let xprop_output = Command::new("xprop")
        .args(["-id", &window_id, "WM_CLASS", "WM_NAME"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok());

    if let Some(output) = xprop_output {
        parse_xprop_output(&output)
    } else {
        AppInfo::unknown()
    }
}

/// Parse xprop output to AppInfo
pub fn parse_xprop_output(output: &str) -> AppInfo {
    let mut title = "Unknown".to_string();
    let mut class = "unknown".to_string();

    for line in output.lines() {
        if line.starts_with("WM_NAME(STRING)") || line.starts_with("WM_NAME(UTF8_STRING)") {
            if let Some((_, value)) = line.split_once(" = ") {
                let trimmed = value.trim().trim_matches('"');
                if !trimmed.is_empty() {
                    title = trimmed.to_string();
                }
            }
        } else if line.starts_with("WM_CLASS(STRING)") {
            if let Some((_, value)) = line.split_once(" = ") {
                // WM_CLASS can be: "instance", "class"
                let parts: Vec<&str> = value.split(',').collect();

                // Prefer the second part (the actual app class)
                if let Some(second) = parts.get(1) {
                    let trimmed = second.trim().trim_matches('"');
                    if !trimmed.is_empty() {
                        class = trimmed.to_lowercase();
                    }
                } else if let Some(first) = parts.get(0) {
                    let trimmed = first.trim().trim_matches('"');
                    if !trimmed.is_empty() {
                        class = trimmed.to_lowercase();
                    }
                }
            }
        }
    }

    AppInfo {
        window_title: title,
        app_class: class,
    }
}
