// src-tauri/src/commands.rs
// ===== Imports =====
use crate::{
    clipboard::watcher::{mark_ignore_next_clipboard_update, ClipboardWatcherHandle},
    config::{load_settings, save_settings, Settings},
    storage::{Clip, ClipStore},
};
use std::{
    fmt::Display,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex, RwLock},
};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_autostart::ManagerExt;
use tracing::{error, info, warn};

// ===== Event Constants =====
const EVT_CLIP_UPDATED: &str = "clip-updated";
const EVT_CLIP_DELETED: &str = "clip-deleted";
const EVT_HISTORY_CLEARED: &str = "history-cleared";
const EVT_SETTINGS_UPDATED: &str = "settings-updated";

// ===== Domain Types =====
#[derive(Clone)]
pub struct AppState {
    pub clip_store: Arc<ClipStore>,
    pub watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
    pub settings: Arc<RwLock<Settings>>,
}

// ===== AppState Implementation =====
impl AppState {
    pub fn new() -> Self {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".clipcontex/clipcontex.db");

        let store = ClipStore::new(&db_path).unwrap_or_else(|e| {
            error!("Failed to initialize ClipStore DB at {:?}: {}", db_path, e);
            panic!("Failed to initialize database: {}", e);
        });

        info!("ClipStore initialized at {:?}", db_path);

        let settings = match load_settings() {
            Ok(s) => s,
            Err(e) => {
                warn!("failed to load settings, using defaults: {}", e);
                Settings::default()
            }
        };

        Self {
            clip_store: Arc::new(store),
            watcher_handle: Arc::new(Mutex::new(None)),
            settings: Arc::new(RwLock::new(settings)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Commands =====
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

    if let Err(e) = app_handle.emit(EVT_CLIP_UPDATED, &(id, is_pinned)) {
        error!(
            "Failed to emit pin_clip event '{}': {}",
            EVT_CLIP_UPDATED, e
        );
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

    if let Err(e) = app_handle.emit(EVT_CLIP_DELETED, &id) {
        error!(
            "Failed to emit delete_clip event '{}': {}",
            EVT_CLIP_DELETED, e
        );
    }

    Ok(())
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
        error!(
            "Failed to emit clear_history event '{}': {}",
            EVT_HISTORY_CLEARED, e
        );
    }

    Ok(())
}

#[command]
pub async fn load_config() -> Result<Settings, String> {
    load_settings().map_err(|e| err("Failed to read config", e))
}

#[command]
pub async fn save_config(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    settings: Settings,
) -> Result<&str, String> {
    if let Err(e) = save_settings(&settings) {
        return Err(err("Failed to save settings", e));
    }

    sync_autostart(&app_handle, settings.is_autostart_enabled);

    {
        let mut guard = app_state.settings.write().unwrap();
        *guard = settings.clone();
    }

    if let Err(e) = app_handle.emit(EVT_SETTINGS_UPDATED, &settings) {
        error!(
            "Failed to emit settings-updated event in save_config: {}",
            e
        );
    }

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
            return Err(err("Failed to complete onboarding", e));
        }
    }
    info!("Onboarding completed.");
    Ok("success")
}

#[command]
pub async fn ignore_next_clipboard_update(content: String) {
    mark_ignore_next_clipboard_update(content);
}

#[command]
pub async fn is_kdotool_installed() -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        match Command::new("kdotool").arg("--version").output() {
            Ok(output) => Ok(output.status.success()),
            Err(e) => Err(err("Failed to execute 'kdotool' command", e)),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

// ===== Helper function =====
fn sync_autostart(app_handle: &AppHandle, enabled: bool) {
    let launcher = app_handle.autolaunch();

    match launcher.is_enabled() {
        Ok(current_status) if enabled && !current_status => {
            if let Err(e) = launcher.enable() {
                err("Failed to enable autostart", e);
            }
        }

        Ok(current_status) if !enabled && current_status => {
            if let Err(e) = launcher.disable() {
                err("failed to disable autostart", e);
            }
        }

        Ok(_) => {
            info!("Autostart already in desired state");
        }

        Err(e) => error!("failed to query autostart status: {}", e),
    }
}

fn err<E: Display>(context: &str, e: E) -> String {
    format!("{}: {}", context, e)
}
