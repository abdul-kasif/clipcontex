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
  } from "$lib/services/clip";

  import type { Clip } from "$lib/stores/types";
  import { listen } from "@tauri-apps/api/event";
  import { setDragging } from "$lib/services/system";

  const appWindow = getCurrentWebviewWindow();

  let query = "";
  let selectedIndex = 0;
  let copiedMessage = "";
  let inputEl: HTMLInputElement | null = null;
  let listEl: HTMLUListElement | null = null;

  $: visibleClips = [...$pinnedClips, ...$clips];
  $: searchTerm.set(query);

  async function pasteClip(clip?: Clip) {
    if (!clip?.content) return;

    try {
      await ignorePasting(clip.content);
      await writeText(clip.content);
      copiedMessage = "Copied!";
      setTimeout(() => (copiedMessage = ""), 500);
    } catch {
      copiedMessage = "Failed";
      setTimeout(() => (copiedMessage = ""), 600);
    }

    appWindow.hide().catch(() => {});
  }

  function navigate(direction: number) {
    if (!visibleClips.length) return;

    selectedIndex =
      (selectedIndex + direction + visibleClips.length) % visibleClips.length;

    tick().then(() => {
      listEl
        ?.querySelector(".clip-item.selected")
        ?.scrollIntoView({ block: "nearest" });
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

  let isDragging: boolean = false;

  async function startDragging() {
    if (isDragging) return;

    isDragging = true;
    setDragging(true);

    try {
      await appWindow.startDragging();
    } finally {
      isDragging = false;
      setDragging(false);
    }
  }

  let unlisten: () => void;

  onMount(async () => {
    await initClipEvents();
    await loadClips(50);

    unlisten = await listen<Theme>("theme-change", (e) => theme.set(e.payload));

    window.addEventListener("keydown", handleKeyDown);
    await tick();
    inputEl?.focus();
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeyDown);
    unlisten?.();
  });
</script>

<div class="quick-picker">
  <!-- Header -->
  <div class="search-container">
    <div class="search-input-wrapper">
      <svg class="search-icon" viewBox="0 0 24 24">
        <path
          fill="currentColor"
          d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5
             16 5.91 13.09 3 9.5 3S3 5.91 3 9.5
             5.91 16 9.5 16c1.61 0 3.09-.59
             4.23-1.57l.27.28v.79l5 4.99L20.49
             19l-4.99-5z"
        />
      </svg>

      <input
        bind:this={inputEl}
        bind:value={query}
        placeholder="Search clips…"
        class="search-input"
        autocomplete="off"
        spellcheck="false"
      />
    </div>

    <button
      class="drag-handle"
      aria-label="Drag window"
      on:mousedown|preventDefault={startDragging}
    >
      ⠿
    </button>
  </div>

  {#if copiedMessage}
    <div class="copied-message">{copiedMessage}</div>
  {/if}

  {#if !visibleClips.length}
    <div class="no-results">
      <img src="/Square71x71Logo.png" class="no-results-icon" alt="" />
      <div>No clips found</div>
    </div>
  {:else}
    <ul class="clip-list" bind:this={listEl}>
      {#if $pinnedClips.length}
        <li class="section-header">
          <span class="section-title">Pinned</span>
          <span class="section-count">{$pinnedClips.length}</span>
        </li>

        {#each $pinnedClips as clip, i}
          <li class="clip-item {i === selectedIndex ? 'selected' : ''}">
            <button class="clip-button" on:click={() => pasteClip(clip)}>
              <div class="content">
                {clip.content.length > 80
                  ? clip.content.slice(0, 80) + "…"
                  : clip.content}
              </div>

              {#if clip.window_title}
                <div class="app-info">
                  <span class="window-title">{clip.window_title}</span>
                </div>
              {/if}
            </button>
          </li>
        {/each}
      {/if}

      {#if $clips.length}
        <li class="section-header">
          <span class="section-title">Recent</span>
          <span class="section-count">{$clips.length}</span>
        </li>

        {#each $clips as clip, i}
          {@const index = $pinnedClips.length + i}
          <li class="clip-item {index === selectedIndex ? 'selected' : ''}">
            <button class="clip-button" on:click={() => pasteClip(clip)}>
              <div class="content">
                {clip.content.length > 80
                  ? clip.content.slice(0, 80) + "…"
                  : clip.content}
              </div>

              {#if clip.window_title}
                <div class="app-info">
                  <span class="window-title">{clip.window_title}</span>
                </div>
              {/if}
            </button>
          </li>
        {/each}
      {/if}
    </ul>
  {/if}
</div>

<style>
  /* ===========================
     Global Normalization
  ============================ */

  :global(html),
  :global(body) {
    width: 100%;
    height: 100%;
    margin: 0;
    overflow: hidden;

    font-family: var(--font-primary);
    line-height: 1.4;
    text-size-adjust: 100%;
    -webkit-text-size-adjust: 100%;
  }

  /* ===========================
     Design Tokens
  ============================ */

  .quick-picker {
    --header-h: 50px;
    --input-h: 36px;
    --pad-x: 12px;
    --pad-y: 6px;
    --font-md: 0.85rem;
    --font-sm: 0.75rem;
    --font-lg: 0.95rem;
  }

  /* ===========================
     Window Container
  ============================ */

  .quick-picker {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;

    background: var(--bg-primary);
    color: var(--text-primary);

    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);

    box-sizing: border-box;
  }

  /* ===========================
     Header / Search
  ============================ */

  .search-container {
    height: var(--header-h);
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 var(--pad-x);

    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .search-input-wrapper {
    flex: 1;
    height: var(--input-h);

    display: grid;
    grid-template-columns: 16px 1fr;
    align-items: center;
    column-gap: 8px;

    padding: 0 10px;

    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
  }

  .search-input-wrapper:focus-within {
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px var(--focus-ring-color);
  }

  .search-icon {
    width: 16px;
    height: 16px;
    color: var(--text-muted);
  }

  .search-input {
    border: none;
    outline: none;
    background: transparent;

    font-size: var(--font-lg);
    font-family: inherit;
    color: inherit;
    line-height: 1.4;
  }

  .drag-handle {
    background: none;
    border: none;
    padding: 6px;

    font: inherit;
    cursor: grab;
    color: var(--text-muted);
  }

  /* ===========================
     Feedback
  ============================ */

  .copied-message {
    padding: 6px var(--pad-x);
    text-align: center;
    font-size: var(--font-md);

    color: var(--success);
    background: var(--bg-accent);
    border-bottom: 1px solid var(--border-color-light);
  }

  /* ===========================
     Clip List
  ============================ */

  .clip-list {
    flex: 1;
    overflow-y: auto;
    margin: 0;
    padding: 0;
    list-style: none;

    scrollbar-gutter: stable;
    scrollbar-width: thin;
    scrollbar-color: var(--border-color) transparent;
  }

  .section-header {
    position: sticky;
    top: 0;
    z-index: 2;

    display: flex;
    justify-content: space-between;
    padding: 6px var(--pad-x);

    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .section-title {
    font-size: var(--font-md);
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    color: var(--text-secondary);
  }

  .section-count {
    font-size: var(--font-md);
    color: var(--text-muted);
  }

  .clip-item {
    padding: var(--pad-y) var(--pad-x);
    border-bottom: 2px solid var(--border-color);
    border-left: 3px solid transparent;
  }

  .clip-item.selected {
    background: var(--bg-accent);
    border-left-color: var(--action-primary);
  }

  .clip-button {
    width: 100%;
    background: none;
    border: none;
    padding: 0;
    text-align: left;
    cursor: pointer;
    font: inherit;
    color: inherit;
  }

  .content {
    font-size: var(--font-md);
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .app-info {
    margin-top: 4px;
    font-size: var(--font-sm);
    color: var(--text-secondary);
  }

  .window-title {
    font-weight: var(--font-weight-semibold);
    color: var(--action-primary);
  }

  /* ===========================
     Empty State
  ============================ */

  .no-results {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    color: var(--text-muted);
  }

  .no-results-icon {
    width: 48px;
    height: 48px;
    opacity: 0.6;
  }
</style>
