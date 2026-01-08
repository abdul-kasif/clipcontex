// ===== Imports =====
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// ===== Crates =====
use crate::core::window_creation;

// ===== Public API =====
pub fn register_quick_picker_shortcut(app: &tauri::App) -> anyhow::Result<()> {
    let quick_picker_shortcut =
        Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

    let app_handle = app.handle().clone();
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                if shortcut == &quick_picker_shortcut
                    && matches!(event.state(), ShortcutState::Pressed)
                {
                    window_creation::hide_and_show_quick_picker_window(&app_handle);
                }
            })
            .build(),
    )?;

    app.global_shortcut().register(quick_picker_shortcut)?;
    tracing::info!("Registered Ctrl+Shift+V for Quick Picker");
    Ok(())
}
