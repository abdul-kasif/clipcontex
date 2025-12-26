// ===== Imports =====
use std::collections::HashSet;

// ===== Public API =====
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
    tags.insert(format!("#{}", app));
}

// ===== Helper Function =====
fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|&needle| haystack.contains(needle))
}
