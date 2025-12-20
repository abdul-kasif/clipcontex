mod app_info;
mod auto_tag;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(test)]
mod tests;

pub use app_info::{get_active_app_info, AppInfo};
pub use auto_tag::generate_auto_tags;
