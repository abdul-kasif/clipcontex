// src-tauri/src/service/clip.rs
//! Service layer for clipboard history operations.
//!
//! These functions act as adapters between the application state (`AppState`)
//! and the persistence layer (`ClipStore`), translating SQLite errors into
//! application-level `AppError`s.

use crate::{command::AppState, error::AppError, storage::Clip};

/// Retrieves the most recent clips from storage, up to the specified limit.
///
/// # Errors
///
/// Returns a [`AppError::Storage`] if the database query fails.
pub fn list_recent_clips(app_state: &AppState, limit: i32) -> Result<Vec<Clip>, AppError> {
    app_state
        .clip_store
        .list_recent_clips(limit)
        .map_err(|e| AppError::Storage(e.to_string()))
}

/// Toggles the pinned status of a clip by ID.
///
/// # Errors
///
/// Returns a [`AppError::Storage`] if the update fails (e.g., invalid ID).
pub fn toggle_pin_status(app_state: &AppState, id: i32, is_pinned: bool) -> Result<(), AppError> {
    app_state
        .clip_store
        .toggle_pin_status(id, is_pinned)
        .map_err(|e| AppError::Storage(e.to_string()))
}

/// Deletes a clip by its database ID.
///
/// Silently succeeds if the ID does not exist.
///
/// # Errors
///
/// Returns a [`AppError::Storage`] only if the database operation fails.
pub fn remove_clip(app_state: &AppState, id: i32) -> Result<(), AppError> {
    app_state
        .clip_store
        .remove_clip(id)
        .map_err(|e| AppError::Storage(e.to_string()))
}

/// Clears all clips from the history.
///
/// This operation cannot be undone.
///
/// # Errors
///
/// Returns a [`AppError::Storage`] if the deletion fails.
pub fn clear_clip_history(app_state: &AppState) -> Result<(), AppError> {
    app_state
        .clip_store
        .clear_clip_history()
        .map_err(|e| AppError::Storage(e.to_string()))
}
