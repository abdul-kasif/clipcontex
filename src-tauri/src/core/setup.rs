// // ===== Imports =====
use std::thread;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;
use tracing::{error, info};

// // ===== Crates =====
#[cfg(desktop)]
use crate::core::global_shortcut;
use crate::{
    clipboard::watcher::ClipboardWatcher,
    command::AppState,
    context::{generate_auto_tags, get_active_app_info},
    core::{cleanup, system_tray, window_creation},
    service::settings,
    storage::Clip,
};

// ===== Public APi =====
pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::new();
    let app_handle = app.handle().clone();
    let app_state_clone = app_state.clone();

    app.manage(app_state);

    if let Err(e) = handle_first_run(&app_handle) {
        error!("Error occurred while handling first run: {}", e);
    }

    start_clipboard_watcher(&app_handle, &app_state_clone);

    cleanup::spawn_auto_cleanup_task(&app_state_clone);

    #[cfg(desktop)]
    global_shortcut::handle_quick_picker_shortcut(app)?;

    #[cfg(desktop)]
    global_shortcut::register_quick_picker_shortcut(app)?;

    system_tray::setup_system_tray(app)?;

    Ok(())
}

// ===== Helper Functions =====
fn handle_first_run(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let settings = settings::load_settings()?;

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

fn start_clipboard_watcher(app_handle: &AppHandle, app_state: &AppState) {
    let app_handle = app_handle.clone();
    let clip_store = app_state.clip_store.clone();
    let settings_arc = app_state.settings.clone();
    let watcher_handle = app_state.watcher_handle.clone();

    thread::spawn(move || {
        let mut watcher = ClipboardWatcher::new();

        let handle = watcher.start(app_handle.clone(), move |event| {
            let content = event.content.trim();
            if content.is_empty() || content.len() < 2 {
                return;
            }

            let app_info = get_active_app_info();

            let should_ignore = {
                let guard = settings_arc.read().unwrap();
                guard
                    .ignored_apps
                    .iter()
                    .any(|a| a.eq_ignore_ascii_case(&app_info.app_class))
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
