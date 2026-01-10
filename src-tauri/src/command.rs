// src-tauri/src/commands.rs
// ===== Imports =====
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_global_shortcut::Shortcut;
use tracing::{error, info, warn};

// ===== Crates =====
use crate::{
    clipboard::watcher::{mark_ignore_next_clipboard_update, ClipboardWatcherHandle},
    config::{load_config, Settings},
    core::global_shortcut::shortcut_from_config,
    error::AppError,
    service::{clip, settings, system},
    storage::{Clip, ClipStore},
};

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
    pub quick_picker_shortcut: Arc<RwLock<Option<Shortcut>>>,
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

        let settings = match load_config() {
            Ok(s) => s,
            Err(e) => {
                warn!("failed to load settings, using defaults: {}", e);
                Settings::default()
            }
        };

        let initial_shortcut = shortcut_from_config(&settings.quick_picker_shortcut);
        Self {
            clip_store: Arc::new(store),
            watcher_handle: Arc::new(Mutex::new(None)),
            settings: Arc::new(RwLock::new(settings)),
            quick_picker_shortcut: Arc::new(RwLock::new(initial_shortcut)),
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
pub async fn list_recent_clips(
    app_state: State<'_, AppState>,
    limit: i32,
) -> Result<Vec<Clip>, String> {
    ipc(clip::list_recent_clips(app_state.inner(), limit))
}

#[command]
pub async fn toggle_pin_status(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    id: i32,
    is_pinned: bool,
) -> Result<(), String> {
    ipc(clip::toggle_pin_status(app_state.inner(), id, is_pinned))?;
    if let Err(e) = app_handle.emit(EVT_CLIP_UPDATED, &(id, is_pinned)) {
        error!(
            "Failed to emit pin_clip event '{}': {}",
            EVT_CLIP_UPDATED, e
        );
    }
    Ok(())
}

#[command]
pub async fn remove_clip(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    id: i32,
) -> Result<(), String> {
    ipc(clip::remove_clip(app_state.inner(), id))?;

    if let Err(e) = app_handle.emit(EVT_CLIP_DELETED, &id) {
        error!(
            "Failed to emit delete_clip event '{}': {}",
            EVT_CLIP_DELETED, e
        );
    }

    Ok(())
}

#[command]
pub async fn clear_clip_history(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    ipc(clip::clear_clip_history(app_state.inner()))?;

    if let Err(e) = app_handle.emit(EVT_HISTORY_CLEARED, ()) {
        error!(
            "Failed to emit clear_history event '{}': {}",
            EVT_HISTORY_CLEARED, e
        );
    }

    Ok(())
}

#[command]
pub async fn ignore_next_clip(content: String) {
    mark_ignore_next_clipboard_update(content);
}

#[command]
pub async fn load_settings() -> Result<Settings, String> {
    ipc(settings::load_settings())
}

#[command]
pub async fn save_settings(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    settings: Settings,
) -> Result<&str, String> {
    ipc(settings::update_settings(
        &app_handle,
        app_state.inner(),
        &settings,
    ))?;
    if let Err(e) = app_handle.emit(EVT_SETTINGS_UPDATED, settings) {
        error!(
            "Failed to emit settings saved event '{}': {}",
            EVT_SETTINGS_UPDATED, e
        );
    }

    Ok("success")
}

#[command]
pub async fn mark_onboarding_complete(
    _app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<&str, String> {
    ipc(settings::mark_onboarding_complete(app_state.inner()))?;
    Ok("success")
}

#[command]
pub async fn check_kdotool_installed() -> Result<bool, String> {
    ipc(system::check_kdotool_installed())
}

// ===== Helper Functions =====
fn ipc<T>(res: Result<T, AppError>) -> Result<T, String> {
    res.map_err(|e| e.to_string())
}
