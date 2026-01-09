use crate::commands::AppState;
use tauri::Manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

pub fn register_quick_picker_shortcut(app: &tauri::App) -> anyhow::Result<()> {
    let state = app.state::<AppState>();

    if let Some(shortcut) = state.quick_picker_shortcut.read().unwrap().clone() {
        app.global_shortcut().register(shortcut)?;
    }

    Ok(())
}
