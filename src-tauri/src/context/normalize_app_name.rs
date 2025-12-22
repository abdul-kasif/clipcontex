// ===== Public API =====
pub fn normalize_app_class(app_class: &str) -> String {
    if let Some(name) = known_app_overrides(app_class) {
        return name.to_string();
    }

    let stripped = app_class.rsplit('.').next().unwrap_or(app_class);

    let cleaned = stripped.replace(['-', '_'], " ");

    cleaned
        .split_whitespace()
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

// ===== Helper Functions =====
fn known_app_overrides(app_class: &str) -> Option<&'static str> {
    #[cfg(target_os = "linux")]
    match app_class {
        "org.kde.konsole" => Some("Konsole"),
        "chromium-browser" | "chromium" => Some("Chromium"),
        "libreoffice-math" => Some("LibreOffice Math"),
        "org.kde.khelpcenter" => Some("Help Center"),
        "org.kde.plasma-systemmonitor" => Some("System Monitor"),
        _ => None,
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
