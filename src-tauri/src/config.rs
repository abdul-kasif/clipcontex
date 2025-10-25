use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use tempfile::NamedTempFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub auto_clean_days: u32,
    pub max_history_size: u32,
    pub dark_mode: bool,
    pub ignored_apps: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_clean_days: 30,
            max_history_size: 200,
            dark_mode: false,
            ignored_apps: vec!["BitWarden".to_string(), "1Password".to_string()],
        }
    }
}

pub fn config_dir() -> PathBuf {
    let db_path = home_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.join(".clipcontex")
}

pub fn config_file_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_settings() -> Result<Settings, String> {
    let path = config_file_path();
    if !path.exists() {
        return Ok(Settings::default());
    }

    let s = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read settings file at {:?}: {}", path, e))?;
    match serde_json::from_str::<Settings>(&s) {
        Ok(config) => Ok(config),
        Err(e) => {
            eprintln!(
                "Failed to parse config file at {:?}: {}. Using defaults.",
                path, e
            );
            Ok(Settings::default())
        }
    }
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let path = config_file_path();
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    // Write to temp file first, then rename (atomic on most systems)
    let mut temp_file =
        NamedTempFile::new_in(&dir).map_err(|e| format!("Failed to create temp file: {}", e))?;
    temp_file
        .write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write to temp file: {}", e))?;
    temp_file
        .flush()
        .map_err(|e| format!("Failed to flush temp file: {}", e))?;

    temp_file
        .persist(&path)
        .map_err(|e| format!("Failed to persist settings file: {}", e))?;

    Ok(())
}
