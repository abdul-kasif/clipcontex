// src-tauri/src/core/cleanup.rs
//! Background auto-cleanup task for clipboard history.
//!
//! Periodically removes old clips based on user-configured retention policy.
//! Runs every 6 hours.

use std::sync::{Arc, RwLock};
use tauri::async_runtime;
use tokio::time::{interval, Duration};
use tracing::error;

use crate::{config::Settings, storage::ClipStore};

/// Spawns a background task that performs automatic cleanup every 6 hours.
///
/// Cleanup respects:
/// - `auto_clean_days`: Remove clips older than N days.
/// - `max_history_size`: Keep at most N clips.
pub fn spawn_auto_cleanup_task(settings: Arc<RwLock<Settings>>, clip_store: Arc<ClipStore>) {
    async_runtime::spawn(async move {
        let mut ticker = interval(Duration::from_hours(6));

        loop {
            ticker.tick().await;

            let (days, max_size) = read_cleanup_settings(&settings);

            if days > 0 {
                match clip_store.perform_cleanup(days as i64, max_size as i64) {
                    Ok(_) => tracing::info!("Auto cleanup completed"),
                    Err(e) => tracing::error!("Auto cleanup failed: {}", e),
                }
            }
        }
    });
}

fn read_cleanup_settings(settings_arc: &Arc<RwLock<Settings>>) -> (u32, u32) {
    match settings_arc.read() {
        Ok(settings) => (settings.auto_clean_days, settings.max_history_size),
        Err(poisoned) => {
            error!("Settings lock is poisoned");
            let settings = poisoned.into_inner();
            (settings.auto_clean_days, settings.max_history_size)
        }
    }
}

