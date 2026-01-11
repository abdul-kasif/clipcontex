// src-tauri/src/context.rs
//! Cross-platform contextual metadata extraction for clipboard entries.
//!
//! This module enriches clipboard history with two key types of contextual information:
//!
//! 1. **Active Application Context**:  
//!    Detects the currently focused application's window title and normalized class name.
//!    - **Linux**: Supports both Wayland (`kdotool`) and X11 (`xdotool`/`xprop`).
//!    - **Windows**: Uses native Win32 APIs.
//!    - **Other platforms**: Returns placeholder values.
//!
//! 2. **Automatic Tag Generation**:  
//!    Analyzes clipboard content and application context to generate semantic tags like:
//!    - `#url`, `#email`, `#code` (content-based)
//!    - `#editor`, `#browser`, `#terminal` (app-based)
//!
//! All operations are designed to be **fail-safe**: any error results in neutral fallbacks
//! (e.g., `"Unknown"` for app info, no tags for invalid content), ensuring the core clipboard
//! functionality remains uninterrupted.

pub mod app_info;
pub mod auto_tags;
pub mod normalize_app_name;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;
