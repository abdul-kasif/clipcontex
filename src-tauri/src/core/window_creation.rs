// ===== Imports =====
use std::time::Duration;
use tauri::{Manager, WebviewUrl};
use tracing::{error, info};

// ===== Crate =====
use crate::core::platform;

// ===== Public API =====
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
        Ok(window) => {
            window.on_window_event(|event| {
                if let tauri::WindowEvent::Destroyed = event {
                    platform::malloc_trim_now();
                    info!("onboarding memory released");
                }
            });
        }

        Err(e) => error!("Failed to create onboarding window: {}", e),
    }
}

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
                info!("main window is created");
                let _ = window.set_focus();
                window.on_window_event(|event| {
                    if let tauri::WindowEvent::Destroyed = event {
                        platform::malloc_trim_now();
                        info!("main window memory released");
                    }
                });
            }
            Err(e) => error!("Failed to create main window: {}", e),
        }
    }
}

pub fn hide_and_show_quick_picker_window(app_handle: &tauri::AppHandle) {
    let app_handle_clone = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Some(window) = app_handle_clone.get_webview_window("quick-picker") {
            // Hide → trim → show
            if let Err(e) = window.hide() {
                error!("Failed to hide Quick Picker: {}", e);
            } else {
                platform::malloc_trim_now();
            }

            tokio::time::sleep(Duration::from_millis(80)).await;

            if let Err(e) = window.show() {
                error!("Failed to re-show Quick Picker: {}", e);
            } else {
                let _ = window.set_focus();
            }

            // Attach focus-loss handler ONCE
            static HANDLER_ATTACHED: std::sync::Once = std::sync::Once::new();
            HANDLER_ATTACHED.call_once(|| {
                let win_ref = window.clone();
                window.on_window_event(move |ev| {
                    if let tauri::WindowEvent::Focused(false) = ev {
                        let _ = win_ref.hide();
                        platform::malloc_trim_now();
                    }
                });
            });
        } else {
            error!("Quick Picker window not found!");
        }
    });
}
