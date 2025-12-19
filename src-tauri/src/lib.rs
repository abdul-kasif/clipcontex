// src-tauri/src/lib.rs
#![cfg_attr(not(debug_assertions), deny(warnings))]
use std::thread;
use tauri::{
    async_runtime::spawn,
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager, WebviewUrl,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tokio::time::Duration;
use tracing::{error, info};

// =======================
// Internal Modules
// =======================
pub mod clipboard;
pub mod commands;
pub mod config;
pub mod context;
pub mod storage;

use crate::{
    clipboard::watcher::ClipboardWatcher,
    commands::AppState,
    config::load_settings,
    context::{app_info::get_active_app_info, generate_auto_tags},
    storage::Clip,
};

// ================================
// Memory Allocator (Jemalloc)
// ================================
#[cfg(target_os = "linux")]
use tikv_jemallocator::Jemalloc;

#[cfg(target_os = "linux")]
#[global_allocator]
static GLOBAL_ALLOCATOR: Jemalloc = Jemalloc;

#[cfg(target_os = "linux")]
mod malloc_trim_support {
    use std::ffi::c_int;
    extern "C" {
        pub fn malloc_trim(__pad: c_int) -> c_int;
    }
    #[inline]
    pub fn trim() {
        unsafe {
            malloc_trim(0);
        }
    }
}

#[cfg(target_os = "linux")]
use malloc_trim_support::trim as malloc_trim_now;

// #[cfg(not(target_os = "linux"))]
// mod malloc_trim_support {
//     #[inline]
//     pub fn trim() {}
// }

// ================================
// Application Entrypoint
// ================================
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // -----------------------
    // Environment & WebKit Setup
    // -----------------------

    #[cfg(target_os = "linux")]
    {
        std::env::set_var(
            "MALLOC_CONF",
            "dirty_decay_ms:1000,muzzy_decay_ms:1000,background_thread:true",
        );

        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        std::env::set_var("WEBKIT_DISABLE_WEBGL", "1");
        std::env::set_var("WEBKIT_DISABLE_MEDIA_SOURCE", "1");
        std::env::set_var("WEBKIT_DISABLE_CACHE", "1");
        std::env::set_var("WEBKIT_DISABLE_WEB_PROCESS_CACHE", "1");
    }

    #[cfg(all(target_os = "linux", debug_assertions))]
    {
        std::env::set_var("G_DEBUG", "gc-friendly");
        std::env::set_var("G_SLICE", "always-malloc");
    }

    // -----------------------
    // Logging Setup
    // -----------------------
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .init();

    // -----------------------
    // Tauri App Builder
    // -----------------------
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .setup(|app| {
            // === Shared global state ===
            let app_state = AppState::new();
            let clip_store = app_state.clip_store.clone();
            let watcher_handle = app_state.watcher_handle.clone();
            let settings_arc = app_state.settings.clone();
            let app_handle = app.handle().clone();

            app.manage(app_state);

            // === Ensure config.json exists (first run detection) ===
            {
                match load_settings() {
                    Ok(settings) => {
                        if settings.is_new_user {
                            info!("First launch → showing onboarding window.");
                            if let Err(e) = app_handle.autolaunch().enable() {
                                error!("Failed to enable autostart {}", e);
                            } else {
                                info!("Autostart enabled successfully");
                            }
                            let app_handle_onboarding = app_handle.clone();

                            thread::spawn(move || {
                                match tauri::WebviewWindowBuilder::new(
                                    &app_handle_onboarding,
                                    "onboarding",
                                    WebviewUrl::App("/onboarding".into()),
                                )
                                .title("Welcome to ClipContex")
                                .inner_size(800.0, 600.0)
                                .resizable(true)
                                .decorations(true)
                                .center()
                                .visible(true)
                                .build()
                                {
                                    Ok(window) => {
                                        info!("Onboarding window created.");

                                        // When destroyed, trim memory
                                        window.on_window_event(|event| {
                                            if let tauri::WindowEvent::Destroyed = event {
                                                #[cfg(target_os = "linux")]
                                                malloc_trim_now();
                                                info!("Onboarding memory released.");
                                            }
                                        });
                                    }
                                    Err(e) => error!("Failed to create onboarding window: {}", e),
                                }
                            });
                        } else {
                            info!("Returning user detected → skipping onboarding.");
                            if let Ok(is_enabled) = app_handle.autolaunch().is_enabled() {
                                if !is_enabled {
                                    error!("Autostart is disabled by user");
                                } else {
                                    info!("Autostart is enabled");
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to load config: {}", e),
                }
            }
            // === Clipboard watcher ===
            {
                let app_handle = app_handle.clone();
                let clip_store = clip_store.clone();
                let settings_arc = settings_arc.clone();
                let watcher_handle = watcher_handle.clone();

                thread::spawn(move || {
                    let mut watcher = ClipboardWatcher::new();

                    let handle = watcher.start(app_handle.clone(), move |event| {
                        let content = event.content.trim();
                        if content.is_empty() || content.len() < 2 {
                            return;
                        }

                        let app_info = get_active_app_info();

                        let auto_tags = generate_auto_tags(
                            content,
                            Some(&app_info.window_title),
                            Some(&app_info.app_class),
                        );

                        let ignored_apps = {
                            let guard = settings_arc.read().unwrap();
                            guard.ignored_apps.clone()
                        };

                        if ignored_apps
                            .iter()
                            .any(|a| a.eq_ignore_ascii_case(&app_info.app_class))
                        {
                            return;
                        }

                        let clip = Clip::new(
                            content.to_string(),
                            app_info.app_class.clone(),
                            app_info.window_title.clone(),
                            auto_tags,
                            false,
                        );

                        match clip_store.save_clip(&clip) {
                            Ok(saved) => {
                                if let Err(e) = app_handle.emit("clip-added", &saved) {
                                    error!("Failed to emit 'clip-added': {}", e);
                                } else {
                                    info!("New clip captured ({} bytes)", saved.content.len());
                                }
                            }
                            Err(e) => error!("Failed to save clip: {}", e),
                        }
                    });

                    *watcher_handle.lock().unwrap() = Some(handle);
                    info!("Clipboard watcher started successfully.");
                });
            }

            // === Auto cleanup thread ===
            {
                let clip_store = clip_store.clone();
                let settings_arc = settings_arc.clone();

                thread::spawn(move || loop {
                    thread::sleep(Duration::from_secs(6 * 60 * 60));

                    let (days, max) = {
                        let s = settings_arc.read().unwrap();
                        (s.auto_clean_days, s.max_history_size)
                    };

                    if days > 0 {
                        match clip_store.perform_cleanup(days as i64, max as i64) {
                            Ok(_) => {
                                info!("Auto cleanup completed.");
                                #[cfg(target_os = "linux")]
                                malloc_trim_now();
                            }
                            Err(e) => error!("Auto cleanup failed: {}", e),
                        }
                    } else {
                        #[cfg(target_os = "linux")]
                        malloc_trim_now();
                    }
                });
            }

            // === Periodic heap trimming thread ===
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(30));
                #[cfg(target_os = "linux")]
                malloc_trim_now();
            });

            // === Quick Picker Global Shortcut (Ctrl+Shift+V) ===
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
                        if let Some(window) = app_handle.get_webview_window("quick-picker") {
                        // Step 1: Always hide and trim first to reset any UI state
                            if let Err(e) = window.hide() {
                                error!("Failed to hide Quick Picker: {}", e);
                            } else {
                                info!("Quick Picker hidden for refresh.");
                                #[cfg(target_os = "linux")]
                                malloc_trim_now();
                            }

                            // Step 2: Small delay — ensures WebKit processes sync on hide/show
                            tokio::time::sleep(Duration::from_millis(80)).await;

                            // Step 3: Show again and refocus
                            if let Err(e) = window.show() {
                                error!("Failed to re-show Quick Picker: {}", e);
                            } else {
                                let _ = window.set_focus();
                                info!("Quick Picker reopened & focused.");
                            }

                            //  Step 4: Ensure we only register focus-loss handler ONCE
                            static HANDLER_ATTACHED: std::sync::Once = std::sync::Once::new();
                            HANDLER_ATTACHED.call_once(|| {
                                let win_ref = window.clone();
                                window.on_window_event(move |ev| {
                                if let tauri::WindowEvent::Focused(false) = ev {
                                    let _ = win_ref.hide();
                                    #[cfg(target_os = "linux")]
                                    malloc_trim_now();
                                    info!("Quick Picker auto-hidden after losing focus.");
                                }
                            });
                        });
                    } else {
                        error!("Quick Picker window not found! (maybe closed accidentally)");
                    }
                });
            }
        })
        .build(),
    )?;

                app.global_shortcut().register(quick_picker_shortcut)?;
                info!("Registered Ctrl+Shift+V for Smart Quick Picker Refresh");
            }

            // === System tray setup ===
            let open_item = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = MenuBuilder::new(app)
                .items(&[&open_item, &quit_item])
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("ClipContex")
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        let app_clone = app.clone();
                        thread::spawn(move || {
                            match load_settings() {
                                Ok(settings) => {
                                    if settings.is_new_user {
                                        match tauri::WebviewWindowBuilder::new(
                                            &app_clone,
                                            "onboarding",
                                            WebviewUrl::App("/onboarding".into()),
                                        )
                                        .title("Welcome to ClipContex")
                                        .inner_size(800.0, 600.0)
                                        .resizable(true)
                                        .decorations(true)
                                        .center()
                                        .visible(true)
                                        .build()
                                        {
                                            Ok(window) => {
                                                info!("Onboarding window created.");

                                                // When destroyed, trim memory
                                                window.on_window_event(|event| {
                                                    if let tauri::WindowEvent::Destroyed = event {
                                                        #[cfg(target_os = "linux")]
                                                        malloc_trim_now();
                                                        info!("Onboarding memory released.");
                                                    }
                                                });
                                            }
                                            Err(e) => {
                                                error!("Failed to create onboarding window: {}", e)
                                            }
                                        }
                                    } else if let Some(main_window) =
                                        app_clone.get_webview_window("main")
                                    {
                                        let _ = main_window.show();
                                        let _ = main_window.set_focus();
                                    } else {
                                        match tauri::WebviewWindowBuilder::new(
                                            &app_clone,
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
                                            Ok(main) => {
                                                info!("Main window created.");
                                                let _ = main.set_focus();
                                            }
                                            Err(e) => error!("Failed to create main window: {}", e),
                                        }
                                    }
                                }
                                Err(e) => error!("Failed to load settings {}", e),
                            }
                        });
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        // === Backend commands ===
        .invoke_handler(tauri::generate_handler![
            commands::get_recent_clips,
            commands::pin_clip,
            commands::delete_clip,
            commands::clear_history,
            commands::load_config,
            commands::save_config,
            commands::complete_onboarding,
            commands::ignore_next_clipboard_update,
            commands::is_kdotool_installed,
        ])
        .run(tauri::generate_context!())
        .expect("Error running Tauri application");
}
