<script lang="ts">
  import { onMount, onDestroy, tick } from "svelte";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { theme, type Theme } from "$lib/services/theme";

  import {
    clips,
    pinnedClips,
    searchTerm,
    loadClips,
    ignorePasting,
    initClipEvents,
  } from "$lib/services/clips";

  import type { Clip } from "$lib/stores/types";
  import { listen } from "@tauri-apps/api/event";

  // --- Window ---
  const appWindow = getCurrentWebviewWindow();

  // --- UI State ---
  let query = "";
  let selectedIndex = 0;
  let copiedMessage = "";
  let inputEl: HTMLInputElement | null = null;
  let listEl: HTMLUListElement | null = null;

  // --- Derived visible list (for keyboard navigation) ---
  $: visibleClips = [...$pinnedClips, ...$clips];

  // --- Sync search with store ---
  $: searchTerm.set(query);

  // --- Clipboard copy ---
  async function pasteClip(clip?: Clip) {
    if (!clip?.content) return;

    try {
      await ignorePasting(clip.content);
      await writeText(clip.content);
      copiedMessage = "Copied!";
      setTimeout(() => (copiedMessage = ""), 500);
    } catch (err) {
      console.error("Clipboard write failed:", err);
      copiedMessage = "Failed";
      setTimeout(() => (copiedMessage = ""), 600);
    }

    appWindow.hide().catch(() => {});
  }

  // --- Keyboard navigation ---
  function navigate(direction: number) {
    if (!visibleClips.length) return;

    selectedIndex =
      (selectedIndex + direction + visibleClips.length) % visibleClips.length;

    tick().then(() => {
      const el = listEl?.querySelector(".clip-item.selected");
      el?.scrollIntoView({ block: "nearest", behavior: "smooth" });
    });
  }

  function handleKeyDown(e: KeyboardEvent) {
    switch (e.key) {
      case "ArrowUp":
        e.preventDefault();
        navigate(-1);
        break;
      case "ArrowDown":
        e.preventDefault();
        navigate(1);
        break;
      case "Enter":
        e.preventDefault();
        pasteClip(visibleClips[selectedIndex]);
        break;
      case "Escape":
        e.preventDefault();
        appWindow.hide().catch(() => {});
        break;
    }
  }

  // --- Lifecycle ---
  onMount(async () => {
    await initClipEvents();
    await loadClips(50);
    const unlisten = await listen<Theme>("theme-change", (event: any) => {
      const newTheme = event.payload;
      theme.set(newTheme);
    });
    window.addEventListener("keydown", handleKeyDown);
    await tick();
    inputEl?.focus();

    onDestroy(() => {
      window.removeEventListener("keydown", handleKeyDown);
      unlisten();
    });
  });
</script>

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
      placeholder="Search clips..."
      class="search-input"
      autocomplete="off"
    />
  </div>

  {#if copiedMessage}
    <div class="copied-message">{copiedMessage}</div>
  {/if}

  {#if !visibleClips.length}
    <div class="no-results">
      <img class="no-results-icon" src="/Square71x71Logo.png" alt="logo" />
      <div class="no-results-text">No clips found</div>
    </div>
  {:else}
    <ul class="clip-list" bind:this={listEl}>
      {#if $pinnedClips.length}
        <li class="section-header">
          <span class="section-title">Pinned</span>
          <span class="section-count">({$pinnedClips.length})</span>
        </li>
        {#each $pinnedClips as clip, i}
          <li class="clip-item {i === selectedIndex ? 'selected' : ''}">
            <button
              class="clip-button"
              on:click|preventDefault={() => pasteClip(clip)}
              aria-label="Copy clip: {clip.content.length > 50
                ? clip.content.slice(0, 50) + '…'
                : clip.content}"
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
            </button>
          </li>
        {/each}
      {/if}

      {#if $clips.length}
        <li class="section-header">
          <span class="section-title">Recent</span>
          <span class="section-count">({$clips.length})</span>
        </li>
        {#each $clips as clip, i (clip.id)}
          {@const index = $pinnedClips.length + i}
          <li class="clip-item {index === selectedIndex ? 'selected' : ''}">
            <button
              class="clip-button"
              on:click|preventDefault={() => pasteClip(clip)}
              aria-label="Copy clip: {clip.content.length > 50
                ? clip.content.slice(0, 50) + '…'
                : clip.content}"
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
            </button>
          </li>
        {/each}
      {/if}
    </ul>
  {/if}
</div>

<style>
  :global(html),
  :global(body) {
    height: 100%;
    margin: 0;
    padding: 0;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: var(--font-primary);
  }

  .quick-picker {
    width: 420px;
    height: 500px;
    background: var(--bg-primary);
    border-radius: var(--radius-lg);
    border: 1px solid var(--border-color);
    box-shadow: var(--shadow-md);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .search-container {
    position: sticky;
    top: 0;
    z-index: 10;
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
    pointer-events: none;
  }

  .search-input {
    width: 88%;
    padding: 8px 12px 8px 36px;
    font-size: var(--font-size-md);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    background: var(--bg-primary);
    color: var(--text-primary);
    outline: none;
  }

  .search-input:focus {
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px var(--focus-ring-color);
  }

  .copied-message {
    padding: 8px;
    font-size: var(--font-size-sm);
    color: var(--success);
    background: var(--bg-accent);
    text-align: center;
    border-bottom: 1px solid var(--border-color-light);
  }

  .clip-list {
    flex: 1;
    overflow-y: auto;
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .section-header {
    position: sticky;
    top: 0;
    z-index: 5;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .section-title {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .section-count {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    background: var(--border-color-light);
    padding: 2px 6px;
    border-radius: 12px;
  }

  .clip-item {
    padding: 10px 12px;
    border-bottom: 1px solid var(--bg-tertiary);
    cursor: pointer;
    display: flex;
  }

  .clip-item:hover {
    background: var(--bg-secondary);
  }

  .clip-item.selected {
    background: var(--bg-accent);
    border-left: 3px solid var(--action-primary);
  }

  .clip-button {
    all: unset;
    display: block;
    width: 100%;
    text-align: left;
    cursor: pointer;
    padding: 5px 6px;
    border-radius: var(--radius-sm);
    outline: none;
  }

  .clip-button:focus-visible {
    /* Use your existing focus ring */
    box-shadow: 0 0 0 3px var(--focus-ring-color);
  }
  .content {
    font-size: var(--font-size-sm);
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-word;
    line-height: 1.3;
  }

  .app-info {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    margin-top: 4px;
  }

  .window-title {
    font-weight: var(--font-weight-semibold);
    color: var(--action-primary);
  }

  .no-results {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    color: var(--text-muted);
    padding: 24px;
    text-align: center;
  }

  .no-results-icon {
    width: 48px;
    height: 48px;
    opacity: 0.6;
    margin-bottom: 16px;
  }

  .no-results-text {
    font-size: var(--font-size-md);
    color: var(--text-secondary);
  }

  .clip-list::-webkit-scrollbar {
    width: 6px;
  }

  .clip-list::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: 3px;
  }
</style>
