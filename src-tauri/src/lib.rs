// ===== Imports =====
use crate::core::window_creation;
use tauri_plugin_autostart::MacosLauncher;

// ===== Modules =====
pub mod clipboard;
pub mod command;
pub mod config;
pub mod context;
pub mod core;
pub mod error;
pub mod service;
pub mod storage;

// ===== Crates =====
use crate::core::setup::setup;

// ===== Allocators =====
#[cfg(target_os = "linux")]
use tikv_jemallocator::Jemalloc;

#[cfg(target_os = "linux")]
#[global_allocator]
static GLOBAL_ALLOCATOR: Jemalloc = Jemalloc;

// ===== Public API =====
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    //     ===== Environment Variables For Linux =====
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

    //     ===== Logging Subscriber =====
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _arg, _cmd| {
            window_creation::create_or_show_main_window(app);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .setup(|app| setup(app))
        .invoke_handler(tauri::generate_handler![
            command::list_recent_clips,
            command::set_clip_pinned,
            command::remove_clip,
            command::clear_clip_history,
            command::ignore_next_clip,
            command::load_settings,
            command::save_settings,
            command::mark_onboarding_complete,
            command::check_kdotool_installed,
        ])
        .run(tauri::generate_context!())
        .expect("Error running Tauri application");
}
