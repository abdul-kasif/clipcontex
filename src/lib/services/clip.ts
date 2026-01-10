// @ts-nocheck
import { writable, derived, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Clip } from "$lib/stores/types";

const EVT_CLIP_ADDED: string = "clip-added";
const EVT_CLIP_UPDATED: string = "clip-updated";
const EVT_CLIP_DELETED: string = "clip-deleted";
const EVT_HISTORY_CLEARED: string = "history-cleared";

// --- Core store: single source of truth ---
export const allClipsStore = writable<Clip[]>([]);

// Tauri Event Initialization
let eventInitialized: boolean = false;

export async function initClipEvents() {
  if (eventInitialized) return;
  eventInitialized = true;

  await listen<Clip>(EVT_CLIP_ADDED, (e) => {
    allClipsStore.update((clips) => {
      if (clips.some((c) => c.id === e.payload.id)) return clips;
      return [e.payload, ...clips];
    });
  });

  await listen<number>(EVT_CLIP_DELETED, (e) => {
    allClipsStore.update((clips) => clips.filter((c) => c.id !== e.payload));
  });

  await listen<number, boolean>(EVT_CLIP_UPDATED, (e) => {
    const [id, isPinned] = e.payload;

    allClipsStore.update((clips) =>
      clips.map((c) => (c.id === id ? { ...c, is_pinned: isPinned } : c)),
    );
  });

  await listen(EVT_HISTORY_CLEARED, () => {
    allClipsStore.set([]);
  });
}

// Safe Tauri invoke wrapper
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
    return [] as unknown as T;
  } finally {
    isLoading.set(false);
  }
}

// Public API
export async function loadClips(limit = 200) {
  const loaded = await safeInvoke<Clip[]>("list_recent_clips", { limit });
  allClipsStore.set(loaded);
}

export async function togglePin(id: number, isPinned: boolean) {
  await safeInvoke("toggle_pin_status", { id, isPinned });
}

export async function deleteClip(id: number) {
  await safeInvoke("remove_clip", { id });
}

export async function clearAllClips() {
  await safeInvoke("clear_clip_history");
}

export async function ignorePasting(content: string) {
  await safeInvoke("ignore_next_clip", { content });
}

// Derived UI stores
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
