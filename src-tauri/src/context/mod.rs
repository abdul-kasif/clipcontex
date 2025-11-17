mod app_info;
mod auto_tag;
#[cfg(target_os = "linux")]
mod linux;
mod project;
#[cfg(test)]
mod tests;

pub use app_info::AppInfo;
pub use auto_tag::generate_auto_tags;
pub use project::extract_project_from_title;

/// Returns the active application info.
/// On unsupported platforms, returns "unknown".
pub fn get_active_app_info() -> AppInfo {
    #[cfg(target_os = "linux")]
    {
        linux::get_active_app_info()
    }
    #[cfg(not(target_os = "linux"))]
    {
        AppInfo::unknown();
    }
}
