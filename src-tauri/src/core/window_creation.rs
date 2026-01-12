// src-tauri/src/core/window_creation.rs
//! Window creation and management utilities.
//!
//! Provides helpers to create and show:
//! - Main application window
//! - Onboarding window
//! - Quick picker window (with focus management)

use std::time::Duration;
use tauri::{Manager, WebviewUrl};
use tracing::{error, info};

/// Creates and shows the onboarding window.
pub fn create_onboarding_window(app_handle: &tauri::AppHandle) {
    match tauri::WebviewWindowBuilder::new(
        app_handle,
        "onboarding",
        WebviewUrl::App("/onboarding".into()),
    )
    .title("Welcome to Clipcontex")
    .inner_size(800.0, 600.0)
    .resizable(true)
    .visible(true)
    .decorations(true)
    .center()
    .build()
    {
        Ok(_) => info!("Onboarding window created"),
        Err(e) => error!("Failed to create onboarding window: {}", e),
    }
}

/// Creates or shows the main application window.
pub fn create_or_show_main_window(app_handle: &tauri::AppHandle) {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        let _ = main_window.show();
        let _ = main_window.set_focus();
    } else {
        match tauri::WebviewWindowBuilder::new(app_handle, "main", WebviewUrl::App("/".into()))
            .title("Clipcontex")
            .inner_size(800.0, 600.0)
            .visible(true)
            .resizable(true)
            .decorations(true)
            .center()
            .build()
        {
            Ok(window) => {
                info!("Main window created");
                let _ = window.set_focus();
            }
            Err(e) => error!("Failed to create main window: {}", e),
        }
    }
}

/// Toggles the quick picker window with a brief hide/show cycle to ensure focus.
///
/// Also attaches a focus-loss handler to auto-hide the window when it loses focus.
pub fn hide_and_show_quick_picker_window(app_handle: &tauri::AppHandle) {
    let app_handle_clone = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Some(window) = app_handle_clone.get_webview_window("quick-picker") {
            if let Err(e) = window.hide() {
                error!("Failed to hide Quick Picker: {}", e);
            }
            tokio::time::sleep(Duration::from_millis(80)).await;

            if let Err(e) = window.show() {
                error!("Failed to re-show Quick Picker: {}", e);
            } else {
                let _ = window.set_focus();
            }

            // Attach focus-loss handler every time (safe to call multiple times)
            let win_ref = window.clone();
            window.on_window_event(move |ev| {
                if let tauri::WindowEvent::Focused(false) = ev {
                    let _ = win_ref.hide();
                }
            });
        } else {
            error!("Quick Picker window not found!");
        }
    });
}
