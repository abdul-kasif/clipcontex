import { invoke } from "@tauri-apps/api/core";
import toast from "svelte-french-toast";
import type { AppSettings } from "$lib/stores/types";

const DEFAULT_SETTINGS: AppSettings = {
  autoCleanDays: 30,
  maxHistorySize: 200,
  darkMode: false,
  ignoredApps: ["Bitwarden", "1Password"],
  isNewUser: true,
  isAutostartEnabled: true,
};

function convertIgnoredApps(ignoredApps: any) {
  let newIgnoredApps: string[] = [];
  if (Array.isArray(ignoredApps)) {
    newIgnoredApps = ignoredApps;
  } else if (typeof ignoredApps === "string") {
    newIgnoredApps = ignoredApps
      .split(",")
      .map((s: string) => s.trim())
      .filter((s: string) => s.length > 0);
  } else {
    newIgnoredApps = [...DEFAULT_SETTINGS.ignoredApps];
  }
  return newIgnoredApps;
}

export async function loadSettings(): Promise<AppSettings> {
  try {
    const config = (await invoke("load_config")) as AppSettings;
    let ignoredApps = convertIgnoredApps(config.ignoredApps);
    console.log("Starting point", ignoredApps);
    return {
      autoCleanDays: config.autoCleanDays ?? DEFAULT_SETTINGS.autoCleanDays,
      maxHistorySize: config.maxHistorySize ?? DEFAULT_SETTINGS.maxHistorySize,
      darkMode: config.darkMode ?? DEFAULT_SETTINGS.darkMode,
      ignoredApps,
      isNewUser: config.isNewUser ?? DEFAULT_SETTINGS.isNewUser,
      isAutostartEnabled:
        config.isAutostartEnabled ?? DEFAULT_SETTINGS.isAutostartEnabled,
    };
  } catch (error) {
    console.warn("Failed to load config, using defaults:", error);
    const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    return { ...DEFAULT_SETTINGS, darkMode: isDark };
  }
}

export async function saveSettings(newSettings: AppSettings) {
  try {
    const response = await invoke("save_config", { settings: newSettings });
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
      type: "string[]",
      default: "['Bitwarden','1Password']",
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
