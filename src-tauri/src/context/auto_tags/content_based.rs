use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

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
        Regex::new(r"(?i)^(https?|ftp)://[^\s]+$|^mailto:[^\s]+$|^[^\s]+/[^\s]+$").unwrap()
    });
    re.is_match(content)
}

fn is_email(content: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
    re.is_match(content)
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
    PREFIXES.iter().any(|&p| content.starts_with(p))
}

fn is_file_path(content: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| Regex::new(r"^(/[\w\-.~]+)+/?$|^[A-Za-z]:[\\\/][\w\\\/\-.~]+$").unwrap());
    re.is_match(content)
}

fn is_json_like(content: &str) -> bool {
    let t = content.trim();
    (t.starts_with('{') && t.ends_with('}')) || (t.starts_with('[') && t.ends_with(']'))
}
