use std::path::PathBuf;
use tauri::{command, AppHandle, State};
use tracing::{error, info, warn};

use crate::{
    context::{extract_project_from_title, generate_auto_tags, get_active_app_info},
    storage::{Clip, ClipStore},
};

/// Shared global state for ClipStore access
pub struct AppState {
    pub clip_store: ClipStore,
}

impl AppState {
    /// Initializes the global state with a persistent database under `~/.clipcontext/clipcontext.db`
    pub fn new() -> Self {
        // Expand or fallback to a local path if $HOME is unavailable
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".clipcontext/clipcontext.db");

        let store = ClipStore::new(&db_path);
        if let Err(e) = store.init() {
            error!("Failed to initialize ClipStore DB at {:?}: {}", db_path, e);
            panic!("Failed to initialize database: {}", e);
        }

        info!("ClipStore initialized at {:?}", db_path);
        Self { clip_store: store }
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
pub async fn search_clips(
    app_state: State<'_, AppState>,
    query: String,
    limit: i32,
) -> Result<Vec<Clip>, String> {
    app_state
        .clip_store
        .search_clips(&query, limit)
        .map_err(|e| err("Search failed", e))
}

#[command]
pub async fn clear_history(app_state: State<'_, AppState>) -> Result<(), String> {
    app_state
        .clip_store
        .clear_history()
        .map_err(|e| err("Failed to clear history", e))
}

#[command]
pub async fn delete_clip(app_state: State<'_, AppState>, id: i32) -> Result<(), String> {
    app_state
        .clip_store
        .delete_clip(id)
        .map_err(|e| err("Failed to delete clip", e))
}

#[command]
pub async fn pin_clip(
    app_state: State<'_, AppState>,
    id: i32,
    is_pinned: bool,
) -> Result<(), String> {
    app_state
        .clip_store
        .set_pin_status(id, is_pinned)
        .map_err(|e| err("Failed to update pin status", e))
}

/// Captures current clipboard content along with context and saves it.
#[command]
pub async fn capture_current_clip(
    _app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<Clip, String> {
    // Offload blocking clipboard access to background thread
    let clipboard_text = tauri::async_runtime::spawn_blocking(|| {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| err("Failed to access clipboard", e))?;
        clipboard
            .get_text()
            .map_err(|e| err("Failed to read clipboard text", e))
    })
    .await
    .map_err(|e| err("Clipboard task failed", e))??;

    if clipboard_text.trim().is_empty() {
        warn!("Clipboard is empty or whitespace-only, skipping capture.");
        return Err("Clipboard is empty".to_string());
    }

    // Get active app context
    let app_info = get_active_app_info();
    let project_name = extract_project_from_title(&app_info.window_title);

    // Auto-generate tags based on clipboard + project
    let auto_tags = generate_auto_tags(&clipboard_text, project_name.as_deref());

    // Construct new clip
    let clip = Clip::new(
        clipboard_text.clone(),
        app_info.app_class,
        app_info.window_title,
        auto_tags,
        vec![],
        false,
    );

    // Save clip in DB
    let saved = app_state
        .clip_store
        .save_clip(&clip)
        .map_err(|e| err("Failed to save clip", e))?;

    // Log contextual info
    let preview = if saved.content.len() > 60 {
        format!("{}...", &saved.content[..57])
    } else {
        saved.content.clone()
    };

    info!(
        "Captured clip: id={:?}, len={}, preview=\"{}\"",
        saved.id,
        saved.content.len(),
        preview
    );

    Ok(saved)
}
