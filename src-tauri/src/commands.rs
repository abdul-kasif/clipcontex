use crate::clipboard::watcher::ClipboardWatcherHandle;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{command, AppHandle, Emitter, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing::{error, info};

use crate::{
    clipboard::watcher::mark_ignore_next_clipboard_update,
    context::{extract_project_from_title, generate_auto_tags, get_active_app_info},
    storage::{Clip, ClipStore},
};

/// Shared global state for ClipStore access
pub struct AppState {
    pub clip_store: Arc<ClipStore>,
    pub watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
}

impl AppState {
    /// Initializes the global state with a persistent database under `~/.clipcontex/clipcontex.db`
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

        Self {
            clip_store: Arc::new(store),
            watcher_handle: Arc::new(Mutex::new(None)), // initialize the watcher handle
        }
    }
}

/// Helper for consistent string error formatting.
pub fn err<E: std::fmt::Display>(context: &str, e: E) -> String {
    format!("{}: {}", context, e)
}

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
    if let Err(e) = app_handle.emit("history-cleared", ()) {
        error!("Failed to emit histroy-cleard event: {}", e);
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
    if let Err(e) = app_handle.emit("clip-deleted", &id) {
        error!("Failed to emit clip-deleted event: {}", e);
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
    if let Err(e) = app_handle.emit("clip-updated", &(id, is_pinned)) {
        error!("Failed to emit clip-updated event: {}", e);
    }

    Ok(())
}

/// Capture current clipboard content (using tauri-plugin-clipboard-manager)
#[command]
pub async fn capture_current_clip(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<Clip, String> {
    tracing::debug!("Attempting to capture clipboard content...");

    // Read clipboard text safely
    let text_result = app_handle
        .clipboard()
        .read_text()
        .map_err(|e| format!("Clipboard read failed: {}", e))?;

    let clipboard_text = text_result.trim();

    if clipboard_text.is_empty() {
        return Err("Clipboard is empty".to_string());
    }

    // Gather active app context
    let app_info = get_active_app_info();
    let project_name = extract_project_from_title(&app_info.window_title);
    tracing::debug!(
        "Active app detected: class='{}', title='{}'",
        app_info.app_class,
        app_info.window_title
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
    let saved_clip = match app_state.clip_store.save_clip(&clip) {
        Ok(saved) => {
            tracing::info!("Saved new clip ({} bytes)", saved.content.len());
            saved
        }
        Err(e) => {
            tracing::error!(" Failed to save clip: {}", e);
            return Err(format!("Failed to save clip: {}", e));
        }
    };

    // Emit event for frontend update
    if let Err(e) = app_handle.emit("clip-added", &saved_clip) {
        tracing::warn!("Failed to emit 'clip-added' event: {}", e);
    } else {
        tracing::debug!("Emitted 'clip-added' event successfully");
    }

    Ok(saved_clip)
}

/// Calling the IGNORE flag to ignore the clipboard update while using quick picker
#[tauri::command]
pub async fn ignore_next_clipboard_update() {
    mark_ignore_next_clipboard_update();
}
