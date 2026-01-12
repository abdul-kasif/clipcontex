// src-tauri/src/context/auto_tags/app_based.rs
//! Application-based tag generation.
//!
//! Maps normalized application names to semantic categories.
//! Also adds a tag for the raw app name (e.g., `#vscode`).

use std::collections::HashSet;

/// Generates tags based on the application class.
///
/// Adds one category tag (e.g., `#editor`) and one app-specific tag (e.g., `#vscode`).
/// The app name is converted to lowercase and used as-is (assumes prior normalization).
pub fn generate_app_based_tags(app_class: &str, tags: &mut HashSet<String>) {
    let app = app_class.to_lowercase();
    let editors = [
        "code", "vscode", "vscodium", "editor", "neovim", "nvim", "helix",
    ];
    let terminals = [
        "konsole",
        "terminal",
        "alacritty",
        "wezterm",
        "kitty",
        "gnome-terminal",
        "xterm",
    ];
    let browsers = [
        "firefox",
        "chrome",
        "brave",
        "browser",
        "chromium",
        "opera",
        "vivaldi",
        "librewolf",
    ];
    let chat_apps = [
        "discord", "telegram", "slack", "signal", "element", "whatsapp",
    ];
    let document_apps = ["pdf", "evince", "okular", "zathura", "xreader", "qpdfview"];
    let file_managers = ["nautilus", "dolphin", "thunar", "pcmanfm", "files", "nemo"];
    let notes_apps = ["notion", "obsidian", "joplin", "logseq", "zettlr"];

    if contains_any(&app, &editors) {
        tags.insert("#editor".into());
    } else if contains_any(&app, &terminals) {
        tags.insert("#terminal".into());
    } else if contains_any(&app, &browsers) {
        tags.insert("#browser".into());
    } else if contains_any(&app, &chat_apps) {
        tags.insert("#chat".into());
    } else if contains_any(&app, &file_managers) {
        tags.insert("#file-manager".into());
    } else if contains_any(&app, &notes_apps) {
        tags.insert("#notes".into());
    } else if contains_any(&app, &document_apps) {
        tags.insert("#document".into());
    }

    // Add app-specific tag (e.g., #vscode)
    tags.insert(format!("#{}", app));
}

/// Checks if a string contains any of the given substrings.
fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|&needle| haystack.contains(needle))
}

// ===== Tests =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_based_tags() {
        let mut tags = HashSet::new();
        generate_app_based_tags("Visual Studio Code", &mut tags);
        let tags: Vec<_> = tags.into_iter().collect();
        assert!(tags.contains(&"#editor".to_string()));
        assert!(tags.contains(&"#visual studio code".to_string()));
    }

    #[test]
    fn test_unknown_app() {
        let mut tags = HashSet::new();
        generate_app_based_tags("MyCustomApp", &mut tags);
        assert_eq!(tags.len(), 1);
        assert!(tags.contains(&"#mycustomapp".to_string()));
    }
}
