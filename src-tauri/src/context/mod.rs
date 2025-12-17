mod app_info;
mod auto_tag;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(test)]
mod tests;

pub use app_info::AppInfo;
pub use auto_tag::generate_auto_tags;

/// Returns the active application info.
/// On unsupported platforms, returns "unknown".
pub fn get_active_app_info() -> AppInfo {
    #[cfg(target_os = "linux")]
    {
        linux::get_active_app_info_linux()
    }
    #[cfg(not(target_os = "linux"))]
    {
        AppInfo::unknown();
    }
}
