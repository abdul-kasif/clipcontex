// ===== Imports =====
use crate::context::AppInfo;

// ===== Modules =====
mod wayland;

// ===== Public API =====
pub fn get_active_app_info_linux() -> AppInfo {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();

    if session_type.eq_ignore_ascii_case("wayland") {
        return wayland::get_active_app_info_linux_wayland();
    }

    AppInfo::unknown()
}
