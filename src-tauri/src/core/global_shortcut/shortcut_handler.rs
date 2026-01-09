use tauri::Manager;
use tauri_plugin_global_shortcut::ShortcutState;

use crate::{commands::AppState, core::window_creation};

pub fn handle_quick_picker_shortcut(app: &tauri::App) -> anyhow::Result<()> {
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(|app, shortcut, event| {
                if !matches!(event.state(), ShortcutState::Pressed) {
                    return;
                }

                let app_state = app.state::<AppState>();
                let active = app_state.quick_picker_shortcut.read().unwrap();

                if let Some(current) = active.as_ref() {
                    if shortcut == current {
                        window_creation::hide_and_show_quick_picker_window(app);
                    }
                }
            })
            .build(),
    )?;

    Ok(())
}
