<script>
  import { onMount, onDestroy, tick } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { listen } from "@tauri-apps/api/event";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { theme } from "$lib/stores/theme"; // ensure theme sync

  // --- Reactive State ---
  let query = $state("");
  let allClips = $state([]);
  let filteredClips = $state([]);
  let selectedIndex = $state(0);
  let copiedMessage = $state("");
  let appWindow = getCurrentWebviewWindow();
  let clipAddedUnlisten = null;
  let inputEl;
  let listEl = $state(null);
  let searchTimeout = null;

  // --- Lightweight fuzzy search (memory-safe) ---
  function fuzzySearch(list, term) {
    if (!term || !term.trim()) return list;
    const q = term.trim().toLowerCase();
    const results = [];
    for (const c of list) {
      const text =
        `${c.content} ${c.app_name} ${c.window_title} ${c.auto_tags} ${c.manual_tags}`.toLowerCase();
      if (text.includes(q)) results.push(c);
    }
    return results;
  }

  // --- Load and Filter ---
  async function loadClips() {
    try {
      const all = await invoke("get_recent_clips", { limit: 50 });
      allClips = Array.isArray(all) ? all : [];
      filterClips();
    } catch (err) {
      console.error("Failed to load clips:", err);
      allClips = [];
      filteredClips = [];
    }
  }

  function filterClips() {
    filteredClips = fuzzySearch(allClips, query);
    selectedIndex = 0;
  }

  // --- Input with debounce to reduce GC churn ---
  function handleInput() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(filterClips, 80);
  }

  // --- Clipboard copy ---
  async function pasteClip(clip) {
    if (!clip) return;
    try {
      await invoke("ignore_next_clipboard_update", { content: clip.content });
      await writeText(clip.content);
      copiedMessage = "Copied!";
      setTimeout(() => (copiedMessage = ""), 500);
    } catch (err) {
      console.error("Failed to write clipboard:", err);
      copiedMessage = "Failed";
      setTimeout(() => (copiedMessage = ""), 600);
    }

    // Hide picker
    try {
      await appWindow.hide();
    } catch (err) {
      console.warn("Quick Picker hide failed:", err);
    }
  }
  // --- Navigation ---
  function navigate(direction) {
    if (!filteredClips.length) return;
    selectedIndex =
      (selectedIndex + direction + filteredClips.length) % filteredClips.length;
    tick().then(() => {
      listEl?.querySelector(".clip-item.selected")?.scrollIntoView({
        block: "nearest",
      });
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
      appWindow.hide().catch(() => {});
    }
  }

  // --- Live updates from backend ---
  function handleClipAdded(event) {
    const newClip = event.payload;
    if (!newClip?.content) return;
    if (allClips[0]?.content === newClip.content) return;
    allClips.unshift(newClip);
    if (allClips.length > 200) allClips.pop(); // limit list size
    filterClips();
  }

  // --- Lifecycle ---
  onMount(async () => {
    await loadClips();
    try {
      clipAddedUnlisten = await listen("clip-added", handleClipAdded);
      await listen("clip-deleted", loadClips);
      await listen("clip-updated", loadClips);
      await listen("history-cleared", loadClips);
    } catch (err) {
      console.warn("Event subscription failed:", err);
    }

    window.addEventListener("keydown", handleKeyDown);
    await tick();
    inputEl?.focus();
  });

  onDestroy(() => {
    clearTimeout(searchTimeout);
    window.removeEventListener("keydown", handleKeyDown);
    clipAddedUnlisten?.();
  });

  // --- Derived lists ---
  let pinnedClips = $derived(filteredClips.filter((c) => c.is_pinned));
  let recentClips = $derived(filteredClips.filter((c) => !c.is_pinned));
</script>

<!-- unchanged HTML layout -->
<div class="quick-picker">
  <div class="search-container">
    <svg class="search-icon" viewBox="0 0 24 24" width="14" height="14">
      <path
        fill="currentColor"
        d="M15.5 14h-.79l-.28-.27C15.41 12.59 
           16 11.11 16 9.5 16 5.91 13.09 3 
           9.5 3S3 5.91 3 9.5 
           5.91 16 9.5 16c1.61 0 3.09-.59 
           4.23-1.57l.27.28v.79l5 4.99L20.49 
           19l-4.99-5zM9.5 14C7.01 14 5 11.99 
           5 9.5S7.01 5 9.5 5 14 7.01 
           14 9.5 11.99 14 9.5 14z"
      />
    </svg>
    <input
      bind:this={inputEl}
      bind:value={query}
      oninput={handleInput}
      placeholder="Search clips..."
      class="search-input"
      autocomplete="off"
    />
  </div>

  {#if copiedMessage}
    <div class="copied-message">{copiedMessage}</div>
  {/if}

  {#if !filteredClips.length}
    <div class="no-results">
      <img
        class="no-results-icon"
        src="src/assests/Square71x71Logo.png"
        alt="logo"
      />
      <div class="no-results-text">No clips found</div>
    </div>
  {:else}
    <ul class="clip-list" bind:this={listEl}>
      {#if pinnedClips.length}
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

      {#if recentClips.length}
        <li class="section-header">
          <span class="section-title">Recent</span>
          <span class="section-count">({recentClips.length})</span>
        </li>
        {#each recentClips as clip, i (clip.id)}
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
    height: 96vh;
    background: var(--bg-primary);
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
    border: 1px solid var(--border-color);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
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
    box-shadow: 0 0 0 3px
      color-mix(in srgb, var(--action-primary), transparent 90%);
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
    margin-top: 100px;
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
