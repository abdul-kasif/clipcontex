// src-tauri/src/context/auto_tags.rs
//! Automatic tag generation for clipboard entries.
//!
//! Tags are generated based on:
//! - **Content analysis**: Detects URLs, emails, code snippets, etc.
//! - **Application context**: Maps app names to categories (e.g., "VS Code" → `#editor`).
//!
//! All tags are prefixed with `#` and deduplicated. The final list is sorted alphabetically
//! for consistent display.

use std::collections::HashSet;

pub mod app_based;
pub mod content_based;

/// Generates automatic tags for a clipboard entry.
///
/// # Arguments
///
/// - `content`: The clipboard text (trimmed internally).
/// - `app_class`: Optional normalized application name (e.g., "Visual Studio Code").
///
/// # Returns
///
/// A sorted vector of unique tags (e.g., `["#code", "#editor", "#vscode"]`).
pub fn generate_auto_tags(content: &str, app_class: Option<&str>) -> Vec<String> {
    let mut tags = HashSet::new();
    let trimmed_content = content.trim();
    if !trimmed_content.is_empty() {
        content_based::generate_content_based_tags(trimmed_content, &mut tags);
    }
    if let Some(app_class) = app_class {
        app_based::generate_app_based_tags(app_class, &mut tags);
    }

    let mut tags_vec: Vec<_> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}

// ===== Tests =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_auto_tags_full() {
        let tags = generate_auto_tags("https://example.com", Some("Firefox"));
        assert!(tags.contains(&"#url".to_string()));
        assert!(tags.contains(&"#browser".to_string()));
        assert!(tags.contains(&"#firefox".to_string()));
    }

    #[test]
    fn test_empty_content() {
        let tags = generate_auto_tags("", Some("Code"));
        assert_eq!(tags, vec!["#code".to_string(), "#editor".to_string()]);
    }

    #[test]
    fn test_no_app_class() {
        let tags = generate_auto_tags("user@example.com", None);
        assert_eq!(tags, vec!["#email".to_string()]);
    }
}
