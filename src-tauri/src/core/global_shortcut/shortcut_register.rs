use anyhow::Result;
use std::sync::{Arc, RwLock};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tracing::{error, info};

/// Registers the current quick picker global shortcut from app state.
pub fn register_quick_picker_shortcut(
    app_handle: &tauri::AppHandle,
    quick_picker_shortcut_arc: Arc<RwLock<Option<Shortcut>>>,
) -> Result<()> {
    let shortcut_opt = match quick_picker_shortcut_arc.read() {
        Ok(guard) => guard.as_ref().cloned(), // or (*guard).clone(), or guard.as_ref().cloned()
        Err(e) => {
            error!("RwLock poisoned: {}", e);
            None
        }
    };

    if let Some(shortcut) = shortcut_opt {
        app_handle.global_shortcut().register(shortcut)?;
        info!("shortcut registered successfully: {}", shortcut);
    }

    Ok(())
}
