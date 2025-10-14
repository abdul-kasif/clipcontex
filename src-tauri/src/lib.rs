use tauri::Manager;
use tracing::{info, error};

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
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_state = AppState::new();
            let clip_store = app_state.clip_store.clone();
            let watcher_handle_ref = app_state.watcher_handle.clone();

            app.manage(app_state);

            // Start clipboard watcher in background
            std::thread::spawn(move || {
                let mut watcher = ClipboardWatcher::new();
                let handle = watcher.start(move |event| {
                    let content = event.content.trim().to_string();
                    if content.is_empty() {
                        return;
                    }

                    let app_info = get_active_app_info();
                    let project_name = extract_project_from_title(&app_info.window_title);
                    let auto_tags = generate_auto_tags(&content, project_name.as_deref());

                    let clip = Clip::new(
                        content.clone(),
                        app_info.app_class,
                        app_info.window_title,
                        auto_tags,
                        vec![],
                        false,
                    );

                    if let Err(e) = clip_store.save_clip(&clip) {
                        error!("Failed to save clip: {}", e);
                    } else {
                        info!("✅ Captured new clip automatically: {}", clip.content);
                    }
                });

                // Store handle so it doesn’t drop
                *watcher_handle_ref.lock().unwrap() = Some(handle);

                info!("📋 Clipboard watcher started successfully and running.");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_recent_clips,
            commands::search_clips,
            commands::clear_history,
            commands::delete_clip,
            commands::pin_clip,
            commands::capture_current_clip,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
