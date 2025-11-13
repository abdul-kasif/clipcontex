//! Auto-tag clipboard content based on heuristics and app context.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Generates auto-tags from clipboard content, project name, and app class.
pub fn generate_auto_tags(
    content: &str,
    project_name: Option<&str>,
    app_class: Option<&str>,
) -> Vec<String> {
    let mut tags = HashSet::new();
    let trimmed = content.trim();

    // --- Project Tag ---
    if let Some(name) = project_name {
        let sanitized = sanitize_tag(name);
        if !sanitized.is_empty() {
            tags.insert(format!("#{}", sanitized));
        }
    }

    // --- Content-Based Heuristics ---
    if is_code_like(trimmed) {
        tags.insert("#code".into());
    }
    if is_url(trimmed) {
        tags.insert("#url".into());
    }
    if is_email(trimmed) {
        tags.insert("#email".into());
    }
    if is_terminal_command(trimmed) {
        tags.insert("#terminal".into());
    }
    if is_file_path(trimmed) {
        tags.insert("#path".into());
    }
    if is_json_like(trimmed) {
        tags.insert("#json".into());
    }

    // --- Context-Based Tags (App Awareness) ---
    if let Some(app) = app_class {
        let app = app.to_lowercase();

        if app.contains("code") || app.contains("vscode") || app.contains("editor") || app.contains("vscodium") {
            tags.insert("#editor".into());
        } else if app.contains("konsole") || app.contains("terminal") || app.contains("alacritty") || app.contains("wezterm") || app.contains("kitty") {
            tags.insert("#terminal".into());
        } else if app.contains("firefox") || app.contains("chrome") || app.contains("brave") || app.contains("browser") || app.contains("chromium") {
            tags.insert("#browser".into());
        } else if app.contains("discord") || app.contains("telegram") || app.contains("slack") || app.contains("signal") {
            tags.insert("#chat".into());
        } else if app.contains("nautilus") || app.contains("dolphin") || app.contains("files") {
            tags.insert("#file-manager".into());
        } else if app.contains("notion") || app.contains("obsidian") {
            tags.insert("#notes".into());
        } else if app.contains("pdf") || app.contains("evince") || app.contains("okular") {
            tags.insert("#document".into());
        }
    }

    // ---  Final Sorting ---
    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}

/// Sanitizes a string into a valid tag (alphanumeric + hyphen/underscore only)
fn sanitize_tag(s: &str) -> String {
    let mut sanitized = s
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect::<String>();

    while sanitized.contains("--") {
        sanitized = sanitized.replace("--", "-");
    }

    sanitized.trim_matches('-').to_lowercase()
}

/// --- Heuristics Section ---

/// Detects if text looks like code or config.
fn is_code_like(content: &str) -> bool {
    static CODE_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    let patterns = CODE_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"(?i)^\s*<\?xml").unwrap(),
            Regex::new(r"(?i)^\s*<(!DOCTYPE|html|head|body)").unwrap(),
            Regex::new(r"^\s*(fn|def|class|import|export|package)\s+\w").unwrap(),
            Regex::new(r#"^\s*from\s+['"][^'"]+['"]\s+import"#).unwrap(),
            Regex::new(r"^\s*(let|const|var)\s+\w+\s*=").unwrap(),
            Regex::new(r#"^\s*\{\s*"?\w+"?\s*:"#).unwrap(), // JSON-like
            Regex::new(r"^\s*[-*]\s+\w+:\s").unwrap(),      // YAML
            Regex::new(r"^\s*#[include|define]").unwrap(),  // C/C++
        ]
    });
    content.lines().any(|line| patterns.iter().any(|re| re.is_match(line)))
}

/// Detects URLs (http, https, ftp, mailto, data)
fn is_url(content: &str) -> bool {
    static URL_REGEX: OnceLock<Regex> = OnceLock::new();
    let re = URL_REGEX.get_or_init(|| {
        Regex::new(r"(?i)^(https?|ftp)://[^\s]+$|^mailto:[^\s]+$|^data:[^\s]+$").unwrap()
    });
    re.is_match(content.trim())
}

/// Detects email addresses
fn is_email(content: &str) -> bool {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    let re = EMAIL_REGEX.get_or_init(|| {
        Regex::new(
            r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$"
        ).unwrap()
    });
    re.is_match(content.trim())
}

/// Detects terminal commands (CLI-like inputs)
fn is_terminal_command(content: &str) -> bool {
    let trimmed = content.trim();
    static PREFIXES: [&str; 15] = [
        "git ", "npm ", "pnpm ", "yarn ", "cargo ", "docker ", "ssh ", "sudo ",
        "./", "~/", "apt ", "pacman ", "brew ", "python ", "pip ",
    ];
    PREFIXES.iter().any(|p| trimmed.starts_with(p))
}

/// Detects file paths like `/home/user/file.txt` or `C:\path\to\file`
fn is_file_path(content: &str) -> bool {
    static PATH_REGEX: OnceLock<Regex> = OnceLock::new();
    let re = PATH_REGEX.get_or_init(|| {
        Regex::new(r"^(/[\w\-.]+)+/?$|^[A-Za-z]:\\[\w\\\-.]+$").unwrap()
    });
    re.is_match(content.trim())
}

/// Detects JSON-like structured text
fn is_json_like(content: &str) -> bool {
    let trimmed = content.trim();
    (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
}
