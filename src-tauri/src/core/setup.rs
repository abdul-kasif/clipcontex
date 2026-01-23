// src-tauri/src/core/setup.rs
//! Application setup and initialization logic.
//!
//! This module configures the app on first launch, starts background services,
//! and registers platform integrations (shortcuts, tray, autostart).

use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;
use tracing::{error, info};

#[cfg(desktop)]
use crate::core::global_shortcut;
use crate::{
    clipboard::watcher::{ClipboardWatcher, ClipboardWatcherHandle},
    context::{app_info::get_active_app_info, auto_tags::generate_auto_tags},
    core::{cleanup, system_tray, window_creation},
    service,
    state::AppState,
    storage::Clip,
};

/// Performs one-time application setup during Tauri's `setup` hook.
///
/// Initializes:
/// - Application state (`AppState`)
/// - Clipboard watcher
/// - Auto-cleanup task
/// - Global shortcut (desktop only)
/// - System tray
/// - First-run onboarding
pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::new();
    let app_handle = app.handle().clone();

    let watcher_handle = app_state.watcher_handle.clone();
    let settings = app_state.settings.clone();
    let clip_store = app_state.clip_store.clone();
    let quick_picker_shortcut_arc = app_state.quick_picker_shortcut.clone();

    app.manage(app_state);

    if let Err(e) = handle_first_run(&app_handle) {
        error!("Error occurred while handling first run: {}", e);
    }

    start_clipboard_watcher(app_handle.clone(), watcher_handle.clone());

    cleanup::spawn_auto_cleanup_task(settings.clone(), clip_store.clone());

    #[cfg(desktop)]
    global_shortcut::handle_quick_picker_shortcut(&app_handle, quick_picker_shortcut_arc.clone())?;

    #[cfg(desktop)]
    global_shortcut::register_quick_picker_shortcut(
        &app_handle,
        quick_picker_shortcut_arc.clone(),
    )?;

    system_tray::setup_system_tray(app)?;

    Ok(())
}

// ===== Helper Functions =====

fn handle_first_run(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let settings = service::settings::load_settings()?;

    if settings.is_new_user {
        info!("First launch -> show onboarding window");

        if let Err(e) = app_handle.autolaunch().enable() {
            error!("Failed to enable autolaunch: {}", e);
        } else {
            info!("Autostart enabled successfully.");
        }
        window_creation::create_onboarding_window(app_handle);
    } else {
        info!("Returning user -> skipping onboarding window");
    }

    Ok(())
}

fn start_clipboard_watcher(
    app_handle: AppHandle,
    watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
) {
    let watcher = ClipboardWatcher::new();

    let handle = watcher.start(app_handle.clone(), move |event| {
        let content = event.content.trim();
        if content.is_empty() || content.len() < 2 {
            return;
        }

        let app_info = get_active_app_info();
        let should_ignore = match service::settings::load_settings() {
            Ok(settings) => settings
                .ignored_apps
                .iter()
                .any(|a| a.eq_ignore_ascii_case(&app_info.app_class)),
            Err(e) => {
                error!("{}", e);
                false
            }
        };

        if should_ignore {
            return;
        }

        let auto_tags = generate_auto_tags(content, Some(&app_info.app_class));
        let clip = Clip::new(
            content.to_string(),
            app_info.app_class.clone(),
            app_info.window_title.clone(),
            auto_tags,
            false,
        );

        let app_state = app_handle.state::<AppState>();
        match service::clip::save_clip(app_state.inner(), clip) {
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

    // Use poison recovery to avoid panic
    *watcher_handle.lock().unwrap_or_else(|e| e.into_inner()) = Some(handle);
    info!("Clipboard watcher started successfully");
}
