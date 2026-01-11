// ===== TODO =====
// - add event driven cleanup
//     - when startup
//     - when clip saved
//     - when settings updated
//     - when main window opened from system tray

// ===== Imports =====
use std::sync::{Arc, RwLock};
use tauri::async_runtime;
use tokio::time::{interval, Duration};
use tracing::error;

// ===== Crates =====
use crate::{config::Settings, storage::ClipStore};

// ===== Public API =====
pub fn spawn_auto_cleanup_task(settings: Arc<RwLock<Settings>>, clip_store: Arc<ClipStore>) {
    async_runtime::spawn(async move {
        let mut ticker = interval(Duration::from_hours(6));

        loop {
            ticker.tick().await;

            let (days, max_size) = read_cleanup_settings(&settings);

            if days > 0 {
                match clip_store.perform_cleanup(days as i64, max_size as i64) {
                    Ok(_) => {
                        tracing::info!("Auto cleanup completed");
                    }
                    Err(e) => tracing::error!("Auto cleanup failed: {}", e),
                }
            }
        }
    });
}

// ===== Helper Functions =====
fn read_cleanup_settings(settings_arc: &Arc<RwLock<Settings>>) -> (u32, u32) {
    match settings_arc.read() {
        Ok(settings) => (settings.auto_clean_days, settings.max_history_size),
        Err(poisoned) => {
            error!("Settings lock is poisioned");
            let settings = poisoned.into_inner();
            (settings.auto_clean_days, settings.max_history_size)
        }
    }
}
