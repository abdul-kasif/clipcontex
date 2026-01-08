//src-tauri/src/context/app_info.rs
#[cfg(target_os = "linux")]
use crate::context::linux;

#[cfg(target_os = "windows")]
use crate::context::windows;

// ===== Domain Types =====
/// Represent the active application context
#[derive(Debug, Clone, PartialEq)]
pub struct AppInfo {
    pub window_title: String,
    pub app_class: String,
}

// ===== AppInfo Implementation =====
impl AppInfo {
    pub fn unknown() -> Self {
        Self {
            window_title: "Unknown".to_string(),
            app_class: "Unknown".to_string(),
        }
    }

    pub fn unknown_app_class() -> String {
        "Unknown".to_string()
    }

    pub fn unknown_window_title() -> String {
        "Unknown".to_string()
    }
}

// ===== Public API =====
pub fn get_active_app_info() -> AppInfo {
    #[cfg(target_os = "linux")]
    {
        linux::get_active_app_info_linux()
    }
    #[cfg(target_os = "windows")]
    {
        windows::get_active_app_info_windows()
    }
}
