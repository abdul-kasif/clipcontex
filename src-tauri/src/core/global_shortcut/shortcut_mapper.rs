// src-tauri/src/core/global_shortcut/shortcut_mapper.rs
//! Maps user-configurable shortcut strings to Tauri shortcut objects.
//!
//! Supports alphabetic keys (A-Z) and common modifiers (Ctrl, Shift, Alt, Super).
//! Does not support function keys, symbols, or international layouts.

use crate::config::ShortcutConfig;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

/// Converts a [`ShortcutConfig`] into a Tauri [`Shortcut`].
///
/// Returns `None` if the key is not supported (e.g., "Space", "Enter").
/// Currently only supports alphabetic keys A-Z.
pub fn shortcut_from_config(cfg: &ShortcutConfig) -> Option<Shortcut> {
    let mut modifiers = Modifiers::empty();

    for m in &cfg.modifiers {
        match m.as_str() {
            "Ctrl" => modifiers |= Modifiers::CONTROL,
            "Shift" => modifiers |= Modifiers::SHIFT,
            "Alt" => modifiers |= Modifiers::ALT,
            "Super" => modifiers |= Modifiers::META,
            _ => {}
        }
    }

    let key = match cfg.key.to_uppercase().as_str() {
        "A" => Code::KeyA,
        "B" => Code::KeyB,
        "C" => Code::KeyC,
        "D" => Code::KeyD,
        "E" => Code::KeyE,
        "F" => Code::KeyF,
        "G" => Code::KeyG,
        "H" => Code::KeyH,
        "I" => Code::KeyI,
        "J" => Code::KeyJ,
        "K" => Code::KeyK,
        "L" => Code::KeyL,
        "M" => Code::KeyM,
        "N" => Code::KeyN,
        "O" => Code::KeyO,
        "P" => Code::KeyP,
        "Q" => Code::KeyQ,
        "R" => Code::KeyR,
        "S" => Code::KeyS,
        "T" => Code::KeyT,
        "U" => Code::KeyU,
        "V" => Code::KeyV,
        "W" => Code::KeyW,
        "X" => Code::KeyX,
        "Y" => Code::KeyY,
        "Z" => Code::KeyZ,
        _ => return None,
    };

    Some(Shortcut::new(Some(modifiers), key))
}
