// src/lib/stores/clips.js
// @ts-nocheck
import { writable, derived } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export const clips = writable([]);
export const pinnedClips = writable([]);
export const searchQuery = writable("");
export const isLoading = writable(false);
export const error = writable(null);

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

export async function loadClips(limit = 200) {
  const allClips = await safeInvoke("get_recent_clips", { limit });
  console.log("loaded clips: ", allClips);
  const pinned = allClips.filter((c) => c.is_pinned);
  const recent = allClips.filter((c) => !c.is_pinned);
  pinnedClips.set(pinned);
  clips.set(recent);
}

export async function searchClips(query) {
  searchQuery.set(query);
  if (!query.trim()) return loadClips();

  const results = await safeInvoke("search_clips", { query, limit: 50 });
  const pinned = results.filter((c) => c.is_pinned);
  const recent = results.filter((c) => !c.is_pinned);
  pinnedClips.set(pinned);
  clips.set(recent);
}

export async function togglePin(id, shouldPin) {
  // Make sure the parameter names match exactly what the backend expects
  console.log('Sending to pin_clip:', { id, shouldPin });
  await safeInvoke("pin_clip", { 
    id: id, 
    isPinned: shouldPin
  });
  await loadClips();
}

export async function deleteClip(id) {
  await safeInvoke("delete_clip", { id });
  await loadClips();
}

export async function clearAllClips() {
  await safeInvoke("clear_history");
  await loadClips();
}
