import { invoke } from "@tauri-apps/api/core";

const DEFAULT_SETTINGS = {
  autoCleanDays: 30,
  maxHistorySize: 200,
  darkMode: false,
  ignoredApps: "Bitwarden,1Password",
  isNewUser: true,
  isAutostartEnabled: true,
};

export async function loadSettings() {
  try {
    const config = await invoke("load_config");
    console.log("Settings loaded", config);
    // config.ignoredApps may be an array
    return {
      autoCleanDays: config.autoCleanDays,
      maxHistorySize: config.maxHistorySize,
      darkMode: config.darkMode,
      ignoredApps: Array.isArray(config.ignoredApps)
        ? config.ignoredApps.join(",")
        : config.ignoredApps || "",
      isNewUser: config.isNewUser,
      isAutostartEnabled: config.isAutostartEnabled,
    };
  } catch (error) {
    console.warn("Failed to load config, using defaults:", error);
    // Check system preference for dark mode
    const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    return { ...DEFAULT_SETTINGS, darkMode: isDark };
  }
}

export async function saveSettings(settings) {
  const s = {
    ...settings,
    ignoredApps:
      typeof settings.ignoredApps === "string"
        ? settings.ignoredApps
            .split(",")
            .map((s) => s.trim())
            .filter(Boolean)
        : settings.ignoredApps,
  };

  try {
    await invoke("save_config", { settings: s });
  } catch (error) {
    console.error("Failed to save settings:", error);
    alert("Failed to save settings. Please try again.");
  }
}

export function getSettingsSchema() {
  return {
    autoCleanDays: {
      type: "number",
      min: 1,
      max: 365,
      default: 30,
    },
    maxHistorySize: {
      type: "number",
      min: 10,
      max: 1000,
      default: 200,
    },
    darkMode: {
      type: "boolean",
      default: false,
    },
    ignoredApps: {
      type: "string",
      default: "Bitwarden,1Password",
    },
    isNewUser: {
      type: "boolean",
      default: true,
    },
    isAutostartEnabled: {
      type: "boolean",
      default: true,
    },
  };
}

