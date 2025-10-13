#[allow(dead_code)]
pub mod clipboard;
pub mod context;
pub mod storage;
pub mod commands;

use commands::{capture_current_clip, clear_history, delete_clip, get_recent_clips, pin_clip, search_clips, AppState};
use tauri::{Manager, command};

#[command]
fn greet() -> String {
    "Hello from Rust!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize global state
            app.manage(AppState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_recent_clips,
            search_clips,
            clear_history,
            delete_clip,
            pin_clip,
            capture_current_clip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}