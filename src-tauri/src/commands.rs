// src-tauri/src/commands.rs
use std::{
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex, RwLock},
};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_autostart::ManagerExt;
use tracing::{error, info, warn};

use crate::{
    clipboard::watcher::{mark_ignore_next_clipboard_update, ClipboardWatcherHandle},
    config::{load_settings, save_settings, Settings as ConfigSettings},
    storage::{Clip, ClipStore},
};

/// Event constants for consistency
const EVT_CLIP_UPDATED: &str = "clip-updated";
const EVT_CLIP_DELETED: &str = "clip-deleted";
const EVT_HISTORY_CLEARED: &str = "history-cleared";
const EVT_SETTINGS_UPDATED: &str = "settings-updated";

#[derive(Clone)]
/// Shared global application state
pub struct AppState {
    pub clip_store: Arc<ClipStore>,
    pub watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
    pub settings: Arc<RwLock<ConfigSettings>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
impl AppState {
    /// Initializes persistent ClipStore under `~/.clipcontex/clipcontex.db`
    pub fn new() -> Self {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".clipcontex/clipcontex.db");

        let store = ClipStore::new(&db_path).unwrap_or_else(|e| {
            error!("Failed to initialize ClipStore DB at {:?}: {}", db_path, e);
            panic!("Failed to initialize database: {}", e);
        });

        info!(target:"clipcontex::initialization","ClipStore initialized at {:?}", db_path);

        let settings = match load_settings() {
            Ok(s) => s,
            Err(e) => {
                warn!(target:"clipcontex::initialization","failed to load settings, using defaults: {}", e);
                ConfigSettings::default()
            }
        };
        Self {
            clip_store: Arc::new(store),
            watcher_handle: Arc::new(Mutex::new(None)),
            settings: Arc::new(RwLock::new(settings)),
        }
    }
}

/// Helper for consistent string error formatting
fn err<E: std::fmt::Display>(context: &str, e: E) -> String {
    format!("{}: {}", context, e)
}

#[command]
pub async fn ignore_next_clipboard_update(content: String) {
    mark_ignore_next_clipboard_update(content);
}

/// Clip management commands

#[command]
pub async fn get_recent_clips(
    app_state: State<'_, AppState>,
    limit: i32,
) -> Result<Vec<Clip>, String> {
    app_state
        .clip_store
        .get_recent_clips(limit)
        .map_err(|e| err("Failed to fetch recent clips", e))
}

#[command]
pub async fn clear_history(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    app_state
        .clip_store
        .clear_history()
        .map_err(|e| err("Failed to clear history", e))?;

    if let Err(e) = app_handle.emit(EVT_HISTORY_CLEARED, ()) {
        error!(target: "clipcontex::commands","Failed to emit clear_history event '{}': {}", EVT_HISTORY_CLEARED, e);
    }

    Ok(())
}

#[command]
pub async fn delete_clip(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    id: i32,
) -> Result<(), String> {
    app_state
        .clip_store
        .delete_clip(id)
        .map_err(|e| err("Failed to delete clip", e))?;

    // Only emit ID instead of full clip data
    if let Err(e) = app_handle.emit(EVT_CLIP_DELETED, &id) {
        error!(target:"clipcontex::commands","Failed to emit delete_clip event '{}': {}", EVT_CLIP_DELETED, e);
    }

    Ok(())
}

#[command]
pub async fn pin_clip(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    id: i32,
    is_pinned: bool,
) -> Result<(), String> {
    app_state
        .clip_store
        .set_pin_status(id, is_pinned)
        .map_err(|e| err("Failed to update pin status", e))?;

    // Only emit minimal update data
    if let Err(e) = app_handle.emit(EVT_CLIP_UPDATED, &(id, is_pinned)) {
        error!(target:"clipcontex::commands","Failed to emit pin_clip event '{}': {}", EVT_CLIP_UPDATED, e);
    }

    Ok(())
}

/// Configuration commands

#[command]
pub async fn load_config() -> Result<ConfigSettings, String> {
    load_settings().map_err(|e| err("Failed to read config", e))
}

#[command]
pub async fn save_config(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    settings: ConfigSettings,
) -> Result<&str, String> {
    // Save to file immediately (atomic write)
    if let Err(e) = save_settings(&settings) {
        return Err(format!("Failed to save config: {}", e));
    }

    // Sync autostart immediately to reflect latest state
    let launcher = app_handle.autolaunch();

    match launcher.is_enabled() {
        Ok(current_status) => {
            if settings.is_autostart_enabled && !current_status {
                if let Err(e) = launcher.enable() {
                    error!(target:"clipcontex::commands","Failed to enable autostart: {}", e);
                } else {
                    info!(target:"clipcontex::commands","Autostart enabled (user changed setting).");
                }
            } else if !settings.is_autostart_enabled && current_status {
                if let Err(e) = launcher.disable() {
                    error!(target:"clipcontex::commands","Failed to disable autostart: {}", e);
                } else {
                    info!(target:"clipcontex::commands","Autostart disabled (user changed setting).");
                }
            } else {
                info!(
                    target:"clipcontex::commands",
                    "Autostart already in correct state → enabled={}",
                    settings.is_autostart_enabled
                );
            }
        }
        Err(e) => error!(target:"clipcontex::commands","Failed to query autostart status: {}", e),
    }

    // Update memory state
    {
        let mut guard = app_state.settings.write().unwrap();
        *guard = settings.clone();
    }

    // Notify frontend (reactive updates)
    if let Err(e) = app_handle.emit(EVT_SETTINGS_UPDATED, &settings) {
        error!(
            target: "clipcontex::commands",
            "Failed to emit settings-updated event in save_config: {}",
            e
        );
    }

    // Free unused heap pages
    #[cfg(target_os = "linux")]
    crate::malloc_trim_support::trim();

    Ok("success")
}

#[command]
pub async fn complete_onboarding(
    _app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<&str, String> {
    {
        let mut settings = app_state.settings.write().unwrap();
        settings.is_new_user = false;

        if let Err(e) = save_settings(&settings) {
            return Err(format!("Failed to save onboarding completion: {}", e));
        }
    }

    #[cfg(target_os = "linux")]
    crate::malloc_trim_support::trim();

    info!(target:"clipcontex::commands","Onboarding completed and memory trimmed.");
    Ok("success")
}

/// Check whether kdotool is installed or not
#[command]
pub async fn is_kdotool_installed() -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        match Command::new("kdotool").arg("--version").output() {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(e.to_string()),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}
