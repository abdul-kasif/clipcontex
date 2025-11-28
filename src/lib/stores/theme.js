// src/lib/stores/theme.js
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { loadSettings } from "$lib/services/settings";

export const theme = writable("light");

/**
 * Apply the theme to <html data-theme="..."> and persist it
 */
function applyTheme(value) {
  document.documentElement.setAttribute("data-theme", value);
  localStorage.setItem("clipcontex-theme", value);
}

/**
 * Load theme from backend config or system preference
 */
async function initializeTheme() {
  try {
    const settings = await loadSettings();
    const mode = settings.darkMode ? "dark" : "light";
    theme.set(mode);
    applyTheme(mode);
  } catch (err) {
    console.warn("Failed to load theme from backend:", err);
    const systemPrefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;
    const fallback = systemPrefersDark ? "dark" : "light";
    theme.set(fallback);
    applyTheme(fallback);
  }

  // Listen for live theme updates from backend
  try {
    await listen("settings-updated", (event) => {
      const newSettings = event.payload;
      if (typeof newSettings.darkMode !== "undefined") {
        const mode = newSettings.darkMode ? "dark" : "light";
        theme.set(mode);
        applyTheme(mode);
      }
    });
  } catch (e) {
    console.warn("Failed to listen for settings updates:", e);
  }
}

/**
 * Subscribe to theme changes to reapply it dynamically.
 */
theme.subscribe((value) => applyTheme(value));

initializeTheme();
