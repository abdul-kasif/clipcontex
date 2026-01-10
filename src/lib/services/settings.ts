import { invoke } from "@tauri-apps/api/core";
import toast from "svelte-french-toast";
import type { AppSettings } from "$lib/stores/types";
import { error } from "./clips";

const DEFAULT_SETTINGS: AppSettings = {
  autoCleanDays: 30,
  maxHistorySize: 200,
  ignoredApps: ["Bitwarden", "1Password"],
  isNewUser: true,
  isAutostartEnabled: true,
  quickPickerShortcut: {
    modifiers: ["Ctrl", "Shift"],
    key: "v",
  },
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
    const config = (await invoke("load_settings")) as AppSettings;
    let ignoredApps = convertIgnoredApps(config.ignoredApps);
    console.log("Starting point", ignoredApps);
    return {
      autoCleanDays: config.autoCleanDays ?? DEFAULT_SETTINGS.autoCleanDays,
      maxHistorySize: config.maxHistorySize ?? DEFAULT_SETTINGS.maxHistorySize,
      ignoredApps,
      isNewUser: config.isNewUser ?? DEFAULT_SETTINGS.isNewUser,
      isAutostartEnabled:
        config.isAutostartEnabled ?? DEFAULT_SETTINGS.isAutostartEnabled,
      quickPickerShortcut:
        config.quickPickerShortcut ?? DEFAULT_SETTINGS.quickPickerShortcut,
    };
  } catch (error) {
    console.warn("Failed to load config, using defaults:", error);
    return DEFAULT_SETTINGS;
  }
}

export async function saveSettings(newSettings: AppSettings) {
  try {
    await invoke("save_settings", { settings: newSettings });
    toast.success("Settings saved successfully", {
      duration: 1500,
      style:
        "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
    });
  } catch (error) {
    toast.error(`Failed to save settings. Please try again`, {
      duration: 1500,
      style:
        "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
    });
  }
}

export async function completeOnboarding(): Promise<String | null> {
  try {
    await invoke("mark_onboarding_complete");
    return "ok";
  } catch (e) {
    const message = typeof error === "string" ? error : "Unknown error";
    toast.error(`Failed to complete onboarding: ${message}`, {
      duration: 1500,
      style:
        "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
    });
  }
}
