// src-tauri/src/context/auto_tags/content_based.rs
//! Content-based tag generation using heuristics and regex patterns.
//!
//! Detects common data types in clipboard content:
//! - Code snippets
//! - URLs
//! - Email addresses
//! - Terminal commands
//! - File paths
//! - JSON structures

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Analyzes clipboard content and adds relevant tags to the set.
pub fn generate_content_based_tags(content: &str, tags: &mut HashSet<String>) {
    if is_code_like(content) {
        tags.insert("#code".into());
    }
    if is_url(content) {
        tags.insert("#url".into());
    }
    if is_email(content) {
        tags.insert("#email".into());
    }
    if is_terminal_command(content) {
        tags.insert("#terminal".into());
    }
    if is_file_path(content) {
        tags.insert("#path".into());
    }
    if is_json_like(content) {
        tags.insert("#json".into());
    }
}

fn is_code_like(content: &str) -> bool {
    static PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    let patterns = PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"(?i)^\s*<\?xml").unwrap(),
            Regex::new(r"(?i)^\s*<(!DOCTYPE|html|head|body)").unwrap(),
            Regex::new(r"^\s*(fn|def|class|import|export|package)\s+\w").unwrap(),
            Regex::new(r#"^\s*from\s+['"][^'"]+['"]\s+import"#).unwrap(),
            Regex::new(r"^\s*(let|const|var)\s+\w+\s*=").unwrap(),
            Regex::new(r#"^\s*\{\s*"?\w+"?\s*:"#).unwrap(),
            Regex::new(r"^\s*[-*]\s+\w+:\s").unwrap(),
            Regex::new(r"^\s*#[ \t]*(include|define|if|ifdef|ifndef|endif|pragma)").unwrap(),
        ]
    });

    content
        .lines()
        .any(|line| patterns.iter().any(|re| re.is_match(line)))
}

fn is_url(content: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?i)^(https?|ftp)://[^\s]+$|^[a-z0-9.-]+\.[a-z]{2,}(:\d+)?(/[^\s]*)?$")
            .unwrap()
    });
    re.is_match(content.trim())
}

fn is_email(content: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
    re.is_match(content.trim())
}

fn is_terminal_command(content: &str) -> bool {
    const PREFIXES: &[&str] = &[
        "git ",
        "npm ",
        "pnpm ",
        "yarn ",
        "cargo ",
        "docker ",
        "ssh ",
        "sudo ",
        "./",
        "~/",
        "apt ",
        "pacman ",
        "brew ",
        "python ",
        "pip ",
        "kubectl ",
        "helm ",
        "terraform ",
        "make ",
        "gcc ",
        "clang ",
        "nvim ",
        "vim ",
    ];
    let trimmed = content.trim_start();
    PREFIXES.iter().any(|&p| trimmed.starts_with(p))
}

fn is_file_path(content: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| Regex::new(r"^(/[\w\-.~]+)+/?$|^[A-Za-z]:[\\\/][\w\\\/\-.~]+$").unwrap());
    re.is_match(content.trim())
}

fn is_json_like(content: &str) -> bool {
    let t = content.trim();
    (t.starts_with('{') && t.ends_with('}')) || (t.starts_with('[') && t.ends_with(']'))
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urls() {
        assert!(is_url("https://google.com"));
        assert!(is_url("ftp://files.example.com"));
        assert!(is_url("example.com/path"));
        assert!(!is_url("hello world"));
    }

    #[test]
    fn test_emails() {
        assert!(is_email("user@example.com"));
        assert!(!is_email("invalid-email"));
    }

    #[test]
    fn test_code_detection() {
        assert!(is_code_like("fn main() {"));
        assert!(is_code_like("<html>"));
        assert!(!is_code_like("just text"));
    }

    #[test]
    fn test_terminal_commands() {
        assert!(is_terminal_command("git commit"));
        assert!(is_terminal_command("./script.sh"));
        assert!(!is_terminal_command("this is not a command"));
    }

    #[test]
    fn test_file_paths() {
        assert!(is_file_path("/home/user/file.txt"));
        assert!(is_file_path("C:\\Users\\file.txt"));
        assert!(!is_file_path("relative/path")); // by design
    }

    #[test]
    fn test_json_like() {
        assert!(is_json_like("{\"key\": \"value\"}"));
        assert!(is_json_like("[1, 2, 3]"));
        assert!(!is_json_like("not json"));
    }
}
