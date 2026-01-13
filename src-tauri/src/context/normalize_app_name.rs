// src-tauri/src/context/normalize_app_class.rs
//! Application class normalization for consistent display.

/// Normalizes raw application class strings into human-readable names.
///
/// Examples:
/// - `"org.kde.konsole"` → `"Konsole"`
/// - `"code-url-handler"` → `"Code Url Handler"`
/// - `"Chromium-browser"` → `"Chromium"`
///
/// Known applications are mapped via hardcoded overrides.
/// Other names are split on `-`, `_`, or `.`, then title-cased.
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

#[allow(unused_variables)]
fn known_app_overrides(app_class: &str) -> Option<&'static str> {
    #[cfg(target_os = "linux")]
    match app_class {
        "org.kde.konsole" => Some("Konsole"),
        "chromium-browser" | "chromium" => Some("Chromium"),
        "google-chrome" => Some("Google Chrome"),
        "libreoffice-writer" => Some("LibreOffice Writer"),
        "libreoffice-calc" => Some("LibreOffice Calc"),
        "libreoffice-impress" => Some("LibreOffice Impress"),
        "libreoffice-math" => Some("LibreOffice Math"),
        "org.kde.khelpcenter" => Some("Help Center"),
        "org.kde.plasma-systemmonitor" => Some("System Monitor"),
        _ => None,
    }

    #[cfg(target_os = "windows")]
    None
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_os = "linux")]
    #[test]
    fn test_normalize_known_apps() {
        assert_eq!(normalize_app_class("org.kde.konsole"), "Konsole");
        assert_eq!(normalize_app_class("chromium-browser"), "Chromium");
    }

    #[test]
    fn test_normalize_generic() {
        assert_eq!(normalize_app_class("my-app_v2"), "My App V2");
        assert_eq!(normalize_app_class("pdfviewer"), "Pdfviewer"); // note: no camelCase split
    }

    #[test]
    fn test_normalize_with_dots() {
        assert_eq!(normalize_app_class("com.google.Chrome"), "Chrome");
    }
}
