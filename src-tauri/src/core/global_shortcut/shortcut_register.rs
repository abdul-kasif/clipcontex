// src-tauri/src/core/global_shortcut/shortcut_register.rs
//! Global shortcut registration logic.
//!
//! This module handles the one-time registration of the quick picker global shortcut
//! with the operating system. It reads the current shortcut from application state
//! and registers it via Tauri's global shortcut plugin.
//!
//! Registration occurs during app setup and is separate from event handling

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tracing::{error, info};

/// Registers the current quick picker global shortcut with the OS.
///
/// Reads the shortcut from shared application state (`quick_picker_shortcut_arc`)
/// and registers it using Tauri's global shortcut plugin.
///
/// If the lock is poisoned, logs an error and skips registration.
/// If the shortcut is `None`, does nothing.
///
/// # Errors
///
/// Returns an error if the OS fails to register the shortcut
/// (e.g., due to permission issues or conflicting shortcuts).
pub fn register_quick_picker_shortcut(
    app_handle: &tauri::AppHandle,
    quick_picker_shortcut_arc: Arc<RwLock<Option<Shortcut>>>,
) -> Result<()> {
    let shortcut_opt = match quick_picker_shortcut_arc.read() {
        Ok(guard) => guard.as_ref().cloned(),
        Err(e) => {
            error!("RwLock poisoned while reading shortcut: {}", e);
            None
        }
    };

    if let Some(shortcut) = shortcut_opt {
        app_handle.global_shortcut().register(shortcut)?;
        info!(
            "Quick picker shortcut registered successfully: {}",
            shortcut
        );
    }

    Ok(())
}
