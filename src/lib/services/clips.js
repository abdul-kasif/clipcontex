// @ts-nocheck
import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Fuse from "fuse.js";

export const clips = writable([]);        // Recent clips (unpinned)
export const pinnedClips = writable([]);  // Pinned clips
export const searchQuery = writable("");
export const isLoading = writable(false);
export const error = writable(null);

let unlisten = null;
let allClips = []; // Keep a full cache
let fuse = null;   // Fuse.js instance

// --- Initialize event listeners ---
async function initEventListeners() {
  if (unlisten) return;

  try {
    unlisten = await listen("clip-added", (event) => {
      const newClip = event.payload;

      // Update full list and Fuse index
      allClips = [newClip, ...allClips.filter(c => c.id !== newClip.id)];
      rebuildFuseIndex();

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

// --- Build Fuse index ---
function rebuildFuseIndex() {
  fuse = new Fuse(allClips, {
    keys: [
      "content",
      "app_name",
      "window_title",
      "auto_tags",
      "manual_tags"
    ],
    includeScore: true,
    threshold: 0.3, // Adjust for fuzziness sensitivity
    ignoreLocation: true,
    minMatchCharLength: 2,
  });
}

// --- Load all clips initially ---
export async function loadClips(limit = 200) {
  const loaded = await safeInvoke("get_recent_clips", { limit });
  allClips = loaded;
  rebuildFuseIndex();

  const pinned = loaded.filter(c => c.is_pinned);
  const recent = loaded.filter(c => !c.is_pinned);
  pinnedClips.set(pinned);
  clips.set(recent);
}

// --- Fuzzy Search (client-side) ---
export function searchClips(query) {
  searchQuery.set(query);

  if (!query.trim()) {
    const pinned = allClips.filter(c => c.is_pinned);
    const recent = allClips.filter(c => !c.is_pinned);
    pinnedClips.set(pinned);
    clips.set(recent);
    return;
  }

  if (!fuse) rebuildFuseIndex();

  const results = fuse.search(query).map(r => r.item);
  if (results.length === 0) {
    // show everything instead of empty
    const pinned = allClips.filter(c => c.is_pinned);
    const recent = allClips.filter(c => !c.is_pinned);
    pinnedClips.set(pinned);
    clips.set(recent);
    return;
  }

  const pinned = results.filter(c => c.is_pinned);
  const recent = results.filter(c => !c.is_pinned);

  pinnedClips.set(pinned);
  clips.set(recent);
}

// --- Pin/unpin a clip ---
export async function togglePin(id, isPinned) {
  await safeInvoke("pin_clip", { id, isPinned: isPinned });

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
    if (movedClip) {
      movedClip.is_pinned = true;
      pinnedClips.update(prev => [movedClip, ...prev]);
    }
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
    if (movedClip) {
      movedClip.is_pinned = false;
      clips.update(prev => [movedClip, ...prev]);
    }
  }

  // Reflect change in cache + Fuse
  const idx = allClips.findIndex(c => c.id === id);
  if (idx >= 0) allClips[idx].is_pinned = isPinned;
  rebuildFuseIndex();
}

// --- Delete a clip ---
export async function deleteClip(id) {
  await safeInvoke("delete_clip", { id });
  clips.update(prev => prev.filter(c => c.id !== id));
  pinnedClips.update(prev => prev.filter(c => c.id !== id));
  allClips = allClips.filter(c => c.id !== id);
  rebuildFuseIndex();
}

// --- Clear all clips ---
export async function clearAllClips() {
  await safeInvoke("clear_history");
  allClips = [];
  fuse = null;
  clips.set([]);
  pinnedClips.set([]);
}

// --- Capture current clipboard manually ---
export async function captureCurrentClip() {
  try {
    isLoading.set(true);
    error.set(null);

    const newClip = await safeInvoke("capture_current_clip");
    allClips = [newClip, ...allClips.filter(c => c.id !== newClip.id)];
    rebuildFuseIndex();
    return newClip;
  } catch (err) {
    console.error("Tauri invoke error: capture_current_clip", err);
    error.set(err.message || "Failed to capture clipboard");
  } finally {
    isLoading.set(false);
  }
}