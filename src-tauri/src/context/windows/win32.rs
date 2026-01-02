// ===== Imports =====
use crate::context::AppInfo;
use std::path::Path;
use windows::{
    core::*,
    Win32::{Foundation::*, System::Threading::*, UI::WindowsAndMessaging::*},
};

// ===== Public API =====
pub fn get_active_app_info_windows_win32() -> AppInfo {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return AppInfo::unknown();
        }

        let window_title = get_windows_window_title(hwnd);
        let app_class = get_windows_application_name(hwnd);

        AppInfo {
            window_title,
            app_class,
        }
    }
}

// ===== Helper Functions =====
fn get_windows_window_title(hwnd: HWND) -> String {
    unsafe {
        let mut title_buffer = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buffer);
        if title_len == 0 {
            return AppInfo::unknown_window_title();
        }

        String::from_utf16_lossy(&title_buffer[..title_len as usize])
    }
}

fn get_windows_application_name(hwnd: HWND) -> String {
    unsafe {
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        let process_handle: HANDLE =
            match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                Ok(handle) => handle,
                Err(_) => return AppInfo::unknown_app_class(),
            };

        let mut class_buffer = [0u16; 1024];
        let mut class_len = class_buffer.len() as u32;
        let result = QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR(class_buffer.as_mut_ptr()),
            &mut class_len,
        );

        let _ = CloseHandle(process_handle);

        if result.is_ok() {
            let full_class_path = String::from_utf16_lossy(&class_buffer[..class_len as usize]);

            let raw_class_name = Path::new(&full_class_path)
                .file_name()
                .and_then(|r| r.to_str())
                .unwrap_or("unknown.exe")
                .to_string();

            Path::new(&raw_class_name)
                .file_stem()
                .and_then(|f| f.to_str())
                .unwrap_or("unknown")
                .to_string()
        } else {
            AppInfo::unknown_app_class()
        }
    }
}
