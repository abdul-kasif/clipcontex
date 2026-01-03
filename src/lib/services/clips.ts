// @ts-nocheck
import { writable, derived, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Clip } from "$lib/stores/types";

// --- Core store: single source of truth ---
export const allClipsStore = writable<Clip[]>([]);

// --- Derived UI stores ---
export const searchTerm = writable("");
const normalizedQuery = derived(searchTerm, (term) =>
  term.trim().toLowerCase(),
);

// Filtered clips
const filteredClips = derived(
  [allClipsStore, normalizedQuery],
  ([$allClips, q]) => {
    if (!q) return $allClips;

    return $allClips.filter((c) => {
      const content = c.content?.toLowerCase() ?? "";
      const app = c.app_name?.toLowerCase() ?? "";
      const title = c.window_title?.toLowerCase() ?? "";
      const auto = c.auto_tags?.toLowerCase() ?? "";
      return (
        content.includes(q) ||
        app.includes(q) ||
        title.includes(q) ||
        auto.includes(q)
      );
    });
  },
);

// Split into pinned / recent
export const pinnedClips = derived(filteredClips, (clips) =>
  clips.filter((c) => c.is_pinned),
);
export const clips = derived(filteredClips, (clips) =>
  clips.filter((c) => !c.is_pinned),
);

export const isLoading = writable(false);
export const error = writable<string | null>(null);
export const noResults = derived(
  [filteredClips, normalizedQuery],
  ([$filtered, q]) => q.length > 0 && $filtered.length === 0,
);
export const searchQueryLength = derived(normalizedQuery, (q) => q.length);

// --- Tauri event listener ---
let unlisten: (() => void) | null = null;

async function initEventListeners() {
  if (unlisten) return;

  try {
    unlisten = await listen("clip-added", (event) => {
      const newClip = event.payload as Clip;
      // Update core store
      allClipsStore.update((clips) => {
        // Avoid duplicates
        const exists = clips.find((c) => c.id === newClip.id);
        if (exists) {
          // Replace if exists (unlikely, but safe)
          return clips.map((c) => (c.id === newClip.id ? newClip : c));
        }
        // Prepend new clip
        return [newClip, ...clips];
      });
    });
  } catch (err) {
    console.error("Failed to initialize Tauri event listeners:", err);
  }
}
initEventListeners();

// --- Safe Tauri invoke wrapper ---
async function safeInvoke<T = any>(
  command: string,
  payload: Record<string, unknown> = {},
): Promise<T> {
  try {
    isLoading.set(true);
    error.set(null);
    return await invoke<T>(command, payload);
  } catch (err: any) {
    console.error(`Tauri invoke error: ${command}`, err);
    const message = err?.message || "Unknown error";
    error.set(message);
    return [] as unknown as T; // safe fallback for array-returning commands
  } finally {
    isLoading.set(false);
  }
}

// --- Public API ---

export async function loadClips(limit = 200) {
  const loaded = await safeInvoke<Clip[]>("get_recent_clips", { limit });
  // Ensure consistent order: most recent first
  allClipsStore.set(loaded);
}

export async function togglePin(id: number, isPinned: boolean) {
  await safeInvoke("pin_clip", { id, isPinned });

  // Update core store
  allClipsStore.update((clips) =>
    clips.map((c) => (c.id === id ? { ...c, is_pinned: isPinned } : c)),
  );
}

export async function deleteClip(id: number) {
  await safeInvoke("delete_clip", { id });
  allClipsStore.update((clips) => clips.filter((c) => c.id !== id));
}

export async function clearAllClips() {
  await safeInvoke("clear_history");
  allClipsStore.set([]);
}

// Optional: if you ever need to trigger a manual refresh (unlikely)
export function refreshFromCache() {
  // Not needed — reactive by design
}
