use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    process::Command,
};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing::{debug, error, info, warn};

use crate::{
    clipboard::watcher::{mark_ignore_next_clipboard_update, ClipboardWatcherHandle},
    config::{load_settings, save_settings, Settings as ConfigSettings},
    context::{extract_project_from_title, generate_auto_tags, get_active_app_info},
    storage::{Clip, ClipStore},
};

/// Event constants for consistency
const EVT_CLIP_ADDED: &str = "clip-added";
const EVT_CLIP_UPDATED: &str = "clip-updated";
const EVT_CLIP_DELETED: &str = "clip-deleted";
const EVT_HISTORY_CLEARED: &str = "history-cleared";
const EVT_SETTINGS_UPDATED: &str = "settings-updated";

#[derive(Clone)]
/// Shared global application state
pub struct AppState {
    pub clip_store: Arc<ClipStore>,
    pub watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
    pub settings: Arc<Mutex<ConfigSettings>>,
}

impl AppState {
    /// Initializes persistent ClipStore under `~/.clipcontex/clipcontex.db`
    pub fn new() -> Self {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".clipcontex/clipcontex.db");

        let store = ClipStore::new(&db_path);
        if let Err(e) = store.init() {
            error!("Failed to initialize ClipStore DB at {:?}: {}", db_path, e);
            panic!("Failed to initialize database: {}", e);
        }

        info!("ClipStore initialized at {:?}", db_path);

        let settings = match load_settings() {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to load settings, using defaults: {}", e);
                ConfigSettings::default()
            }
        };
        Self {
            clip_store: Arc::new(store),
            watcher_handle: Arc::new(Mutex::new(None)),
            settings: Arc::new(Mutex::new(settings)),
        }
    }
}

/// Helper for consistent string error formatting
fn err<E: std::fmt::Display>(context: &str, e: E) -> String {
    format!("{}: {}", context, e)
}

/// Capture current clipboard content (using tauri-plugin-clipboard-manager)

#[command]
pub async fn capture_current_clip(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<Clip, String> {
    debug!("Attempting to capture clipboard content...");

    // Read clipboard text safely
    let text_result = app_handle
        .clipboard()
        .read_text()
        .map_err(|e| err("Clipboard read failed", e))?;

    let clipboard_text = text_result.trim();
    if clipboard_text.is_empty() {
        return Err("Clipboard is empty".to_string());
    }

    // Gather active app context
    let app_info = get_active_app_info();
    let project_name = extract_project_from_title(&app_info.window_title);
    debug!(
        "Active app detected: class='{}', title='{}'",
        app_info.app_class, app_info.window_title
    );

    // Auto-generate tags
    let auto_tags = generate_auto_tags(
        clipboard_text,
        project_name.as_deref(),
        Some(&app_info.app_class),
    );

    // Construct Clip model
    let clip = Clip::new(
        clipboard_text.to_string(),
        app_info.app_class,
        app_info.window_title,
        auto_tags,
        vec![],
        false,
    );

    // Persist the new clip
    let saved_clip = app_state
        .clip_store
        .save_clip(&clip)
        .map_err(|e| err("Failed to save clip", e))?;

    info!("Saved new clip ({} bytes)", saved_clip.content.len());

    // Emit event for frontend update
    if let Err(e) = app_handle.emit(EVT_CLIP_ADDED, &saved_clip) {
        warn!("Failed to emit '{}': {}", EVT_CLIP_ADDED, e);
    } else {
        debug!("Emitted '{}' successfully", EVT_CLIP_ADDED);
    }

    Ok(saved_clip)
}

#[command]
pub async fn ignore_next_clipboard_update() {
    mark_ignore_next_clipboard_update();
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
        error!("Failed to emit '{}': {}", EVT_HISTORY_CLEARED, e);
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
        error!("Failed to emit '{}': {}", EVT_CLIP_DELETED, e);
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

    if let Err(e) = app_handle.emit(EVT_CLIP_UPDATED, &(id, is_pinned)) {
        error!("Failed to emit '{}': {}", EVT_CLIP_UPDATED, e);
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
) -> Result<(), String> {
    match save_settings(&settings) {
        Ok(_) => {
            // update in-memory
            {
                let mut guard = app_state.settings.lock().unwrap();
                *guard = settings.clone();
            }
            if let Err(e) = app_handle.emit(EVT_SETTINGS_UPDATED, &settings) {
                warn!("Failed to emit settings-updated: {}", e);
            }
            Ok(())
        }
        Err(e) => Err(format!("Failed to save config: {}", e)),
    }
}

#[command]
pub async fn is_kdotool_installed() -> Result<bool, String> {
    match Command::new("kdotool").arg("--version").output() {
        Ok(output) => Ok(output.status.success()),
        Err(e) => Err(e.to_string()),
    }
}
