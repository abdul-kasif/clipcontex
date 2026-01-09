// ===== Imports =====
use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tracing::info;

// ===== Crates =====
use crate::{
    command::AppState,
    config::{load_config, save_config, Settings},
    core::global_shortcut::shortcut_from_config,
    error::AppError,
};

// ===== Public API =====
pub fn load_settings() -> Result<Settings, AppError> {
    load_config().map_err(|e| AppError::Config(e.to_string()))
}

pub fn update_settings(
    app_handle: &AppHandle,
    app_state: &AppState,
    settings: &Settings,
) -> Result<(), AppError> {
    let old_settings = read_settings(app_state)?;

    if old_settings.quick_picker_shortcut != settings.quick_picker_shortcut {
        let old_shortcut = shortcut_from_config(&old_settings.quick_picker_shortcut);

        let new_shortcut = shortcut_from_config(&settings.quick_picker_shortcut)
            .ok_or_else(|| AppError::Shortcut("Invalid shortcut".into()))?;

        update_quick_picker_shortcut(app_handle, app_state, old_shortcut, new_shortcut)?;
    }

    save_config(settings).map_err(|e| AppError::Config(e.to_string()))?;

    sync_autostart(app_handle, settings.is_autostart_enabled)?;

    write_settings(app_state, settings.clone())?;

    Ok(())
}

pub fn mark_onboarding_complete(app_state: &AppState) -> Result<(), AppError> {
    let mut settings = app_state
        .settings
        .write()
        .map_err(|e| AppError::Config(e.to_string()))?;

    if settings.is_new_user {
        settings.is_new_user = false;
        save_config(&settings).map_err(|e| AppError::Config(e.to_string()))?;
    }

    Ok(())
}

// ===== Helper Functions =====
fn update_quick_picker_shortcut(
    app_handle: &AppHandle,
    app_state: &AppState,
    old_shortcut: Option<Shortcut>,
    new_shortcut: Shortcut,
) -> Result<(), AppError> {
    if let Some(old) = old_shortcut {
        app_handle
            .global_shortcut()
            .unregister(old)
            .map_err(|e| AppError::Shortcut(e.to_string()))?;
    }

    app_handle
        .global_shortcut()
        .register(new_shortcut)
        .map_err(|e| AppError::Shortcut(e.to_string()))?;

    app_state
        .quick_picker_shortcut
        .write()
        .map(|mut guard| *guard = Some(new_shortcut))
        .map_err(|_| AppError::Shortcut("Shortcut lock poisoned".to_string()))?;

    Ok(())
}

fn sync_autostart(app_handle: &AppHandle, enabled: bool) -> Result<(), AppError> {
    let launcher = app_handle.autolaunch();

    let current = launcher
        .is_enabled()
        .map_err(|e| AppError::Config(e.to_string()))?;

    match (current, enabled) {
        (false, true) => {
            launcher
                .enable()
                .map_err(|e| AppError::Config(e.to_string()))?;
            info!("Autostart enabled");
        }
        (true, false) => {
            launcher
                .disable()
                .map_err(|e| AppError::Config(e.to_string()))?;
            info!("Autostart disabled");
        }
        _ => {
            info!("Autostart already in desired state");
        }
    }
    Ok(())
}

fn read_settings(app_state: &AppState) -> Result<Settings, AppError> {
    app_state
        .settings
        .read()
        .map(|guard| guard.clone())
        .map_err(|_| AppError::Config("Settings lock poisoned".to_string()))
}

fn write_settings(app_state: &AppState, new_settings: Settings) -> Result<(), AppError> {
    app_state
        .settings
        .write()
        .map(|mut guard| {
            *guard = new_settings;
        })
        .map_err(|_| AppError::Config("Settings lock poisoned".to_string()))
}

