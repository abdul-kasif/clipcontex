use std::process::Command;
use tracing::{debug, info, warn};

use crate::context::app_info::AppInfo;

/// Main entry: automatically detects environment and picks the right backend.
pub fn get_active_app_info() -> AppInfo {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    info!("🔍 Detected session type: {}", session_type);
    info!("🖥 Detected desktop environment: {}", desktop_env);

    if session_type.eq_ignore_ascii_case("wayland") {
        if desktop_env.contains("KDE") {
            return get_active_app_info_wayland_kde();
        } else if desktop_env.contains("GNOME") {
            return get_active_app_info_wayland_gnome();
        } else {
            warn!("Unknown Wayland desktop — falling back to X11");
            return get_active_app_info_x11();
        }
    }

    get_active_app_info_x11()
}

fn get_active_app_info_wayland_kde() -> AppInfo {
    info!("🪟 Detected KDE Wayland → trying qdbus / gdbus methods");

    // Try Plasma 6 (modern scripting interface)
    let js_script = r#"
        var client = workspace.activeClient;
        if (client) {
            print(JSON.stringify({
                title: client.caption,
                wm_class: client.resourceClass
            }));
        } else {
            print("{}");
        }
    "#;

    // Prefer qdbus, fallback to gdbus
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "if command -v qdbus >/dev/null; then \
                qdbus org.kde.KWin /Scripting org.kde.kwin.Scripting.evaluateScript '{}' 2>/dev/null; \
             elif command -v gdbus >/dev/null; then \
                gdbus call --session --dest org.kde.KWin --object-path /Scripting \
                          --method org.kde.kwin.Scripting.evaluateScript '{}' 2>/dev/null; \
             else \
                echo 'NO_DBUS'; \
             fi",
            js_script, js_script
        ))
        .output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);

        debug!("qdbus/gdbus Plasma6 stdout: {}", stdout);
        debug!("qdbus/gdbus Plasma6 stderr: {}", stderr);

        if stdout.contains('{') {
            if let Some(start) = stdout.find('{') {
                let json_str = stdout[start..].trim();
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                    let title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
                    let class = parsed.get("wm_class").and_then(|v| v.as_str()).unwrap_or("unknown").to_lowercase();
                    info!("✅ Active app (Plasma6): class='{}', title='{}'", class, title);
                    return AppInfo { window_title: title, app_class: class };
                }
            }
        }
    }

    warn!("❌ Plasma6 API failed — trying legacy Plasma5 method");

    // Try Plasma 5 legacy interface
    let win_path = run_dbus_cmd("qdbus org.kde.KWin /KWin org.kde.KWin.activeWindow")
        .or_else(|| run_dbus_cmd("gdbus call --session --dest org.kde.KWin --object-path /KWin --method org.kde.KWin.activeWindow"))
        .unwrap_or_default();

    if win_path.is_empty() || win_path.contains("Error") {
        warn!("⚠️ No valid active window via legacy KWin — falling back to X11");
        return get_active_app_info_x11();
    }

    let title = run_dbus_prop(&win_path, "org.kde.KWin.Window.caption", "Unknown");
    let class = run_dbus_prop(&win_path, "org.kde.KWin.Window.resourceClass", "unknown");

    info!("✅ Active app (Plasma5): class='{}', title='{}'", class, title);
    AppInfo { window_title: title, app_class: class }
}

fn get_active_app_info_wayland_gnome() -> AppInfo {
    info!("🧠 Detected GNOME Wayland → using gdbus");

    let output = Command::new("bash")
        .arg("-c")
        .arg("gdbus call --session --dest org.gnome.Shell --object-path /org/gnome/Shell \
              --method org.gnome.Shell.Eval 'global.display.focus_window && global.display.focus_window.title'")
        .output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        debug!("gdbus GNOME stdout: {}", stdout);

        if stdout.contains("Some") {
            let title = stdout
                .split('"')
                .nth(1)
                .unwrap_or("Unknown")
                .to_string();
            return AppInfo {
                window_title: title.clone(),
                app_class: "gnome".into(),
            };
        }
    }

    warn!("❌ GNOME gdbus detection failed — falling back to X11");
    get_active_app_info_x11()
}

/// Run xprop-based X11 fallback.
fn get_active_app_info_x11() -> AppInfo {
    info!("💠 Using X11 xprop fallback");

    let window_id_output = Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output();

    let window_id = window_id_output.ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|s| s.split_whitespace().last().map(|v| v.to_string()))
        .unwrap_or_default();

    if window_id.is_empty() || window_id == "0x0" {
        warn!("No active window found via xprop.");
        return AppInfo::unknown();
    }

    let props = Command::new("xprop")
        .args(["-id", &window_id, "WM_CLASS", "WM_NAME"])
        .output();

    if let Ok(out) = props {
        let text = String::from_utf8_lossy(&out.stdout);
        return parse_xprop_output(&text);
    }

    AppInfo::unknown()
}

/// Helper: parse WM_CLASS and WM_NAME for X11 fallback.
pub fn parse_xprop_output(output: &str) -> AppInfo {
    let mut title = "Unknown".to_string();
    let mut class = "unknown".to_string();

    for line in output.lines() {
        if line.starts_with("WM_NAME") {
            if let Some(v) = line.split(" = ").nth(1) {
                title = v.trim().trim_matches('"').to_string();
            }
        } else if line.starts_with("WM_CLASS") {
            if let Some(v) = line.split(" = ").nth(1) {
                let parts: Vec<_> = v.split(',').collect();
                if let Some(c) = parts.last() {
                    class = c.trim().trim_matches('"').to_lowercase();
                }
            }
        }
    }

    AppInfo { window_title: title, app_class: class }
}

/// Helper: run a DBus command and get trimmed stdout.
fn run_dbus_cmd(cmd: &str) -> Option<String> {
    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Helper: query KDE window properties (caption/class)
fn run_dbus_prop(path: &str, prop: &str, default: &str) -> String {
    let cmd = format!("qdbus org.kde.KWin {} {}", path, prop);
    run_dbus_cmd(&cmd)
        .filter(|v| !v.is_empty() && !v.contains("Error"))
        .unwrap_or(default.to_string())
}
