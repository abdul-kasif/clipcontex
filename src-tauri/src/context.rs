mod app_info;
mod auto_tags;
mod normalize_app_name;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(test)]
mod tests;

pub use app_info::{get_active_app_info, AppInfo};
pub use auto_tags::generate_auto_tags;
pub use normalize_app_name::normalize_app_class;
