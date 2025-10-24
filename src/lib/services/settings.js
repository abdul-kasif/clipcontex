import { invoke } from '@tauri-apps/api/core';

const DEFAULT_SETTINGS = {
  autoCleanDays: 30,
  maxHistorySize: 200,
  darkMode: false,
  ignoredApps: "Bitwarden,1Password"
};

export async function loadSettings() {
  try {
    // Try to load from config file
    const config = await invoke('load_config');
    return { ...DEFAULT_SETTINGS, ...config };
  } catch (error) {
    console.warn('Failed to load config, using defaults:', error);
    // Check system preference for dark mode
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    return { ...DEFAULT_SETTINGS, darkMode: isDark };
  }
}

export async function saveSettings(settings) {
  try {
    await invoke('save_config', { settings });
    alert('Settings saved successfully!');
  } catch (error) {
    console.error('Failed to save settings:', error);
    alert('Failed to save settings. Please try again.');
  }
}

export function getSettingsSchema() {
  return {
    autoCleanDays: {
      type: 'number',
      min: 1,
      max: 365,
      default: 30
    },
    maxHistorySize: {
      type: 'number',
      min: 10,
      max: 1000,
      default: 200
    },
    darkMode: {
      type: 'boolean',
      default: false
    },
    ignoredApps: {
      type: 'string',
      default: "Bitwarden,1Password"
    }
  };
}