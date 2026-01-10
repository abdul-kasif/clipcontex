// ===== Crates =====
use crate::{command::AppState, error::AppError, storage::Clip};

// ===== Public API =====
pub fn list_recent_clips(app_state: &AppState, limit: i32) -> Result<Vec<Clip>, AppError> {
    app_state
        .clip_store
        .list_recent_clips(limit)
        .map_err(|e| AppError::Storage(e.to_string()))
}

pub fn toggle_pin_status(app_state: &AppState, id: i32, is_pinned: bool) -> Result<(), AppError> {
    app_state
        .clip_store
        .toggle_pin_status(id, is_pinned)
        .map_err(|e| AppError::Storage(e.to_string()))
}

pub fn remove_clip(app_state: &AppState, id: i32) -> Result<(), AppError> {
    app_state
        .clip_store
        .remove_clip(id)
        .map_err(|e| AppError::Storage(e.to_string()))
}

pub fn clear_clip_history(app_state: &AppState) -> Result<(), AppError> {
    app_state
        .clip_store
        .clear_clip_history()
        .map_err(|e| AppError::Storage(e.to_string()))
}
