import { writable } from "svelte/store";
import { setBoolean, getBoolean } from "$lib/stores/uiPreference";
import { emit } from "@tauri-apps/api/event";

export type Theme = "light" | "dark";

const STORAGE_KEY = "darkMode";

export const theme = writable<Theme>("light");

let isInitialized: boolean = false;

function applyTheme(value: Theme) {
  document.documentElement.setAttribute("data-theme", value);
}

function getSystemThemme(): Theme {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

async function initializeTheme() {
  const storedDarkMode = await getBoolean(STORAGE_KEY, null as any);

  let resolvedTheme: Theme;

  if (storedDarkMode === null) {
    resolvedTheme = getSystemThemme();
    await setBoolean(STORAGE_KEY, resolvedTheme === "dark");
  } else {
    resolvedTheme = storedDarkMode ? "dark" : "light";
  }
  theme.set(resolvedTheme);
  applyTheme(resolvedTheme);

  isInitialized = true;
}

theme.subscribe(async (value: Theme) => {
  applyTheme(value);
  if (!isInitialized) return;
  await setBoolean(STORAGE_KEY, value === "dark");

  await emit("theme-change", value);
});

initializeTheme();
