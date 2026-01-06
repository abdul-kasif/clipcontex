// ===== Imports =====
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tokio::time::Duration;
use tracing::error;

// ===== Crates =====
use crate::core::platform;

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
                    let app_handle = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Some(window) = app_handle.get_webview_window("quick-picker") {
                            // Hide → trim → show
                            if let Err(e) = window.hide() {
                                error!("Failed to hide Quick Picker: {}", e);
                            } else {
                                platform::malloc_trim_now();
                            }

                            tokio::time::sleep(Duration::from_millis(80)).await;

                            if let Err(e) = window.show() {
                                error!("Failed to re-show Quick Picker: {}", e);
                            } else {
                                let _ = window.set_focus();
                            }

                            // Attach focus-loss handler ONCE
                            static HANDLER_ATTACHED: std::sync::Once = std::sync::Once::new();
                            HANDLER_ATTACHED.call_once(|| {
                                let win_ref = window.clone();
                                window.on_window_event(move |ev| {
                                    if let tauri::WindowEvent::Focused(false) = ev {
                                        let _ = win_ref.hide();
                                        platform::malloc_trim_now();
                                    }
                                });
                            });
                        } else {
                            error!("Quick Picker window not found!");
                        }
                    });
                }
            })
            .build(),
    )?;

    app.global_shortcut().register(quick_picker_shortcut)?;
    tracing::info!("Registered Ctrl+Shift+V for Quick Picker");
    Ok(())
}
