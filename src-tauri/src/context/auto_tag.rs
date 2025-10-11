//! Auto-tag clipboard content based on heuristics.

use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Generates auto-tags from clipboard content and optional project name.
/// Returns a vector of unique, normalized tags.
pub fn generate_auto_tags(content: &str, project_name: Option<&str>) -> Vec<String> {
    let mut tags = HashSet::new();

    // Project tag (e.g., #my-app)
    if let Some(name) = project_name {
        let sanitized = sanitize_tag(name);
        if !sanitized.is_empty() {
            tags.insert(format!("#{}", sanitized));
        }
    }

    let trimmed_content = content.trim();

    // Content-based tags
    if is_code_like(trimmed_content) {
        tags.insert("#code".to_string());
    }
    if is_url(trimmed_content) {
        tags.insert("#url".to_string());
    }
    if is_email(trimmed_content) {
        tags.insert("#email".to_string());
    }
    if is_terminal_command(trimmed_content) {
        tags.insert("#terminal".to_string());
    }

    // Return sorted vector for consistent ordering
    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}

/// Sanitizes a string to be a valid tag (alphanumeric + hyphen/underscore).
fn sanitize_tag(s: &str) -> String {
    let mut sanitized = s
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();

    // Collapse multiple consecutive dashes into one
    while sanitized.contains("--") {
        sanitized = sanitized.replace("--", "-");
    }

    sanitized.trim_matches('-').to_lowercase()
}

/// Heuristic: likely source code or config.
fn is_code_like(content: &str) -> bool {
    static CODE_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    let patterns = CODE_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"(?i)^\s*<\?xml").unwrap(),
            Regex::new(r"(?i)^\s*<(!DOCTYPE|html|head|body)").unwrap(),
            Regex::new(r"^\s*import\s+\w").unwrap(),
            Regex::new(r#"^\s*from\s+['"][^'"]+['"]\s+import"#).unwrap(),
            Regex::new(r"^\s*export\s+(default\s+)?\w").unwrap(),
            Regex::new(r"^\s*function\s+\w").unwrap(),
            Regex::new(r"^\s*const\s+\w+\s*=").unwrap(),
            Regex::new(r"^\s*let\s+\w+\s*=").unwrap(),
            Regex::new(r"^\s*var\s+\w+\s*=").unwrap(),
            Regex::new(r"^\s*class\s+\w").unwrap(),
            Regex::new(r"^\s*def\s+\w").unwrap(),     // Python
            Regex::new(r"^\s*fn\s+\w").unwrap(),      // Rust
            Regex::new(r"^\s*package\s+\w").unwrap(), // Go/Java
            Regex::new(r"^\s*public\s+class\s+\w").unwrap(),
            Regex::new(r#"^\s*\{\s*"?\w+"?\s*:"#).unwrap(), // JSON-like
            Regex::new(r"^\s*[-*]\s+\w+:\s").unwrap(),      // YAML
        ]
    });

    content
        .lines()
        .any(|line| patterns.iter().any(|re| re.is_match(line)))
}

/// Heuristic: valid URL (supports http, https, ftp, mailto, data)
fn is_url(content: &str) -> bool {
    static URL_REGEX: OnceLock<Regex> = OnceLock::new();
    let re = URL_REGEX.get_or_init(|| {
        // Match typical URLs and scheme-based URLs
        Regex::new(
            r"(?i)^(https?|ftp)://[^\s]+$|^mailto:[^\s]+$|^data:[^\s]+$"
        ).unwrap()
    });
    re.is_match(content.trim())
}

/// Heuristic: valid email
fn is_email(content: &str) -> bool {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    let re = EMAIL_REGEX.get_or_init(|| {
        Regex::new(
            r"(?i)^[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap()
    });
    re.is_match(content.trim())
}

/// Heuristic: terminal command (common CLI patterns)
fn is_terminal_command(content: &str) -> bool {
    let trimmed = content.trim();
    static TERMINAL_PREFIXES: [&str; 10] = [
        "git ", "npm ", "pnpm ", "yarn ", "cargo ", "docker ", "ssh ", "sudo ", "./", "~/",
    ];
    TERMINAL_PREFIXES
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
}
