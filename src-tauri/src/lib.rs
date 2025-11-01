use std::time::Duration;
use tauri::async_runtime::spawn;
use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};
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
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_state = AppState::new();
            let clip_store = app_state.clip_store.clone();
            let watcher_handle_ref = app_state.watcher_handle.clone();
            let settings_ref = app_state.settings.clone();

            let app_handle = app.handle().clone();
            app.manage(app_state);

            // Async clipboard watcher thread
            tauri::async_runtime::spawn({
                let app_handle = app_handle.clone();
                let clip_store = clip_store.clone();
                let settings_ref = settings_ref.clone();
                let watcher_handle_ref = watcher_handle_ref.clone();

                async move {
                    let mut watcher = ClipboardWatcher::new();
                    let handle = watcher.start(app_handle.clone(), move |event| {
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

                        // Ignore clips from excluded apps
                        let ignored = {
                            let settings_guard = settings_ref.lock().unwrap();
                            settings_guard.ignored_apps.clone()
                        };

                        if ignored
                            .iter()
                            .any(|a| a.eq_ignore_ascii_case(&app_info.app_class))
                        {
                            return;
                        }

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
                    info!("Clipboard watcher started (async).");
                }
            });

            // Auto Cleanup (every 6 hours)
            tauri::async_runtime::spawn({
                let clip_store = clip_store.clone();
                let settings_ref = settings_ref.clone();
                async move {
                    loop {
                        tokio::time::sleep(Duration::from_secs(60 * 60 * 6)).await;
                        let (days, max_size) = {
                            let s = settings_ref.lock().unwrap();
                            (s.auto_clean_days, s.max_history_size)
                        };
                        if days > 0 {
                            if let Err(e) = clip_store.perform_cleanup(days as i64, max_size as i64)
                            {
                                error!("Auto-clean failed: {}", e);
                            }
                        }
                    }
                }
            });

            // Quick Picker Shortcut (Lazy Window)
            #[cfg(desktop)]
            {
                let app_handle_clone = app_handle.clone();
                let quick_picker_shortcut =
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            if shortcut == &quick_picker_shortcut
                                && matches!(event.state(), ShortcutState::Pressed)
                            {
                                let app_handle = app_handle_clone.clone();
                                spawn(async move {
                                    if let Some(_) = app_handle.get_webview_window("quick-picker") {
                                        match tauri::WebviewWindowBuilder::new(
                                            &app_handle,
                                            "quick-picker",
                                            tauri::WebviewUrl::App("quick-picker".into()),
                                        )
                                        .title("Quick Picker")
                                        .inner_size(420.0, 450.0)
                                        .resizable(false)
                                        .decorations(false)
                                        .visible(true)
                                        .always_on_top(true)
                                        .build()
                                        {
                                            Ok(new_window) => {
                                                info!("Quick Picker created.");
                                                let new_window_ = new_window.clone();
                                                new_window.on_window_event(move |event| {
                                                    if let tauri::WindowEvent::Focused(false) =
                                                        event
                                                    {
                                                        let _ = new_window_.close();
                                                        info!("Quick Picker auto-closed on blur.");
                                                    }
                                                });
                                            }
                                            Err(e) => {
                                                error!("Failed to create Quick Picker: {}", e)
                                            }
                                        }
                                    } else {
                                        info!("Creating Quick Picker window...");
                                        match tauri::WebviewWindowBuilder::new(
                                            &app_handle,
                                            "quick-picker",
                                            tauri::WebviewUrl::App("quick-picker".into()),
                                        )
                                        .title("Quick Picker")
                                        .inner_size(500.0, 600.0)
                                        .resizable(false)
                                        .decorations(false)
                                        .visible(true)
                                        .always_on_top(true)
                                        .build()
                                        {
                                            Ok(new_window) => {
                                                info!("Quick Picker created.");
                                                let new_window_ = new_window.clone();
                                                new_window.on_window_event(move |event| {
                                                    if let tauri::WindowEvent::Focused(false) =
                                                        event
                                                    {
                                                        let _ = new_window_.close();
                                                        info!("Quick Picker auto-closed on blur.");
                                                    }
                                                });
                                            }
                                            Err(e) => {
                                                error!("Failed to create Quick Picker: {}", e)
                                            }
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

            //  System Tray-
            let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = MenuBuilder::new(app)
                .items(&[&open_item, &quit_item])
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("ClipContex")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            // Main Window: Minimize on Close
            if let Some(main_window) = app.get_webview_window("main") {
                let main_window_ = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = main_window_.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_recent_clips,
            commands::clear_history,
            commands::delete_clip,
            commands::pin_clip,
            commands::capture_current_clip,
            commands::ignore_next_clipboard_update,
            commands::load_config,
            commands::save_config,
            commands::is_kdotool_installed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
