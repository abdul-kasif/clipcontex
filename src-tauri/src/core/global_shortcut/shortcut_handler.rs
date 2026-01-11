use std::sync::{Arc, RwLock};
use tracing::error;

use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

use crate::core::window_creation;
pub fn handle_quick_picker_shortcut(
    app_handle: &tauri::AppHandle,
    quick_picker_shortcut_arc: Arc<RwLock<Option<Shortcut>>>,
) -> anyhow::Result<()> {
    app_handle.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, shortcut, event| {
                if !matches!(event.state(), ShortcutState::Pressed) {
                    return;
                }

                let current_shortcut = match quick_picker_shortcut_arc.read() {
                    Ok(guard) => guard.as_ref().cloned(),
                    Err(e) => {
                        error!("RwLock poisoned: {}", e);
                        return;
                    }
                };

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
