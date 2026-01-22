import { invoke } from "@tauri-apps/api/core";
import toast from "svelte-french-toast";
import type { AppSettings } from "$lib/stores/types";
import { error } from "./clip";
import { showToast } from "$lib/utils/toast";

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
    showToast('success', 'Settings saved successfully!!');
  } catch (error) {
    showToast("error", "Failed to save settings, Please try again.");
  }
}

export async function completeOnboarding(): Promise<String | null> {
  try {
    await invoke("mark_onboarding_complete");
    return "ok";
  } catch (e) {
    const message = typeof error === "string" ? error : "Unknown error";
    showToast("error", "Failed to cmoplete onboarding. Please try again.");
  }
}
