// src-tauri/src/storage/clip.rs
// ===== Imports =====
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ===== Domain Types =====
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Clip {
    pub id: Option<i32>,
    pub content: String,
    pub app_name: String,
    pub window_title: String,
    pub auto_tags: String,
    pub is_pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ===== Clip Implementation =====
impl Clip {
    pub fn new(
        content: String,
        app_name: String,
        window_title: String,
        auto_tags: Vec<String>,
        is_pinned: bool,
    ) -> Self {
        Self {
            id: None,
            content,
            app_name,
            window_title,
            auto_tags: auto_tags.join(","),
            is_pinned,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
