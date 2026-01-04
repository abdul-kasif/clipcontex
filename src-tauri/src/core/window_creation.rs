use crate::core::platform;
use tauri::{Manager, WebviewUrl};
use tracing::{error, info};

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
