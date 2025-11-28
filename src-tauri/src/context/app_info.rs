//src-tauri/src/context/app_info.rs
/// Represent the active application context
#[derive(Debug, Clone, PartialEq)]
pub struct AppInfo {
    /// The window title (e.g., "my-app — VS Code")
    pub window_title: String,
    /// The application class (e.g., "code", "firefox", "konsole")
    pub app_class: String,
}

impl AppInfo {
    pub fn unknown() -> Self {
        Self {
            window_title: "Unknown".to_string(),
            app_class: "Unknown".to_string(),
        }
    }
}
