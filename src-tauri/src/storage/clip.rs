// src-tauri/src/storage/clip.rs
//! Domain model for a clipboard entry.
//!
//! A [`Clip`] represents a single item in the user's clipboard history,
//! capturing not only the content but also contextual metadata such as
//! the source application, window title, and auto-generated tags.

// ===== Imports =====

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ===== Domain Types =====

/// Represents a clipboard entry with contextual metadata.
///
/// Each `Clip` is designed to be stored persistently (e.g., in SQLite) and
/// displayed in the application’s history UI. It includes:
/// - The actual clipboard `content`.
/// - Metadata like `app_name` and `window_title` to aid user recall.
/// - `auto_tags`: a comma-separated string of inferred tags (e.g., "url,email").
/// - Pin status for user-controlled persistence.
/// - Timestamps for sorting and cleanup.
///
/// # Storage Note
///
/// The `auto_tags` field is stored as a single `String` (comma-delimited)
/// for simplicity. This avoids schema complexity when full-text or tag-based
/// querying is not required. If advanced tag operations are needed in the future,
/// consider migrating to a separate tags table or JSON storage.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Clip {
    /// The database ID assigned upon persistence. `None` for new, unsaved clips.
    pub id: Option<i32>,

    /// The actual clipboard content (text only).
    ///
    /// Binary or rich content is not supported in this version.
    pub content: String,

    /// Name of the application that owned the clipboard at capture time.
    ///
    /// Example: `"Visual Studio Code"`, `"Google Chrome"`.
    pub app_name: String,

    /// Title of the active window when the clip was captured.
    ///
    /// Example: `"main.rs - my-project"`.
    pub window_title: String,

    /// Auto-generated tags as a comma-separated string.
    ///
    /// Tags are derived from content analysis (e.g., detecting URLs, emails).
    /// Stored as `"tag1,tag2,tag3"` for compactness and ease of display.
    /// Not intended for efficient querying—use only for UI hints.
    pub auto_tags: String,

    /// Whether the user has pinned this clip to prevent automatic cleanup.
    ///
    /// Pinned clips are excluded from age- or size-based deletion policies.
    pub is_pinned: bool,

    /// UTC timestamp when the clip was first created.
    pub created_at: DateTime<Utc>,

    /// UTC timestamp when the clip was last modified (e.g., pin status changed).
    pub updated_at: DateTime<Utc>,
}

// ===== Public API Implementation =====

impl Clip {
    /// Creates a new, unsaved [`Clip`] instance.
    ///
    /// Timestamps are set to the current UTC time.
    ///
    /// # Parameters
    ///
    /// - `content`: The clipboard text.
    /// - `app_name`: Name of the source application.
    /// - `window_title`: Title of the source window.
    /// - `auto_tags`: A list of semantic tags; joined into a comma-separated string internally.
    /// - `is_pinned`: Initial pin state (`false` by default for new clips).
    ///
    /// # Example
    ///
    /// ```rust
    /// use clipcontex_lib::storage::Clip;
    /// let clip = Clip::new(
    ///     "https://example.com".into(),
    ///     "Firefox".into(),
    ///     "Example Domain".into(),
    ///     vec!["url".to_string()],
    ///     false,
    /// );
    /// assert_eq!(clip.auto_tags, "url");
    /// ```
    pub fn new(
        content: String,
        app_name: String,
        window_title: String,
        auto_tags: Vec<String>,
        is_pinned: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            content,
            app_name,
            window_title,
            auto_tags: auto_tags.join(","),
            is_pinned,
            created_at: now,
            updated_at: now,
        }
    }
}
