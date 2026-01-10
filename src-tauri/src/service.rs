// src-tauri/src/service.rs
//! Application service layer.
//!
//! This module contains stateless business logic that coordinates between:
//! - Persistent storage (`storage`),
//! - User configuration (`config`),
//! - System capabilities (`system`),
//! - Tauri application state (`AppState`).
//!
//! Each submodule encapsulates a specific concern:
//! - [`clip`]: Clipboard history operations (CRUD, pinning).
//! - [`settings`]: Configuration management with side effects
//!   (e.g., global shortcuts, autostart).
//! - [`system`]: Platform-specific utilities (e.g., dependency checks).
//!
//! Services act as an anti-corruption layer between Tauri commands and domain logic,
//! ensuring clean separation of concerns and testable behavior.

pub mod clip;
pub mod settings;
pub mod system;
