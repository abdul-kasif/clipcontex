// src-tauri/src/storage/clip_store.rs
//! Persistent storage for clipboard history using SQLite.
//!
//! The [`ClipStore`] provides thread-safe access to a SQLite database that stores
//! clipboard entries (`Clip`). It supports CRUD operations, pinning, cleanup by age or size,
//! and automatic schema initialization.

// ===== Imports =====

use std::{
    fs::create_dir_all,
    path::Path,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use tracing::{debug, warn};

// ===== Modules =====

use crate::storage::Clip;

// ===== Domain Types =====

/// A thread-safe wrapper around an SQLite connection for managing clipboard history.
#[derive(Debug)]
pub struct ClipStore {
    conn: Arc<Mutex<Connection>>,
}

// ===== Public API Implementation =====

impl ClipStore {
    /// Creates a new `ClipStore` instance backed by a SQLite database at the given path.
    ///
    /// If the parent directory of `db_path` does not exist, it will be created recursively.
    /// The database schema (table + indexes) is initialized if not already present.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database file cannot be opened or created.
    /// - The parent directory cannot be created.
    /// - Schema initialization fails.
    pub fn new(db_path: impl AsRef<Path>) -> SqliteResult<Self> {
        let db_path = db_path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            create_dir_all(parent).map_err(|e| {
                rusqlite::Error::ToSqlConversionFailure(
                    format!("Failed to create database directory: {}", e).into(),
                )
            })?;
        }

        let conn = Connection::open(db_path)?;

        // Enable WAL mode for better concurrency and durability
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        // Initialize schema
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS clips (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                app_name TEXT,
                window_title TEXT,
                auto_tags TEXT,
                is_pinned BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_created_at ON clips(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_is_pinned ON clips(is_pinned);
            "#,
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Saves a new clip to the database.
    ///
    /// The returned `Clip` includes the assigned database ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the database write fails.
    pub fn save_clip(&self, clip: Clip) -> SqliteResult<Clip> {
        let conn = self.get_db_connection();

        conn.execute(
            r#"
            INSERT INTO clips (
                content, app_name, window_title,
                auto_tags, is_pinned,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                clip.content,
                clip.app_name,
                clip.window_title,
                clip.auto_tags,
                clip.is_pinned,
                clip.created_at.to_rfc3339(),
                clip.updated_at.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid() as i32;
        debug!("Saved clip with id={}", id);

        Ok(Clip {
            id: Some(id),
            ..clip.clone()
        })
    }

    /// Retrieves the most recent clips, up to the specified limit.
    ///
    /// Clips are ordered from newest to oldest by `created_at`.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails or timestamp parsing fails.
    pub fn list_recent_clips(&self, limit: i32) -> SqliteResult<Vec<Clip>> {
        let conn = self.get_db_connection();

        let mut stmt = conn.prepare(
            r#"
            SELECT id, content, app_name, window_title,
                   auto_tags, is_pinned,
                   created_at, updated_at
            FROM clips
            ORDER BY created_at DESC
            LIMIT ?1
            "#,
        )?;

        let clips = stmt
            .query_map([limit], |row| {
                let created_raw: String = row.get(6)?;
                let updated_raw: String = row.get(7)?;

                Ok(Clip {
                    id: Some(row.get(0)?),
                    content: row.get(1)?,
                    app_name: row.get(2)?,
                    window_title: row.get(3)?,
                    auto_tags: row.get(4)?,
                    is_pinned: row.get(5)?,
                    created_at: parse_timestamp(&created_raw),
                    updated_at: parse_timestamp(&updated_raw),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(clips)
    }

    /// Updates the pinned status of a clip by its ID.
    ///
    /// Sets `is_pinned` and updates the `updated_at` timestamp.
    ///
    /// # Errors
    ///
    /// Returns an error if the update fails (e.g., invalid ID).
    pub fn toggle_pin_status(&self, id: i32, is_pinned: bool) -> SqliteResult<()> {
        let conn = self.get_db_connection();

        conn.execute(
            "UPDATE clips SET is_pinned = ?1, updated_at = ?2 WHERE id = ?3",
            params![is_pinned, Utc::now().to_rfc3339(), id],
        )?;

        Ok(())
    }

    /// Deletes a clip by its database ID.
    ///
    /// No error is returned if the ID does not exist.
    ///
    /// # Errors
    ///
    /// Returns an error only if the database operation fails.
    pub fn remove_clip(&self, id: i32) -> SqliteResult<()> {
        let conn = self.get_db_connection();
        conn.execute("DELETE FROM clips WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Deletes all clips from the database.
    ///
    /// This operation cannot be undone.
    ///
    /// # Errors
    ///
    /// Returns an error if the deletion fails.
    pub fn clear_clip_history(&self) -> SqliteResult<()> {
        let conn = self.get_db_connection();
        conn.execute("DELETE FROM clips", [])?;
        debug!("Cleared all clips from history.");
        Ok(())
    }

    /// Performs automatic cleanup of the clip history.
    ///
    /// Applies two strategies in sequence:
    /// 1. Removes clips older than `days`.
    /// 2. Trims history to at most `max_size` most recent clips.
    ///
    /// # Errors
    ///
    /// Returns an error if either cleanup step fails.
    pub fn perform_cleanup(&self, days: i64, max_size: i64) -> SqliteResult<()> {
        self.remove_clips_older_than_days(days)?;
        self.enforce_max_size(max_size)?;
        Ok(())
    }
}

// ===== Private Helper Methods =====

impl ClipStore {
    /// Acquires a lock on the SQLite connection, recovering from poisoning if necessary.
    ///
    /// If a previous thread panicked while holding the mutex, the lock is still acquired
    /// and a warning is logged. This is safe because SQLite operations are short and atomic.
    fn get_db_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| {
            warn!("Database mutex was poisoned; recovering...");
            e.into_inner()
        })
    }

    /// Deletes clips older than the specified number of days.
    fn remove_clips_older_than_days(&self, days: i64) -> SqliteResult<usize> {
        let conn = self.get_db_connection();
        let cutoff = (Utc::now() - Duration::days(days)).to_rfc3339();

        let deleted = conn.execute(
            "DELETE FROM clips WHERE datetime(created_at) < datetime(?1)",
            params![cutoff],
        )?;

        debug!("Deleted {} clips older than {} days", deleted, days);
        Ok(deleted)
    }

    /// Ensures the total number of clips does not exceed `max_size`.
    ///
    /// Keeps the `max_size` most recent clips (by `created_at`) and deletes the rest.
    fn enforce_max_size(&self, max_size: i64) -> SqliteResult<usize> {
        let conn = self.get_db_connection();

        let deleted = conn.execute(
            r#"
            DELETE FROM clips
            WHERE id NOT IN (
                SELECT id FROM clips
                ORDER BY created_at DESC
                LIMIT ?1
            )
            "#,
            params![max_size],
        )?;

        debug!(
            "Trimmed {} old clips to enforce max size of {}",
            deleted, max_size
        );
        Ok(deleted)
    }
}

// ===== Standalone Helper Functions =====

/// Parses an RFC3339 timestamp string into a `DateTime<Utc>`.
///
/// On parsing failure, logs a warning and returns the current time as a fallback.
/// This ensures robustness when reading potentially corrupted or malformed data.
fn parse_timestamp(s: &str) -> DateTime<Utc> {
    match DateTime::parse_from_rfc3339(s) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(e) => {
            warn!("Failed to parse timestamp: '{}', {}", s, e);
            Utc::now()
        }
    }
}

// ===== Integration Tests =====

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn setup_test_store() -> ClipStore {
        ClipStore::new(":memory:").expect("Failed to create in-memory store")
    }

    #[test]
    fn test_save_and_retrieve_clip() {
        let store = setup_test_store();
        let clip = Clip::new(
            "Hello, world!".to_string(),
            "Terminal".to_string(),
            "bash".to_string(),
            vec!["test".to_string()],
            false,
        );

        let saved = store.save_clip(clip).unwrap();
        assert!(saved.id.is_some());
        assert_eq!(saved.content, "Hello, world!");

        let recent = store.list_recent_clips(10).unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, saved.id);
    }

    #[test]
    fn test_pin_status_update() {
        let store = setup_test_store();
        let clip = Clip::new("Pin me".into(), "App".into(), "Win".into(), vec![], false);
        let saved = store.save_clip(clip).unwrap();

        store.toggle_pin_status(saved.id.unwrap(), true).unwrap();

        let recent = store.list_recent_clips(1).unwrap();
        assert!(recent[0].is_pinned);
    }

    #[test]
    fn test_remove_clip() {
        let store = setup_test_store();
        let clip = Clip::new(
            "Delete me".into(),
            "App".into(),
            "Win".into(),
            vec![],
            false,
        );
        let saved = store.save_clip(clip).unwrap();
        let id = saved.id.unwrap();

        store.remove_clip(id).unwrap();

        let recent = store.list_recent_clips(10).unwrap();
        assert!(recent.is_empty());
    }

    #[test]
    fn test_clear_clip_history() {
        let store = setup_test_store();
        let clip = Clip::new("Temp".into(), "App".into(), "Win".into(), vec![], false);
        store.save_clip(clip).unwrap();
        store.clear_clip_history().unwrap();

        let recent = store.list_recent_clips(10).unwrap();
        assert!(recent.is_empty());
    }

    #[test]
    fn test_cleanup_by_age() {
        let store = setup_test_store();

        let mut old_clip = Clip::new(
            "Old".into(),
            "OldApp".into(),
            "OldWin".into(),
            vec![],
            false,
        );
        old_clip.created_at = Utc::now() - Duration::days(10);
        old_clip.updated_at = old_clip.created_at;
        store.save_clip(old_clip).unwrap();

        let new_clip = Clip::new(
            "New".into(),
            "NewApp".into(),
            "NewWin".into(),
            vec![],
            false,
        );
        store.save_clip(new_clip).unwrap();

        store.perform_cleanup(5, 100).unwrap();

        let remaining = store.list_recent_clips(10).unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].content, "New");
    }

    #[test]
    fn test_cleanup_by_max_size() {
        let store = setup_test_store();

        for i in 0..5 {
            let clip = Clip::new(
                format!("Clip {}", i),
                "App".into(),
                "Win".into(),
                vec![],
                false,
            );
            store.save_clip(clip).unwrap();
        }

        store.perform_cleanup(30, 3).unwrap();

        let remaining = store.list_recent_clips(10).unwrap();
        assert_eq!(remaining.len(), 3);
        assert_eq!(remaining[0].content, "Clip 4");
        assert_eq!(remaining[1].content, "Clip 3");
        assert_eq!(remaining[2].content, "Clip 2");
    }

    #[test]
    fn test_timestamp_parsing_failure() {
        let store = setup_test_store();
        {
            let conn = store.get_db_connection();
            conn.execute(
                r#"INSERT INTO clips (content, app_name, window_title, auto_tags, is_pinned, created_at, updated_at)
                   VALUES ('bad', 'app', 'win', '', 0, 'invalid', 'also_invalid')"#,
                [],
            )
            .unwrap();
        }

        let clips = store.list_recent_clips(10).unwrap();
        assert_eq!(clips.len(), 1);
    }

    #[test]
    fn test_concurrent_access_smoke() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(setup_test_store());

        let handles: Vec<_> = (0..5)
            .map(|i| {
                let store = Arc::clone(&store);
                thread::spawn(move || {
                    let clip = Clip::new(
                        format!("Concurrent {}", i),
                        "App".into(),
                        "Win".into(),
                        vec![],
                        false,
                    );
                    store.save_clip(clip).unwrap();
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let clips = store.list_recent_clips(10).unwrap();
        assert_eq!(clips.len(), 5);
    }

    #[test]
    fn test_large_content_handling() {
        let store = setup_test_store();
        let large_content = "A".repeat(10_000_000); // 10 MB
        let clip = Clip::new(
            large_content.clone(),
            "BigApp".into(),
            "BigWin".into(),
            vec![],
            false,
        );

        let start = Instant::now();
        let _saved = store.save_clip(clip).unwrap();
        let save_duration = start.elapsed();

        let retrieved = store.list_recent_clips(1).unwrap();
        let retrieve_duration = start.elapsed() - save_duration;

        assert_eq!(retrieved[0].content, large_content);

        eprintln!(
            "Saved 10MB clip in {:?}, retrieved in {:?}",
            save_duration, retrieve_duration
        );
    }
}
