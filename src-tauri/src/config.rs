// src-tauri/src/config.rs
//! Application configuration management.
//!
//! This module handles loading and saving user settings from/to
//! `~/.clipcontex/config.json`. It ensures atomic writes, graceful degradation
//! on corruption, and provides sensible defaults.

// ===== Imports =====

use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use tempfile::NamedTempFile;
use tracing::warn;

// ===== Domain Types =====

/// Application-wide user settings.
///
/// Stored in `~/.clipcontex/config.json` using camelCase JSON keys.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Number of days after which unpinned clips are automatically deleted.
    pub auto_clean_days: u32,

    /// Maximum number of clips to retain in history (after cleanup).
    pub max_history_size: u32,

    /// List of application names whose clipboard content should be ignored.
    ///
    /// Example: `["BitWarden", "1Password"]`.
    pub ignored_apps: Vec<String>,

    /// Indicates whether the user is running the app for the first time.
    ///
    /// This flag is typically reset by the frontend after onboarding.
    pub is_new_user: bool,

    /// Whether the app should start automatically with the operating system.
    pub is_autostart_enabled: bool,

    /// Keyboard shortcut to open the quick picker UI.
    ///
    /// Format:
    /// - `modifiers`: list of modifier keys (e.g., `"Ctrl"`, `"Shift"`, `"Alt"`, `"Cmd"`).
    /// - `key`: single character or named key (e.g., `"v"`, `"Space"`).
    ///
    /// ⚠️ Invalid shortcuts may fail during Tauri hotkey registration.
    pub quick_picker_shortcut: ShortcutConfig,
}

/// Represents a keyboard shortcut configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutConfig {
    /// Modifier keys (e.g., `["Ctrl", "Shift"]`).
    pub modifiers: Vec<String>,
    /// Main key (e.g., `"v"`).
    pub key: String,
}

// ===== Default Implementation =====

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_clean_days: 30,
            max_history_size: 200,
            ignored_apps: vec!["BitWarden".to_string(), "1Password".to_string()],
            is_new_user: true,
            is_autostart_enabled: true,
            quick_picker_shortcut: ShortcutConfig {
                modifiers: vec!["Ctrl".into(), "Shift".into()],
                key: "v".into(),
            },
        }
    }
}

// ===== Public API =====

/// Loads user settings from disk.
///
/// If the config file is missing or invalid, this function:
/// 1. Logs a warning,
/// 2. Creates a new config file with default values,
/// 3. Returns the default settings.
///
/// # Errors
///
/// Returns an error only if I/O operations fail (e.g., permission denied).
pub fn load_config() -> Result<Settings, String> {
    let path = config_file_path();

    if !path.exists() {
        let defaults = Settings::default();
        save_config(&defaults)?;
        return Ok(defaults);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read settings file {:?}: {}", path, e))?;

    match serde_json::from_str::<Settings>(&content) {
        Ok(cfg) => Ok(cfg),
        Err(e) => {
            warn!(
                "Invalid config format at {:?}: {}. Reverting to defaults.",
                path, e
            );
            let defaults = Settings::default();
            save_config(&defaults)?;
            Ok(defaults)
        }
    }
}

/// Saves settings to disk atomically.
///
/// Uses a temporary file + rename to prevent corruption on crash or power loss.
///
/// # Errors
///
/// Returns an error if any step fails (directory creation, temp file, write, persist).
pub fn save_config(settings: &Settings) -> Result<(), String> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let path = config_file_path();
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    let mut tmp = NamedTempFile::new_in(&dir)
        .map_err(|e| format!("Failed to create temporary file: {}", e))?;

    tmp.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write temp config: {}", e))?;

    // No need to flush explicitly — `persist` handles it
    tmp.persist(&path)
        .map_err(|e| format!("Failed to persist settings: {}", e))?;

    Ok(())
}

/// Returns the path to the configuration directory: `~/.clipcontex`
pub fn config_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| {
            // Fallback: use current directory only in development
            // In production, this should rarely happen
            warn!("Unable to determine home directory; using current directory for config");
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        })
        .join(".clipcontex")
}

// ===== Helper Functions =====
/// Returns the full path to the config file: `~/.clipcontex/config.json`
fn config_file_path() -> PathBuf {
    config_dir().join("config.json")
}
