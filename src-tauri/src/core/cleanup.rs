use crate::{commands::AppState, config::Settings, core::platform};
use std::sync::{Arc, RwLock};
use tracing::error;

use tauri::async_runtime;
use tokio::time::{interval, Duration};

pub fn spawn_auto_cleanup_task(app_state: &AppState) {
    let app_state_clone = app_state.clone();
    async_runtime::spawn(async move {
        let mut ticker = interval(Duration::from_hours(6));

        loop {
            ticker.tick().await;

            let (days, max_size) = read_cleanup_settings(&app_state_clone.settings);

            if days > 0 {
                match app_state_clone
                    .clip_store
                    .perform_cleanup(days as i64, max_size as i64)
                {
                    Ok(_) => {
                        tracing::info!("Auto cleanup completed");
                        platform::malloc_trim_now();
                    }
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
            error!("Settings lock is poisioned");
            let settings = poisoned.into_inner();
            (settings.auto_clean_days, settings.max_history_size)
        }
    }
}
