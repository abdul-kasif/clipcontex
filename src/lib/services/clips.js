// @ts-nocheck
import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// --- Stores ---
export const clips = writable([]); // Recent (unpinned)
export const pinnedClips = writable([]); // Pinned
export const isLoading = writable(false);
export const error = writable(null);
export const noResults = writable(false);

let unlisten = null;
let allClips = []; // Full in-memory cache (shared for search)

// --- Initialize live event listener once ---
async function initEventListeners() {
  if (unlisten) return;

  try {
    unlisten = await listen("clip-added", (event) => {
      const newClip = event.payload;
      allClips = [newClip, ...allClips.filter((c) => c.id !== newClip.id)];

      if (newClip.is_pinned) {
        pinnedClips.update((prev) => [
          newClip,
          ...prev.filter((c) => c.id !== newClip.id),
        ]);
      } else {
        clips.update((prev) => [
          newClip,
          ...prev.filter((c) => c.id !== newClip.id),
        ]);
      }
    });
  } catch (err) {
    console.error("Failed to initialize event listeners:", err);
  }
}
initEventListeners();

// --- Safe invoke helper ---
async function safeInvoke(command, payload = {}) {
  try {
    isLoading.set(true);
    error.set(null);
    return await invoke(command, payload);
  } catch (err) {
    console.error(`Tauri invoke error: ${command}`, err);
    error.set(err.message || "Unknown error");
    return [];
  } finally {
    isLoading.set(false);
  }
}

// --- Load all clips ---
export async function loadClips(limit = 200) {
  const loaded = await safeInvoke("get_recent_clips", { limit });
  allClips = loaded;

  const pinned = loaded.filter((c) => c.is_pinned);
  const recent = loaded.filter((c) => !c.is_pinned);
  pinnedClips.set(pinned);
  clips.set(recent);
}

export function searchClips(query) {
  const q = query.trim().toLowerCase();

  if (!q) {
    // Reset to full list
    const pinned = allClips.filter((c) => c.is_pinned);
    const recent = allClips.filter((c) => !c.is_pinned);
    pinnedClips.set(pinned);
    clips.set(recent);
    noResults.set(false);
    return;
  }

  // Use lightweight substring search
  const results = allClips.filter((c) => {
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

  // if no results found, show all clips
  if (results.length === 0) {
    const pinned = allClips.filter((c) => c.is_pinned);
    const recent = allClips.filter((c) => !c.is_pinned);
    pinnedClips.set(pinned);
    clips.set(recent);
    noResults.set(true);
    return;
  }

  noResults.set(false);

  const pinned = results.filter((c) => c.is_pinned);
  const recent = results.filter((c) => !c.is_pinned);

  pinnedClips.set(pinned);
  clips.set(recent);
}

// --- Pin / Unpin ---
export async function togglePin(id, isPinned) {
  await safeInvoke("pin_clip", { id, isPinned });

  let movedClip = null;

  if (isPinned) {
    clips.update((prev) => {
      const idx = prev.findIndex((c) => c.id === id);
      if (idx >= 0) {
        movedClip = prev[idx];
        prev.splice(idx, 1);
      }
      return [...prev];
    });
    if (movedClip) {
      movedClip.is_pinned = true;
      pinnedClips.update((prev) => [movedClip, ...prev]);
    }
  } else {
    pinnedClips.update((prev) => {
      const idx = prev.findIndex((c) => c.id === id);
      if (idx >= 0) {
        movedClip = prev[idx];
        prev.splice(idx, 1);
      }
      return [...prev];
    });
    if (movedClip) {
      movedClip.is_pinned = false;
      clips.update((prev) => [movedClip, ...prev]);
    }
  }

  // Reflect in full cache
  const idx = allClips.findIndex((c) => c.id === id);
  if (idx >= 0) allClips[idx].is_pinned = isPinned;
}

// --- Delete Clip ---
export async function deleteClip(id) {
  await safeInvoke("delete_clip", { id });
  allClips = allClips.filter((c) => c.id !== id);
  clips.update((prev) => prev.filter((c) => c.id !== id));
  pinnedClips.update((prev) => prev.filter((c) => c.id !== id));
}

// --- Clear All ---
export async function clearAllClips() {
  await safeInvoke("clear_history");
  allClips = [];
  clips.set([]);
  pinnedClips.set([]);
}
