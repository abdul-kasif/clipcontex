// src-tauri/src/config.rs
// ===== Imports =====
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use tempfile::NamedTempFile;

// ===== Domain Type =====
/// Application settings persisted in `~/.clipcontex/config.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub auto_clean_days: u32,
    pub max_history_size: u32,
    pub ignored_apps: Vec<String>,
    pub is_new_user: bool,
    pub is_autostart_enabled: bool,
    pub quick_picker_shortcut: ShortcutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutConfig {
    pub modifiers: Vec<String>,
    pub key: String,
}

// ===== Settings Implementation =====
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

// ===== Mechanism =====
/// Loads user settings from disk.  
/// If missing or invalid, falls back to defaults and ensures file creation.
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
            eprintln!(
                "Invalid config format at {:?}: {}. Reverting to defaults.",
                path, e
            );
            let defaults = Settings::default();
            save_config(&defaults)?;
            Ok(defaults)
        }
    }
}

/// Saves settings atomically via a temporary file and rename.
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
    tmp.flush()
        .map_err(|e| format!("Failed to flush temp config: {}", e))?;

    tmp.persist(&path)
        .map_err(|e| format!("Failed to persist settings: {}", e))?;

    Ok(())
}

// ===== Helper Functions =====
fn config_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".clipcontex")
}

fn config_file_path() -> PathBuf {
    config_dir().join("config.json")
}
