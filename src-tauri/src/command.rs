// src-tauri/src/command.rs
//! Tauri IPC command handlers.
//!
//! This module defines all commands exposed to the frontend via `invoke()`.
//! Each command delegates to the service layer and may emit real-time events
//! for UI updates. Errors are converted to user-friendly strings.

use tauri::{command, AppHandle, Emitter, State};
use tracing::error;

use crate::{
    clipboard::watcher::mark_ignore_next_clipboard_update,
    config::Settings,
    error::AppError,
    service::{clip, settings, system},
    state::AppState,
    storage::Clip,
};

// ===== Event Constants =====

/// Emitted when a clip's pinned status changes.
pub const EVT_CLIP_UPDATED: &str = "clip-updated";

/// Emitted when a clip is deleted.
pub const EVT_CLIP_DELETED: &str = "clip-deleted";

/// Emitted when the entire clipboard history is cleared.
pub const EVT_HISTORY_CLEARED: &str = "history-cleared";

/// Emitted when user settings are successfully updated.
pub const EVT_SETTINGS_UPDATED: &str = "settings-updated";

// ===== Commands =====

/// Retrieves the most recent clipboard entries.
///
/// # Arguments
///
/// - `limit`: Maximum number of clips to return (e.g., 50).
///
/// # Returns
///
/// A list of [`Clip`] objects ordered from newest to oldest.
#[command]
pub async fn list_recent_clips(
    app_state: State<'_, AppState>,
    limit: i32,
) -> Result<Vec<Clip>, String> {
    ipc(clip::list_recent_clips(app_state.inner(), limit))
}

/// Toggles the pinned status of a clipboard entry.
///
/// Pinned clips are excluded from automatic cleanup.
///
/// # Arguments
///
/// - `id`: Database ID of the clip.
/// - `is_pinned`: Desired pin state.
///
/// # Events
///
/// Emits [`EVT_CLIP_UPDATED`] with `(id, is_pinned)` on success.
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

/// Deletes a clipboard entry by ID.
///
/// # Arguments
///
/// - `id`: Database ID of the clip to remove.
///
/// # Events
///
/// Emits [`EVT_CLIP_DELETED`] with the `id` on success.
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

/// Clears all clipboard history.
///
/// Cannot be undone. Pinned clips are also removed.
///
/// # Events
///
/// Emits [`EVT_HISTORY_CLEARED`] on success.
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

/// Instructs the clipboard watcher to ignore the next update with this content.
///
/// Used to prevent self-triggering when the app itself writes to the clipboard
/// (e.g., during a paste operation).
#[command]
pub async fn ignore_next_clip(content: String) {
    mark_ignore_next_clipboard_update(content);
}

/// Loads current user settings from disk.
///
/// Falls back to defaults if config is missing or invalid.
#[command]
pub async fn load_settings() -> Result<Settings, String> {
    ipc(settings::load_settings())
}

/// Saves updated user settings and applies side effects.
///
/// Side effects include:
/// - Re-registering global shortcut if changed.
/// - Enabling/disabling OS autostart.
/// - Updating in-memory state.
///
/// # Arguments
///
/// - `settings`: The new settings to persist.
///
/// # Returns
///
/// `"success"` on success.
///
/// # Events
///
/// Emits [`EVT_SETTINGS_UPDATED`] with the saved settings.
#[command]
pub async fn save_settings(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    settings: Settings,
) -> Result<&'static str, String> {
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

/// Marks the onboarding flow as complete.
///
/// Sets `is_new_user = false` in settings and persists to disk.
#[command]
pub async fn mark_onboarding_complete(
    _app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> Result<&'static str, String> {
    ipc(settings::mark_onboarding_complete(app_state.inner()))?;
    Ok("success")
}

/// Checks whether `kdotool` is installed (Linux-only).
///
/// Required for simulating keyboard input on Linux.
/// Always returns `false` on non-Linux platforms.
#[command]
pub async fn check_kdotool_installed() -> Result<bool, String> {
    ipc(system::check_kdotool_installed())
}

// ===== Helper Functions =====

/// Converts application errors to strings for IPC.
fn ipc<T>(res: Result<T, AppError>) -> Result<T, String> {
    res.map_err(|e| e.to_string())
}

