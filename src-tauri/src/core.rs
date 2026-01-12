// src-tauri/src/core.rs
//! Core application lifecycle and platform integration.
//!
//! This module orchestrates:
//! - Application startup and state initialization ([`setup`]).
//! - Window management ([`window_creation`]).
//! - System tray integration ([`system_tray`]).
//! - Background cleanup tasks ([`cleanup`]).
//! - Global shortcut handling ([`global_shortcut`]).
//!
//! It serves as the glue between Tauri's runtime and your domain logic.

pub mod cleanup;
pub mod global_shortcut;
pub mod setup;
pub mod system_tray;
pub mod window_creation;
