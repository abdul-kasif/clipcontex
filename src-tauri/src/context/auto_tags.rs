// ===== Imports =====
use std::collections::HashSet;

// ===== Modules =====
mod app_based;
mod content_based;

// ===== Public API =====
pub fn generate_auto_tags(content: &str, app_class: Option<&str>) -> Vec<String> {
    let mut tags = HashSet::new();
    let trimmed_content = content.trim();
    if !trimmed_content.is_empty() {
        content_based::generate_content_based_tags(content, &mut tags);
    }
    if let Some(app_class) = app_class {
        app_based::generate_app_based_tags(app_class, &mut tags);
    }

    let mut tags_vec: Vec<_> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}
