use crate::context::AppInfo;

mod win32;

pub fn get_active_app_info_windows() -> AppInfo {
    win32::get_active_app_info_windows_win32()
}

