// src-tauri/src/core/system_tray.rs
//! System tray icon and menu integration.
//!
//! Sets up a single system tray icon with "Open" and "Quit" menu items.
//! The tray is created only once, even if this function is called multiple times
//! (e.g., during Tauri hot reload in development mode).
//!
//! Requires a default window icon to be defined in `tauri.conf.json`.

use std::sync::atomic::{AtomicBool, Ordering};

use tauri::tray::TrayIconBuilder;
use tracing::error;

use crate::{core::window_creation, service::settings};

static TRAY_CREATED: AtomicBool = AtomicBool::new(false);

/// Configures the system tray icon and menu.
///
/// This function is idempotent: subsequent calls after the first are silently ignored.
/// This prevents duplicate tray icons during development (e.g., with Tauri hot reload).
///
/// # Panics
///
/// Panics if no default window icon is configured in `tauri.conf.json`.
pub fn setup_system_tray(app: &tauri::App) -> tauri::Result<()> {
    // Prevent duplicate tray creation (common in dev mode with hot reload)
    if TRAY_CREATED.swap(true, Ordering::Relaxed) {
        return Ok(());
    }

    let open_item = tauri::menu::MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let quit_item = tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let tray_menu = tauri::menu::MenuBuilder::new(app)
        .items(&[&open_item, &quit_item])
        .build()?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone()) // Assumes icon is set in tauri.conf.json
        .tooltip("Clipcontex")
        .menu(&tray_menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => match settings::load_settings() {
                Ok(settings) => {
                    if !settings.is_new_user {
                        window_creation::create_or_show_main_window(app);
                    } else {
                        window_creation::create_onboarding_window(app);
                    }
                }
                Err(e) => error!("Failed to load settings: {}", e),
            },
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;

    Ok(())
}