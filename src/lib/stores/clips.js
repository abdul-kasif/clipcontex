// @ts-nocheck
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const clips = writable([]);        // Recent clips (unpinned)
export const pinnedClips = writable([]);  // Pinned clips
export const searchQuery = writable("");
export const isLoading = writable(false);
export const error = writable(null);

let unlisten = null;

// --- Initialize event listeners ---
async function initEventListeners() {
  if (unlisten) return;

  try {
    unlisten = await listen("clip-added", (event) => {
      const newClip = event.payload;
      console.log("New clip received:", newClip);

      // Add to pinned or recent
      if (newClip.is_pinned) {
        pinnedClips.update(prev => [newClip, ...prev.filter(c => c.id !== newClip.id)]);
      } else {
        clips.update(prev => [newClip, ...prev.filter(c => c.id !== newClip.id)]);
      }
    });

    console.log("Event listener initialized");
  } catch (err) {
    console.error("Failed to initialize event listeners:", err);
  }
}
initEventListeners();

// --- Safe invoke wrapper ---
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

// --- Load all clips initially ---
export async function loadClips(limit = 200) {
  const allClips = await safeInvoke("get_recent_clips", { limit });
  const pinned = allClips.filter(c => c.is_pinned);
  const recent = allClips.filter(c => !c.is_pinned);
  pinnedClips.set(pinned);
  clips.set(recent);
}

// --- Search clips ---
export async function searchClips(query) {
  searchQuery.set(query);
  if (!query.trim()) return loadClips();

  const results = await safeInvoke("search_clips", { query, limit: 50 });
  const pinned = results.filter(c => c.is_pinned);
  const recent = results.filter(c => !c.is_pinned);
  pinnedClips.set(pinned);
  clips.set(recent);
}

// --- Pin/unpin a clip ---
export async function togglePin(id, isPinned) {
  await safeInvoke("pin_clip", { id, isPinned: isPinned });

  // Move the clip between lists locally without full reload
  if (isPinned) {
    let movedClip = null;
    clips.update(prev => {
      const idx = prev.findIndex(c => c.id === id);
      if (idx >= 0) {
        movedClip = prev[idx];
        prev.splice(idx, 1);
      }
      return prev;
    });
    if (movedClip) pinnedClips.update(prev => [ { ...movedClip, is_pinned: true }, ...prev ]);
  } else {
    let movedClip = null;
    pinnedClips.update(prev => {
      const idx = prev.findIndex(c => c.id === id);
      if (idx >= 0) {
        movedClip = prev[idx];
        prev.splice(idx, 1);
      }
      return prev;
    });
    if (movedClip) clips.update(prev => [ { ...movedClip, is_pinned: false }, ...prev ]);
  }
}

// --- Delete a clip ---
export async function deleteClip(id) {
  await safeInvoke("delete_clip", { id });
  clips.update(prev => prev.filter(c => c.id !== id));
  pinnedClips.update(prev => prev.filter(c => c.id !== id));
}

// --- Clear all clips ---
export async function clearAllClips() {
  await safeInvoke("clear_history");
  clips.set([]);
  pinnedClips.set([]);
}

// --- Capture current clipboard manually ---
export async function captureCurrentClip() {
  try {
    isLoading.set(true);
    error.set(null);

    const newClip = await safeInvoke("capture_current_clip");

    // Automatically added via event listener
    console.log("Captured new clip:", newClip);
    return newClip;
  } catch (err) {
    console.error("Tauri invoke error: capture_current_clip", err);
    error.set(err.message || "Failed to capture clipboard");
  } finally {
    isLoading.set(false);
  }
}
