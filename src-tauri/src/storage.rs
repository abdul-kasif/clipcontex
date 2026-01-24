// src-tauri/src/storage.rs
//! Persistent clipboard history storage.
//!
//! This module provides a thread-safe, SQLite-backed storage system for managing
//! clipboard history entries. It consists of two core components:
//!
//! - [`Clip`]: The domain model representing a single clipboard entry with metadata.
//! - [`ClipStore`]: A database interface for saving, retrieving, updating, and cleaning up clips.
//!
//! The storage layer is designed to be:
//! - **Efficient**: Uses indexed queries and WAL mode for performance.
//! - **Robust**: Handles edge cases like timestamp corruption gracefully.
//! - **Resource-conscious**: Supports automatic cleanup by age or maximum history size.
//!
//! # Example
//!
//! ```rust
//! use clipcontex_lib::storage::{Clip, ClipStore};
//!
//! // Initialize storage (e.g., in app startup)
//! let store = ClipStore::new("data/clipboard.db").expect("Failed to open storage");
//!
//! // Create and save a clip
//! let clip = Clip::new(
//!     "Hello, world!".into(),
//!     "Terminal".into(),
//!     "bash".into(),
//!     vec!["text".to_string()],
//!     false,
//! );
//! let saved_clip = store.save_clip(clip).unwrap();
//!
//! // Retrieve recent history
//! let history = store.list_recent_clips(50).unwrap();
//! ```
//!
//! For automatic maintenance, call [`ClipStore::perform_cleanup`] on application startup
//! to enforce retention policies.

mod clip;
mod clip_store;

pub use clip::Clip;
pub use clip_store::ClipStore;
