// src-tauri/src/core/global_shortcut.rs
//! Global shortcut management for the quick picker.
//!
//! This module provides end-to-end handling of the user-configurable global shortcut
//! that triggers the quick picker window. It consists of three responsibilities:
//!
//! - **Mapping**: Converting user-defined shortcut strings (`Ctrl+Shift+V`) into
//!   platform-native shortcut objects ([`shortcut_mapper`]).
//! - **Registration**: Registering the shortcut with the operating system
//!   ([`shortcut_register`]).
//! - **Handling**: Listening for shortcut press events and triggering UI actions
//!   ([`shortcut_handler`]).
//!
//! The public API exposes only the essential setup functions used during application
//! initialization.

pub mod shortcut_handler;
pub mod shortcut_mapper;
pub mod shortcut_register;

pub use shortcut_handler::handle_quick_picker_shortcut;
pub use shortcut_mapper::shortcut_from_config;
pub use shortcut_register::register_quick_picker_shortcut;

