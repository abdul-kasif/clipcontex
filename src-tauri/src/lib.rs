use std::thread;
use tauri::{Emitter, Manager};
use tracing::{error, info};

// use tauri_plugin_global_shortcut::{GlobalShortcut, Shortcut};

pub mod clipboard;
pub mod commands;
pub mod context;
pub mod storage;

use crate::{
    clipboard::watcher::ClipboardWatcher,
    commands::AppState,
    context::{extract_project_from_title, generate_auto_tags, get_active_app_info},
    storage::Clip,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tauri::Builder::default()
        // Initialize required Tauri plugins
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_state = AppState::new();
            let clip_store = app_state.clip_store.clone();
            let watcher_handle_ref = app_state.watcher_handle.clone();

            let app_handle = app.handle().clone();
            let app_handle_for_shortcut = app_handle.clone();
            let app_handle_for_thread = app_handle.clone();

            app.manage(app_state);

            #[cfg(desktop)]
            {
                use tauri::async_runtime::spawn;
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let app_handle_clone = app_handle_for_shortcut.clone();
                let ctrl_n_shortcut =
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            if shortcut == &ctrl_n_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        let app_handle = app_handle_clone.clone();
                                        spawn(async move {
                                            if let Some(picker_window) =
                                                app_handle.get_webview_window("quick-picker")
                                            {
                                                match picker_window.is_visible() {
                                                    Ok(true) => {
                                                        // already visible, just focus
                                                        if let Err(e) = picker_window.set_focus() {
                                                            error!(
                                                                "Failed to focus quick picker: {}",
                                                                e
                                                            );
                                                        }
                                                    }
                                                    Ok(false) | Err(_) => {
                                                        // hidden or error, show + focus
                                                        if let Err(e) = picker_window.show() {
                                                            error!(
                                                                "Failed to show picker window: {}",
                                                                e
                                                            );
                                                            return;
                                                        }
                                                        thread::sleep(
                                                            std::time::Duration::from_millis(60),
                                                        );
                                                        if let Err(e) = picker_window.set_focus() {
                                                            error!(
                                                                "Failed to focus quick picker: {}",
                                                                e
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                error!("Quick Picker window not found!");
                                            }
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        })
                        .build(),
                )?;
                app.global_shortcut().register(ctrl_n_shortcut)?;
            }

            // Spawn the clipboard watcher thread
            std::thread::spawn(move || {
                let mut watcher = ClipboardWatcher::new();

                // pass app_handle into watcher (new version supports this)
                let handle = watcher.start(app_handle_for_thread.clone(), move |event| {
                    let content = event.content.trim().to_string();
                    if content.is_empty() {
                        return;
                    }

                    let app_info = get_active_app_info();
                    let project_name = extract_project_from_title(&app_info.window_title);
                    let auto_tags = generate_auto_tags(
                        &content,
                        project_name.as_deref(),
                        Some(&app_info.app_class),
                    );

                    let clip = Clip::new(
                        content.clone(),
                        app_info.app_class,
                        app_info.window_title,
                        auto_tags,
                        vec![],
                        false,
                    );

                    match clip_store.save_clip(&clip) {
                        Ok(saved_clip) => {
                            info!("Captured new clip automatically: {}", saved_clip.content);
                            if let Err(e) = app_handle.emit("clip-added", &saved_clip) {
                                error!("Failed to emit clip-added event: {}", e);
                            }
                        }
                        Err(e) => error!("Failed to save clip: {}", e),
                    }
                });

                *watcher_handle_ref.lock().unwrap() = Some(handle);
                info!("Clipboard watcher started successfully.");
            });

            Ok(())
        })
        // All invoke handlers remain the same
        .invoke_handler(tauri::generate_handler![
            commands::get_recent_clips,
            commands::search_clips,
            commands::clear_history,
            commands::delete_clip,
            commands::pin_clip,
            commands::capture_current_clip,
            commands::ignore_next_clipboard_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
