<script>
  import { onMount, onDestroy, tick } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { listen } from "@tauri-apps/api/event";
  import Fuse from "fuse.js";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  // Import theme store to ensure theme is applied
  import { theme } from "$lib/stores/theme";

  let query = $state("");
  let allClips = $state([]);
  let filteredClips = $state([]);
  let selectedIndex = $state(0);
  let fuse = $state(null);
  let copiedMessage = $state("");
  let appWindow = getCurrentWebviewWindow();
  let clipAddedUnlisten = null;
  let inputEl;
  let listEl = $state(null);

  function buildFuse(list) {
    fuse = new Fuse(list, {
      keys: ["content", "app_name", "window_title", "auto_tags", "manual_tags"],
      threshold: 0.3,
      includeScore: true,
    });
  }

  async function loadClips() {
    try {
      const all = await invoke("get_recent_clips", { limit: 50 });
      allClips = Array.isArray(all) ? all : [];
      buildFuse(allClips);
      filterClips();
    } catch (err) {
      console.error("Failed to load clips:", err);
      allClips = [];
      buildFuse(allClips);
      filterClips();
    }
  }

  function filterClips() {
    if (!query || !query.trim()) {
      filteredClips = [...allClips];
    } else if (fuse) {
      const results = fuse.search(query);
      filteredClips = results.map((r) => r.item);
    } else {
      filteredClips = [];
    }
    selectedIndex = 0;
  }

  async function pasteClip(clip) {
    if (!clip) return;
    try {
      await invoke("ignore_next_clipboard_update");
      await writeText(clip.content);
      copiedMessage = "Copied!";
      setTimeout(() => (copiedMessage = ""), 500);
    } catch (err) {
      console.error("Failed to write to clipboard:", err);
      copiedMessage = "Failed to copy";
      setTimeout(() => (copiedMessage = ""), 500);
    }

    try {
      await appWindow.hide();
    } catch (err) {
      console.warn("Failed to hide quick-picker window:", err);
    }
  }

  function navigate(direction) {
    if (!filteredClips || filteredClips.length === 0) return;
    selectedIndex =
      (selectedIndex + direction + filteredClips.length) % filteredClips.length;
    tick().then(() => {
      const sel = listEl?.querySelector(".clip-item.selected");
      sel?.scrollIntoView({ block: "nearest" });
    });
  }

  function handleKeyDown(e) {
    if (e.key === "ArrowUp") {
      e.preventDefault();
      navigate(-1);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      navigate(1);
    } else if (e.key === "Enter") {
      e.preventDefault();
      pasteClip(filteredClips[selectedIndex]);
    } else if (e.key === "Escape") {
      e.preventDefault();
      appWindow.hide().catch((err) => console.warn("hide failed", err));
    }
  }

  function handleClipAdded(event) {
    const newClip = event.payload;
    if (!newClip?.content) return;
    if (allClips.length > 0 && allClips[0].content === newClip.content) return;
    allClips = [newClip, ...allClips.slice(0, 199)];
    buildFuse(allClips);
    filterClips();
  }

  onMount(async () => {
    await loadClips();
    try {
      clipAddedUnlisten = await listen("clip-added", handleClipAdded);
      await listen("clip-deleted", loadClips);
      await listen("clip-updated", loadClips);
      await listen("history-cleared", loadClips);
    } catch (err) {
      console.warn("Failed to subscribe to clip events:", err);
    }
    window.addEventListener("keydown", handleKeyDown);
    await tick();
    inputEl?.focus();
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeyDown);
    clipAddedUnlisten?.();
  });

  // Optimized getters using Svelte 5 runes
  let pinnedClips = $derived(filteredClips.filter((c) => c.is_pinned));
  let recentClips = $derived(filteredClips.filter((c) => !c.is_pinned));
</script>

