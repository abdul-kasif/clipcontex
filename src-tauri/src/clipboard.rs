// src-tauri/src/clipboard.rs
//! Clipboard monitoring and processing subsystem.
//!
//! This module provides intelligent clipboard history capture with:
//! - **Deduplication**: Avoids saving repeated content within a time window.
//! - **Self-trigger prevention**: Ignores clipboard changes caused by the app itself (e.g., paste actions).
//! - **Background polling**: Monitors clipboard changes without blocking the main thread.
//!
//! The system is composed of two key components:
//! - [`dedupe`]: In-memory deduplication logic.
//! - [`watcher`]: Background thread that polls the system clipboard and emits events.
//!
//! Designed to be lightweight, robust, and respectful of system resources.

pub mod dedupe;
pub mod watcher;

