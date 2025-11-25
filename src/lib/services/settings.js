import { invoke } from "@tauri-apps/api/core";
import toast from "svelte-french-toast";
import { theme } from "$lib/stores/theme";

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
    const response = await invoke("save_config", { settings: s });
    if (response === "success") {
      toast.success("Settings saved successfully", {
        duration: 1500,
        style:
          "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
      });
    }
  } catch (error) {
    console.error("Failed to save settings:", error);
    toast.error("Failed to save settings. Please try again", {
      duration: 1500,
      style:
        "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
    });
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
