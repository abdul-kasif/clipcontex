use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a clipboard content with context.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Clip {
    pub id: Option<i32>,
    pub content: String,
    pub app_name: String,
    pub window_title: String,
    pub auto_tags: String, // comma seperated tags like "#code, #terninal"
    pub manual_tags: String,
    pub is_pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Clip {
    /// Creates a new clip with current timestamps
    pub fn new(
        content: String,
        app_name: String,
        window_title: String,
        auto_tags: Vec<String>,
        manual_tags: Vec<String>, // will be used in upcomming version
        is_pinned: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            content,
            app_name,
            window_title,
            auto_tags: auto_tags.join(","),
            manual_tags: manual_tags.join(","),
            is_pinned,
            created_at: now,
            updated_at: now,
        }
    }

    /// Parses auto_tags back into a Vec<String>.
    pub fn auto_tags_vec(&self) -> Vec<String> {
        if self.auto_tags.is_empty() {
            vec![]
        } else {
            self.auto_tags
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    }

    /// Parses auto_tags back into a Vec<String>.
    pub fn manual_tags_vec(&self) -> Vec<String> {
        if self.manual_tags.is_empty() {
            vec![]
        } else {
            self.manual_tags
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    }
}
