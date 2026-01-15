// src-tauri/src/lib.rs
//! ClipContex — A smart clipboard manager for developers and power users.
//!
//! This application captures clipboard history with contextual metadata (app name,
//! window title, auto-tags), provides quick search/pin functionality, and respects
//! user privacy through local-first storage.
//!
//! The core architecture follows a layered design:
//! - **Clipboard**: Clipboard monitoring and processing subsystem (`clipboard` module).
//! - **Storage**: SQLite-backed persistence (`storage` module).
//! - **Config**: User settings management (`config` module).
//! - **Core**: Application lifecycle and window logic (`core` module).
//! - **Context**: Cross-platform contextual metadata extraction for clipboard entries (`context` module).
//! - **State**: Application-wide shared state management (`state` module).
//! - **Commands**: Tauri IPC handlers (`command` module).
//! - **Services**: Background logic (e.g., clipboard monitoring in `service`).
//! - **Error**: Application-specific error types (`error` module).

// ===== Global Allocator =====

// Use mimalloc for improved memory allocation performance,
// especially beneficial in long-running desktop applications.
use mimalloc::MiMalloc;
use tracing::warn;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// ===== Modules =====

pub mod clipboard;
pub mod command;
pub mod config;
pub mod context;
pub mod core;
pub mod error;
pub mod service;
pub mod state;
pub mod storage;

// ===== Imports =====

use crate::core::setup::setup;
use crate::core::window_creation;
use tauri_plugin_autostart::MacosLauncher;

// ===== Public API =====

/// Launches the ClipContex Tauri application.
///
/// This function:
/// - Initializes logging via `tracing`.
/// - Applies platform-specific workarounds (e.g., for Linux WebKit).
/// - Registers Tauri plugins (autostart, clipboard, single-instance, etc.).
/// - Sets up the application state and IPC command handlers.
/// - Starts the Tauri runtime.
///
/// On macOS and Windows, the app supports system autostart.
/// On Linux, certain WebKit flags are disabled to improve stability.
///
/// # Panics
///
/// Panics if the Tauri runtime fails to start (e.g., due to missing assets or context error).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ===== Platform-Specific Environment Setup =====

    // On Linux, disable problematic WebKit features to avoid rendering issues
    // in older or headless environments (e.g., Wayland compositors, minimal X11).
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        std::env::set_var("WEBKIT_DISABLE_WEBGL", "1");
        std::env::set_var("WEBKIT_DISABLE_MEDIA_SOURCE", "1");
        std::env::set_var("WEBKIT_DISABLE_CACHE", "1");
        std::env::set_var("WEBKIT_DISABLE_WEB_PROCESS_CACHE", "1");
    }

    // ===== Logging Initialization =====

    // Initialize a compact, human-readable tracing subscriber.
    // Timestamps and targets are omitted to reduce noise in logs.
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .compact()
        .init();

    // ===== Tauri Application Builder =====

    tauri::Builder::default()
        // Ensure only one instance of the app runs at a time.
        .plugin(tauri_plugin_single_instance::init(|app, _arg, _cmd| {
            window_creation::create_or_show_main_window(app);
        }))
        // Enable file opening via OS (e.g., double-click on `.clipcontex` files).
        .plugin(tauri_plugin_opener::init())
        // Persistent key-value store (used for non-clipboard state if needed).
        .plugin(tauri_plugin_store::Builder::default().build())
        // Access to system clipboard (read/write).
        .plugin(tauri_plugin_clipboard_manager::init())
        // OS information (e.g., platform detection).
        .plugin(tauri_plugin_os::init())
        // Autostart on login:
        // - macOS: LaunchAgent
        // - Windows: Registry
        // - Linux: .desktop file in ~/.config/autostart (handled by the plugin if permissions allow)
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]), // No additional arguments on launch
        ))
        // Custom setup logic (e.g., database init, tray menu, clipboard monitor).
        .setup(|app| setup(app))
        // Register all IPC commands exposed to the frontend.
        .invoke_handler(tauri::generate_handler![
            command::list_recent_clips,
            command::toggle_pin_status,
            command::remove_clip,
            command::clear_clip_history,
            command::ignore_next_clip,
            command::load_settings,
            command::save_settings,
            command::mark_onboarding_complete,
            command::check_kdotool_installed,
            command::set_dragging,
        ])
        // Build and run the application.
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            warn!("Failed to start clipcontex: {}", e);
            std::process::exit(1)
        })
}
