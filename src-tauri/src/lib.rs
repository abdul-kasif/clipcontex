// src-tauri/src/lib.rs
use std::time::Duration;
use std::thread;

use tauri::{
    Manager,WebviewUrl,Emitter,
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::async_runtime::spawn;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tracing::{error, info};

pub mod clipboard;
pub mod commands;
pub mod config;
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
    // Logging
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .init();

    tauri::Builder::default()
        // plugins (you had these already)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // ---- Shared application state ----
            let app_state = AppState::new();
            let clip_store = app_state.clip_store.clone();
            let watcher_handle_ref = app_state.watcher_handle.clone();
            let settings_ref = app_state.settings.clone();
            let app_handle = app.handle().clone();

            // Manage app state for invocables
            app.manage(app_state);

            // Clipboard watcher: single dedicated thread
            {
                let app_handle = app_handle.clone();
                let clip_store = clip_store.clone();
                let settings_ref = settings_ref.clone();
                let watcher_handle_ref = watcher_handle_ref.clone();

                // spawn a system thread; the watcher internally spawns a loop-thread
                thread::spawn(move || {
                    // create watcher and start it; it returns a handle that owns the watcher thread
                    let mut watcher = ClipboardWatcher::new();

                    let handle = watcher.start(app_handle.clone(), move |event| {
                        let content = event.content.trim();
                        if content.is_empty() {
                            return;
                        }
                        
                        // Only process if content is substantial
                        if content.len() < 2 {
                            return;
                        }

                        // get app context and auto-tags
                        let app_info = get_active_app_info();
                        let project_name = extract_project_from_title(&app_info.window_title);
                        let auto_tags = generate_auto_tags(
                            content,
                            project_name.as_deref(),
                            Some(&app_info.app_class),
                        );

                        // check ignored apps from settings
                        let ignored_apps = {
                            let guard = settings_ref.lock().unwrap();
                            guard.ignored_apps.clone()
                        };

                        if ignored_apps.iter().any(|a| a.eq_ignore_ascii_case(&app_info.app_class)) {
                            // skip saving this clip
                            return;
                        }

                        // Create clip without cloning content unnecessarily
                        let clip = Clip::new(
                            content.to_string(),
                            app_info.app_class.clone(),
                            app_info.window_title.clone(),
                            auto_tags,
                            vec![],
                            false,
                        );

                        match clip_store.save_clip(&clip) {
                            Ok(saved_clip) => {
                                // emit to frontend - only emit minimal data
                                if let Err(e) = app_handle.emit("clip-added", &saved_clip) {
                                    error!("Failed to emit clip-added: {}", e);
                                } else {
                                    info!("Captured new clip ({} bytes)", saved_clip.content.len());
                                }
                            }
                            Err(e) => error!("Failed to save clip: {}", e),
                        }
                    });

                    // store handle so it can be stopped from elsewhere if needed
                    *watcher_handle_ref.lock().unwrap() = Some(handle);

                    info!("Clipboard watcher started (dedicated thread).");
                });
            }

            // Auto cleanup thread (infrequent)
            {
                let clip_store = clip_store.clone();
                let settings_ref = settings_ref.clone();
                thread::spawn(move || {
                    loop {
                        // Sleep 6 hours between runs (same as before)
                        std::thread::sleep(Duration::from_secs(60 * 60 * 6));

                        let (days, max_size) = {
                            let s = settings_ref.lock().unwrap();
                            (s.auto_clean_days, s.max_history_size)
                        };

                        if days > 0 {
                            match clip_store.perform_cleanup(days as i64, max_size as i64) {
                                Ok(_) => info!("Auto cleanup completed."),
                                Err(e) => error!("Auto cleanup failed: {}", e),
                            }
                        }
                    }
                });
            }

            // Global shortcut: toggle persistent Quick Picker
            #[cfg(desktop)]
            {
                let app_handle_clone = app_handle.clone();
                let quick_picker_shortcut =
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

                // Register handler that toggles the quick-picker visibility (fast)
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            if shortcut == &quick_picker_shortcut
                                && matches!(event.state(), ShortcutState::Pressed)
                            {
                                let ah = app_handle_clone.clone();
                                // spawn small task so the handler is non-blocking
                                spawn(async move {
                                    if let Some(qw) = ah.get_webview_window("quick-picker") {
                                        match qw.is_visible() {
                                            Ok(true) => {
                                                let _ = qw.hide();
                                            }
                                            Ok(false) => {
                                                let _ = qw.show();
                                                let _ = qw.set_focus();
                                            }
                                            Err(e) => {
                                                error!("Quick-picker visibility check failed: {}", e);
                                            }
                                        }
                                    } else {
                                        // unlikely, but recreate fallback (should not normally happen)
                                        match tauri::WebviewWindowBuilder::new(
                                            &ah,
                                            "quick-picker",
                                            WebviewUrl::App("/quick-picker".into()),
                                        )
                                        .title("Quick Picker")
                                        .inner_size(420.0, 500.0)
                                        .resizable(false)
                                        .decorations(false)
                                        .visible(true)
                                        .always_on_top(true)
                                        .build()
                                        {
                                            Ok(new_w) => {
                                                info!("Quick Picker recreated as fallback.");
                                                let _ = new_w.set_focus();
                                            }
                                            Err(e) => error!("Failed to recreate quick-picker: {}", e),
                                        }
                                    }
                                });
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(quick_picker_shortcut)?;
                info!("Registered global shortcut: Ctrl+Shift+V for Quick Picker");
            }

            // System tray (open -> lazy-create main window)
            let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = MenuBuilder::new(app).items(&[&open_item, &quit_item]).build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("ClipContex")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        let a = app.clone();
                        // spawn small thread to avoid blocking tray callback
                        std::thread::spawn(move || {
                            // If main exists, show & focus; otherwise create it lazily
                            if let Some(mw) = a.get_webview_window("main") {
                                let _ = mw.show();
                                let _ = mw.set_focus();
                            } else {
                                match tauri::WebviewWindowBuilder::new(
                                    &a,
                                    "main",
                                    WebviewUrl::App("/".into()),
                                )
                                .title("ClipContex")
                                .inner_size(800.0, 600.0)
                                .resizable(true)
                                .decorations(true)
                                .visible(true)
                                .build()
                                {
                                    Ok(new_main) => {
                                        info!("Main window lazily created.");
                                        // When user closes the main window we'll let it fully close/destroy
                                        // (don't intercept with `prevent_close`), so WebKit process will exit.
                                        let _ = new_main.set_focus();
                                    }
                                    Err(e) => error!("Failed to create main window: {}", e),
                                }
                            }
                        });
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        // invoke handlers (same commands you used previously)
        .invoke_handler(tauri::generate_handler![
            commands::get_recent_clips,
            commands::clear_history,
            commands::delete_clip,
            commands::pin_clip,
            commands::ignore_next_clipboard_update,
            commands::load_config,
            commands::save_config,
            commands::is_kdotool_installed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}