use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
#[allow(unused_variables)]
use std::path::{Path, PathBuf};
use tracing::debug;

use super::clip::Clip;

/// Manages persistent storage of clipboard clips.
#[derive(Debug, Clone)]
pub struct ClipStore {
    db_path: PathBuf,
}

impl ClipStore {
    /// Creates a new ClipStore with the given database path.
    pub fn new(db_path: impl AsRef<Path>) -> Self {
        Self {
            db_path: db_path.as_ref().to_path_buf(),
        }
    }

    /// Initializes the database and creates tables + indexes if they don't exist.
    pub fn init(&self) -> SqliteResult<()> {
        // Ensure directory exists safely
        if let Some(parent) = self.db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        }

        let conn = Connection::open(&self.db_path)?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS clips (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                app_name TEXT,
                window_title TEXT,
                auto_tags TEXT,
                manual_tags TEXT,
                is_pinned BOOLEAN,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_created_at ON clips(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_is_pinned ON clips(is_pinned);
            "#,
        )?;

        Ok(())
    }

    /// Saves a clip to the database.
    /// Returns the same clip with its assigned `id`.
    pub fn save_clip(&self, clip: &Clip) -> SqliteResult<Clip> {
        let conn = Connection::open(&self.db_path)?;

        conn.execute(
            r#"
            INSERT INTO clips (
                content, app_name, window_title,
                auto_tags, manual_tags, is_pinned,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                clip.content,
                clip.app_name,
                clip.window_title,
                clip.auto_tags,
                clip.manual_tags,
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

    /// Retrieves the most recent clips (up to `limit`), ordered by creation time.
    pub fn get_recent_clips(&self, limit: i32) -> SqliteResult<Vec<Clip>> {
        let conn = Connection::open(&self.db_path)?;

        let mut stmt = conn.prepare(
            r#"
            SELECT id, content, app_name, window_title,
                   auto_tags, manual_tags, is_pinned,
                   created_at, updated_at
            FROM clips
            ORDER BY created_at DESC
            LIMIT ?1
            "#,
        )?;

        let clips = stmt
            .query_map([limit], |row| {
                let created_raw: String = row.get(7)?;
                let updated_raw: String = row.get(8)?;

                Ok(Clip {
                    id: Some(row.get(0)?),
                    content: row.get(1)?,
                    app_name: row.get(2)?,
                    window_title: row.get(3)?,
                    auto_tags: row.get(4)?,
                    manual_tags: row.get(5)?,
                    is_pinned: row.get(6)?,
                    created_at: parse_timestamp(&created_raw),
                    updated_at: parse_timestamp(&updated_raw),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(clips)
    }

    /// Clears all clips from the database.
    pub fn clear_history(&self) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;

        conn.execute("DELETE FROM clips", [])?;

        debug!("Cleared all clips from history.");
        Ok(())
    }

    /// Deletes a specific clip by ID.
    pub fn delete_clip(&self, id: i32) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;

        conn.execute("DELETE FROM clips WHERE id = ?1", [id])?;

        Ok(())
    }

    /// Updates pin status for a given clip.
    pub fn set_pin_status(&self, id: i32, is_pinned: bool) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;

        conn.execute(
            "UPDATE clips SET is_pinned = ?1, updated_at = ?2 WHERE id = ?3",
            params![is_pinned, Utc::now().to_rfc3339(), id],
        )?;

        Ok(())
    }

    /// Removes clips older than the specified number of days.
    pub fn remove_older_than_days(&self, days: i64) -> SqliteResult<usize> {
        let conn = Connection::open(&self.db_path)?;
        let cutoff = (Utc::now() - Duration::days(days.into())).to_rfc3339();

        let deleted = conn.execute(
            "DELETE FROM clips WHERE datetime(created_at) < datetime(?1)",
            params![cutoff],
        )?;

        debug!("Deleted {} clips older than {} days", deleted, days);
        Ok(deleted)
    }

    /// Ensures that only the most recent `max_size` clips are kept.
    pub fn enforce_max_size(&self, max_size: i64) -> SqliteResult<usize> {
        let conn = Connection::open(&self.db_path)?;

        // Delete all except the most recent `max_size` clips
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

    // To perform cleanup automatically when application starts.
    pub fn perform_cleanup(&self, days: i64, max_size: i64) -> SqliteResult<()> {
        self.remove_older_than_days(days)?;
        self.enforce_max_size(max_size)?;
        Ok(())
    }
}
/// Safely parses RFC3339 timestamps, falling back to `Utc::now()` if invalid.
fn parse_timestamp(s: &str) -> DateTime<Utc> {
    match DateTime::parse_from_rfc3339(s) {
        Ok(dt) => dt.with_timezone(&Utc),
        #[allow(unused_variables)]
        Err(e) => {
            // warn!("Invalid timestamp '{}': {}, defaulting to now()", s, e);
            Utc::now()
        }
    }
}
