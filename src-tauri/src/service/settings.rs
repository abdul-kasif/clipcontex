// src-tauri/src/service/settings.rs
//! Service layer for managing user settings and related side effects.
//!
//! This module handles:
//! - Loading/saving configuration from disk.
//! - Updating global shortcuts when keybindings change.
//! - Syncing autostart preference with the OS.
//! - Managing onboarding state.

use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tracing::info;

use crate::{
    config::{load_config, save_config, Settings},
    core::global_shortcut::shortcut_from_config,
    error::AppError,
    state::AppState,
};

/// Loads user settings from the configuration file.
///
/// If the file is missing or invalid, returns default settings and creates the file.
///
/// # Errors
///
/// Returns [`AppError::Config`] if I/O or deserialization fails.
pub fn load_settings() -> Result<Settings, AppError> {
    load_config().map_err(|e| AppError::Config(e.to_string()))
}

/// Updates application settings and applies side effects.
///
/// Side effects include:
/// - Re-registering the global shortcut if it changed.
/// - Enabling/disabling OS autostart.
/// - Updating in-memory state.
///
/// # Errors
///
/// May return [`AppError::Config`], [`AppError::Shortcut`], or [`AppError::Core`]
/// depending on the failure point.
pub fn update_settings(
    app_handle: &AppHandle,
    app_state: &AppState,
    settings: &Settings,
) -> Result<(), AppError> {
    let old_settings = read_settings(app_state)?;

    // Update global shortcut if changed
    if old_settings.quick_picker_shortcut != settings.quick_picker_shortcut {
        let old_shortcut = shortcut_from_config(&old_settings.quick_picker_shortcut);
        let new_shortcut = shortcut_from_config(&settings.quick_picker_shortcut)
            .ok_or_else(|| AppError::Shortcut("Invalid shortcut configuration".into()))?;

        update_quick_picker_shortcut(app_handle, app_state, old_shortcut, new_shortcut)?;
    }

    // Persist to disk
    save_config(settings).map_err(|e| AppError::Config(e.to_string()))?;

    // Sync autostart
    sync_autostart(app_handle, settings.is_autostart_enabled)?;

    // Update in-memory state
    write_settings(app_state, settings.clone())?;

    Ok(())
}

/// Marks the onboarding flow as complete.
///
/// Sets `is_new_user = false` in both memory and config file.
///
/// # Errors
///
/// Returns [`AppError::Config`] if the settings lock is poisoned or saving fails.
pub fn mark_onboarding_complete(app_state: &AppState) -> Result<(), AppError> {
    let mut settings = app_state
        .settings
        .write()
        .map_err(|_| AppError::Config("Failed to acquire write lock on settings".to_string()))?;

    if settings.is_new_user {
        settings.is_new_user = false;
        save_config(&settings).map_err(|e| AppError::Config(e.to_string()))?;
    }

    Ok(())
}

// ===== Helper Functions =====

fn update_quick_picker_shortcut(
    app_handle: &AppHandle,
    app_state: &AppState,
    old_shortcut: Option<Shortcut>,
    new_shortcut: Shortcut,
) -> Result<(), AppError> {
    // Unregister old shortcut if exists
    if let Some(old) = old_shortcut {
        app_handle
            .global_shortcut()
            .unregister(old)
            .map_err(|e| AppError::Shortcut(format!("Failed to unregister old shortcut: {}", e)))?;
    }

    // Register new shortcut
    app_handle
        .global_shortcut()
        .register(new_shortcut)
        .map_err(|e| AppError::Shortcut(format!("Failed to register new shortcut: {}", e)))?;

    // Update in-memory reference
    app_state
        .quick_picker_shortcut
        .write()
        .map(|mut guard| *guard = Some(new_shortcut))
        .map_err(|_| {
            AppError::Shortcut("Failed to update in-memory shortcut (lock poisoned)".to_string())
        })?;

    Ok(())
}

fn sync_autostart(app_handle: &AppHandle, enabled: bool) -> Result<(), AppError> {
    let launcher = app_handle.autolaunch();
    let current = launcher
        .is_enabled()
        .map_err(|e| AppError::Config(format!("Failed to check autostart status: {}", e)))?;

    match (current, enabled) {
        (false, true) => {
            launcher
                .enable()
                .map_err(|e| AppError::Config(format!("Failed to enable autostart: {}", e)))?;
            info!("Autostart enabled");
        }
        (true, false) => {
            launcher
                .disable()
                .map_err(|e| AppError::Config(format!("Failed to disable autostart: {}", e)))?;
            info!("Autostart disabled");
        }
        _ => {
            info!("Autostart already in desired state");
        }
    }
    Ok(())
}

fn read_settings(app_state: &AppState) -> Result<Settings, AppError> {
    app_state
        .settings
        .read()
        .map(|guard| guard.clone())
        .map_err(|_| AppError::Config("Failed to acquire read lock on settings".to_string()))
}

fn write_settings(app_state: &AppState, new_settings: Settings) -> Result<(), AppError> {
    app_state
        .settings
        .write()
        .map(|mut guard| *guard = new_settings)
        .map_err(|_| AppError::Config("Failed to acquire write lock on settings".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ShortcutConfig;
    use crate::state::AppState;
    use crate::storage::ClipStore;
    use std::sync::{atomic::AtomicBool, Arc, Mutex, RwLock};

    #[test]
    fn test_read_write_settings() {
        let initial = Settings::default();
        let app_state = AppState {
            watcher_handle: Arc::new(Mutex::new(None)),
            settings: Arc::new(RwLock::new(initial.clone())),
            quick_picker_shortcut: Arc::new(RwLock::new(None)),
            clip_store: Arc::new(ClipStore::new(":memory:").expect("In-memory store")),
            is_quick_picker_dragging: Arc::new(AtomicBool::new(false)),
        };

        let read = read_settings(&app_state).unwrap();
        assert_eq!(read, initial);

        let mut updated = initial;
        updated.is_new_user = false;
        write_settings(&app_state, updated.clone()).unwrap();

        let read_again = read_settings(&app_state).unwrap();
        assert_eq!(read_again, updated);
    }

    #[test]
    fn test_shortcut_comparison() {
        let sc1 = ShortcutConfig {
            modifiers: vec!["Ctrl".into()],
            key: "v".into(),
        };
        let sc2 = ShortcutConfig {
            modifiers: vec!["Ctrl".into()],
            key: "c".into(),
        };
        assert_ne!(sc1, sc2);
    }
}
