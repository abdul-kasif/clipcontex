// src-tauri/src/state.rs
//! Application-wide shared state management.
//!
//! The [`AppState`] struct holds all long-lived, shared resources required by the application:
//! - Persistent storage (`ClipStore`)
//! - User configuration (`Settings`)
//! - Global shortcut registration
//! - Background clipboard watcher
//!
//! It is designed to be:
//! - **Thread-safe**: All fields are wrapped in appropriate synchronization primitives.
//! - **Lifecycle-aware**: Gracefully shuts down background tasks on drop.
//! - **Resilient**: Falls back to safe defaults on initialization errors.

use std::sync::{Arc, Mutex, RwLock};

use tauri_plugin_global_shortcut::Shortcut;
use tracing::{error, info, warn};

use crate::{
    clipboard::watcher::ClipboardWatcherHandle,
    config::{config_dir, Settings},
    core::global_shortcut::shortcut_from_config,
    service::settings::load_settings,
    storage::ClipStore,
};

/// Shared application state accessible from Tauri commands and services.
///
/// This struct is stored in Tauri's managed state and passed to command handlers.
/// All fields are wrapped in `Arc` and appropriate locks to ensure thread safety.
pub struct AppState {
    /// Persistent storage for clipboard history.
    pub clip_store: Arc<ClipStore>,
    /// Handle to the background clipboard watcher thread.
    ///
    /// Wrapped in `Mutex<Option<...>>` because the watcher is started after app setup.
    pub watcher_handle: Arc<Mutex<Option<ClipboardWatcherHandle>>>,
    /// Current user settings, loaded from disk.
    ///
    /// Uses `RwLock` to allow concurrent reads (common) and exclusive writes (rare).
    pub settings: Arc<RwLock<Settings>>,
    /// Currently registered global shortcut for the quick picker.
    ///
    /// `None` if shortcut registration failed or was disabled.
    pub quick_picker_shortcut: Arc<RwLock<Option<Shortcut>>>,
}

impl AppState {
    /// Initializes application state with persistent storage and user settings.
    ///
    /// # Initialization Steps
    ///
    /// 1. Opens or creates the SQLite database at `~/.clipcontex/clipcontex.db`.
    ///    On failure, falls back to an in-memory database.
    /// 2. Loads user settings from `~/.clipcontex/config.json`, using defaults if missing/invalid.
    /// 3. Prepares global shortcut registration (actual registration happens later).
    ///
    /// # Panics
    ///
    /// Only panics if even the in-memory database fails to initialize (should never happen).
    pub fn new() -> Self {
        let db_path = config_dir().join("clipcontex.db");

        let store = match ClipStore::new(&db_path) {
            Ok(store) => {
                info!("ClipStore initialized at {:?}", db_path);
                store
            }
            Err(e) => {
                error!(
                    "Failed to initialize ClipStore at {:?}: {}. Using in-memory fallback.",
                    db_path, e
                );
                ClipStore::new(":memory:").expect("In-memory database must always succeed")
            }
        };

        let settings = match load_settings() {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to load settings, using defaults: {}", e);
                Settings::default()
            }
        };

        let initial_shortcut = shortcut_from_config(&settings.quick_picker_shortcut);

        Self {
            clip_store: Arc::new(store),
            watcher_handle: Arc::new(Mutex::new(None)),
            settings: Arc::new(RwLock::new(settings)),
            quick_picker_shortcut: Arc::new(RwLock::new(initial_shortcut)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AppState {
    /// Gracefully shuts down the clipboard watcher on application exit.
    ///
    /// Attempts to stop the background thread without blocking indefinitely.
    /// If the lock is held by another thread, logs a warning and proceeds.
    fn drop(&mut self) {
        if let Ok(mut handle_guard) = self.watcher_handle.try_lock() {
            if let Some(mut handle) = handle_guard.take() {
                info!("Requesting clipboard watcher shutdown...");
                handle.stop();
                info!("Clipboard watcher shutdown complete.");
            }
        } else {
            warn!("Could not acquire watcher lock during shutdown; thread may leak.");
        }
    }
}