<div class="quick-picker">
  <div class="search-container">
    <svg class="search-icon" viewBox="0 0 24 24" width="14" height="14">
      <path
        fill="currentColor"
        d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 
           13.09 3 9.5 3S3 5.91 3 9.5 
           5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79
           l5 4.99L20.49 19l-4.99-5zm-6 0
           C7.01 14 5 11.99 5 9.5S7.01 5
           9.5 5 14 7.01 14 9.5 11.99
           14 9.5 14z"
      />
    </svg>
    <input
      bind:this={inputEl}
      bind:value={query}
      oninput={filterClips}
      placeholder="Search clips..."
      class="search-input"
      autocomplete="off"
    />
  </div>

  {#if copiedMessage}
    <div class="copied-message">{copiedMessage}</div>
  {/if}

  {#if filteredClips.length === 0}
    <div class="no-results">
      <div class="no-results-icon">📋</div>
      <div class="no-results-text">No clips found</div>
    </div>
  {:else}
    <ul class="clip-list" bind:this={listEl}>
      {#if pinnedClips.length > 0}
        <li class="section-header">
          <span class="section-title">Pinned</span>
          <span class="section-count">({pinnedClips.length})</span>
        </li>
        {#each pinnedClips as clip, i}
          <li
            class="clip-item {i === selectedIndex ? 'selected' : ''}"
            onclick={() => pasteClip(clip)}
          >
            <div class="clip-content">
              <div class="content" title={clip.content}>
                {clip.content.length > 80
                  ? clip.content.substring(0, 80) + "…"
                  : clip.content}
              </div>
              {#if clip.window_title}
                <div class="app-info">
                  <span class="window-title">{clip.window_title}</span>
                </div>
              {/if}
            </div>
          </li>
        {/each}
      {/if}

      {#if recentClips.length > 0}
        <li class="section-header">
          <span class="section-title">Recent</span>
          <span class="section-count">({recentClips.length})</span>
        </li>
        {#each recentClips as clip, i}
          {@const index = pinnedClips.length + i}
          <li
            class="clip-item {index === selectedIndex ? 'selected' : ''}"
            onclick={() => pasteClip(clip)}
          >
            <div class="clip-content">
              <div class="content" title={clip.content}>
                {clip.content.length > 80
                  ? clip.content.substring(0, 80) + "…"
                  : clip.content}
              </div>
              {#if clip.window_title}
                <div class="app-info">
                  <span class="window-title">{clip.window_title}</span>
                </div>
              {/if}
            </div>
          </li>
        {/each}
      {/if}
    </ul>
  {/if}
</div>

<style>
  .quick-picker {
    width: 100%;
    background: var(--bg-primary);
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    border: 1px solid var(--border-color);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    overflow: hidden;
  }

  .search-container {
    position: relative;
    padding: 12px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .search-icon {
    position: absolute;
    left: 28px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text-muted);
    z-index: 1;
  }

  .search-input {
    width: 85%;
    padding: 8px 12px 8px 36px;
    font-size: 0.9rem;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    outline: none;
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .search-input:focus {
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--action-primary), transparent 90%);
  }

  .copied-message {
    padding: 8px 12px;
    font-size: 0.8rem;
    color: var(--success);
    background: var(--bg-accent);
    border-bottom: 1px solid var(--border-color-light);
    text-align: center;
  }

  .clip-list {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 450px;
    overflow-y: auto;
  }

  .section-header {
    padding: 8px 12px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .section-title {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .section-count {
    font-size: 0.7rem;
    color: var(--text-muted);
    background: var(--border-color-light);
    padding: 1px 6px;
    border-radius: 12px;
  }

  .clip-item {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    background: var(--bg-primary);
    padding: 10px 12px;
    border-bottom: 1px solid var(--bg-tertiary);
    cursor: pointer;
    transition: none;
  }

  .clip-item:last-child {
    border-bottom: none;
  }

  .clip-item:hover {
    background: var(--bg-secondary);
  }

  .clip-item.selected {
    background: var(--bg-accent);
    border-left: 3px solid var(--action-primary);
  }

  .clip-content {
    flex: 1;
    min-width: 0;
  }

  .content {
    font-size: 0.85rem;
    color: var(--text-primary);
    line-height: 1.4;
    word-break: break-word;
    white-space: pre-wrap;
    margin-bottom: 4px;
  }

  .app-info {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-wrap: wrap;
    font-size: 0.7rem;
    color: var(--text-secondary);
  }

  .window-title {
    color: var(--action-primary);
    font-weight: 600;
  }

  .no-results {
    padding: 32px 12px;
    text-align: center;
    color: var(--text-secondary);
  }

  .no-results-icon {
    font-size: 2rem;
    margin-bottom: 8px;
    opacity: 0.6;
  }

  .no-results-text {
    font-size: 0.9rem;
    color: var(--text-muted);
  }

  /* Scrollbar styling */
  .clip-list::-webkit-scrollbar {
    width: 6px;
  }

  .clip-list::-webkit-scrollbar-track {
    background: var(--bg-tertiary);
  }

  .clip-list::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: 3px;
  }

  .clip-list::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }
</style>