// src-tauri/src/core/global_shortcut/shortcut_handler.rs
//! Global shortcut event handling.
//!
//! This module sets up a Tauri plugin handler that listens for global shortcut
//! press events. When the configured quick picker shortcut is pressed,
//! it triggers the quick picker window toggle.
//!
//! The handler compares the triggered shortcut against the current app state
//! to support dynamic shortcut changes.

use std::sync::{Arc, RwLock};
use tracing::error;

use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

use crate::core::window_creation;

/// Installs a global shortcut event handler for the quick picker.
///
/// The handler:
/// - Listens for all global shortcut events.
/// - Filters for `Pressed` state only.
/// - Compares the triggered shortcut against the current configured shortcut.
/// - Toggles the quick picker window if they match.
///
/// Uses poison recovery on lock failure to avoid crashing the app.
///
/// # Errors
///
/// Returns an error if the plugin fails to install (should not happen under normal conditions).
pub fn handle_quick_picker_shortcut(
    app_handle: &tauri::AppHandle,
    quick_picker_shortcut_arc: Arc<RwLock<Option<Shortcut>>>,
) -> anyhow::Result<()> {
    app_handle.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, shortcut, event| {
                // Only respond to key press (not release)
                if !matches!(event.state(), ShortcutState::Pressed) {
                    return;
                }

                // Safely read current shortcut from state
                let current_shortcut = match quick_picker_shortcut_arc.read() {
                    Ok(guard) => guard.as_ref().cloned(),
                    Err(e) => {
                        error!("RwLock poisoned while handling shortcut: {}", e);
                        return;
                    }
                };

                // Trigger quick picker only if shortcuts match
                if let Some(current) = current_shortcut {
                    if *shortcut == current {
                        window_creation::hide_and_show_quick_picker_window(app_handle);
                    }
                }
            })
            .build(),
    )?;

    Ok(())
}

