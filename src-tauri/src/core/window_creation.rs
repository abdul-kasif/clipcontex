// src-tauri/src/core/window_creation.rs
//! Window creation and management utilities.
//!
//! Provides helpers to create and show:
//! - Main application window
//! - Onboarding window
//! - Quick picker window (with focus and drag-aware behavior)
//!
//! The quick picker window includes special logic to:
//! - Ensure it gains focus reliably via a hide/show cycle.
//! - Auto-hide when losing focus — **unless** the user is dragging the window.
//!   Drag state is tracked via [`AppState::is_quick_picker_dragging`].

use std::{sync::atomic::Ordering, time::Duration};
use tauri::{Manager, WebviewUrl};
use tracing::{error, info};

use crate::state;

/// Creates and shows the onboarding window.
///
/// This window is displayed only for new users on first launch.
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
///
/// If the window already exists, brings it to front and focuses it.
/// Otherwise, creates a new instance.
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
/// Also attaches a focus-loss handler that auto-hides the window **only when not dragging**.
/// Drag state is read from the shared [`AppState::is_quick_picker_dragging`] flag.
///
/// This prevents the window from disappearing during user-initiated movement.
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

            let is_dragging = app_handle_clone
                .state::<state::AppState>()
                .is_quick_picker_dragging
                .clone();

            // Attach focus-loss handler every time (safe to call multiple times)
            let win_ref = window.clone();
            window.on_window_event(move |ev| {
                if let tauri::WindowEvent::Focused(false) = ev {
                    // Only hide if the user isn't dragging the window
                    if !is_dragging.load(Ordering::Relaxed) {
                        let _ = win_ref.hide();
                    }
                }
            });
        } else {
            error!("Quick Picker window not found!");
        }
    });
}