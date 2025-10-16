use std::{path::PathBuf, process::Command};
use tauri::{command, AppHandle, State, Emitter};
use tracing::{error, info};
use std::sync::{Arc, Mutex};
use crate::clipboard::watcher::ClipboardWatcherHandle;

use crate::{
    context::{extract_project_from_title, generate_auto_tags, get_active_app_info},
    storage::{Clip, ClipStore},
};

/// Shared global state for ClipStore access
pub struct AppState {
    pub clip_store: Arc<ClipStore>,
    pub watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
}

impl AppState {
    /// Initializes the global state with a persistent database under `~/.clipcontext/clipcontext.db`
    pub fn new() -> Self {
        let db_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".clipcontext/clipcontext.db");

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

/// Capture current clipboard content (Wayland/X11 compatible)
#[command]
pub async fn capture_current_clip(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<Clip, String> {
    // Run in background thread (blocking clipboard I/O)
    let clipboard_text = tauri::async_runtime::spawn_blocking(|| {
        #[cfg(target_os = "linux")]
        {
            // Check if Wayland is active
            let is_wayland = std::env::var("WAYLAND_DISPLAY").is_ok();
            if is_wayland {
                // Use wl-paste for Wayland
                match Command::new("wl-paste").arg("--no-newline").output() {
                    Ok(output) if !output.stdout.is_empty() => {
                        let text = String::from_utf8_lossy(&output.stdout).to_string();
                        if text.trim().is_empty() {
                            Err("Clipboard is empty (Wayland)".to_string())
                        } else {
                            Ok(text)
                        }
                    }
                    Ok(_) => Err("Clipboard is empty (Wayland)".to_string()),
                    Err(_) => {
                        // Fallback to arboard if wl-paste missing
                        let mut clipboard = arboard::Clipboard::new()
                            .map_err(|e| err("Failed to access clipboard", e))?;
                        clipboard
                            .get_text()
                            .map_err(|e| err("Failed to read clipboard text", e))
                    }
                }
            } else {
                // X11 path — arboard should work fine
                let mut clipboard =
                    arboard::Clipboard::new().map_err(|e| err("Failed to access clipboard", e))?;
                clipboard
                    .get_text()
                    .map_err(|e| err("Failed to read clipboard text", e))
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            // For macOS and Windows — arboard is reliable
            let mut clipboard =
                arboard::Clipboard::new().map_err(|e| err("Failed to access clipboard", e))?;
            clipboard
                .get_text()
                .map_err(|e| err("Failed to read clipboard text", e))
        }
    })
    .await
    .map_err(|e| err("Clipboard task failed", e))??;

    if clipboard_text.trim().is_empty() {
        return Err("Clipboard is empty".to_string());
    }

    // Get contextual info from active window
    let app_info = get_active_app_info();
    let project_name = extract_project_from_title(&app_info.window_title);

    // Auto-generate tags
    let auto_tags = generate_auto_tags(&clipboard_text, project_name.as_deref());

    // Build clip object
    let clip = Clip::new(
        clipboard_text.clone(),
        app_info.app_class,
        app_info.window_title,
        auto_tags,
        vec![],
        false,
    );

    // Save to DB
    let saved = app_state
        .clip_store
        .save_clip(&clip)
        .map_err(|e| err("Failed to save clip", e))?;

    // Emit event to notify frontend about the new clip
    let _ = app_handle.emit("clip-added", &saved).map_err(|e| {
        error!("Failed to emit clip-added event: {}", e);
    });

    // Log result
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
