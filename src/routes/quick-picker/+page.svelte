<script>
  import { onMount, onDestroy, tick } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { listen } from '@tauri-apps/api/event';
  import Fuse from 'fuse.js';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';

  let query = '';
  let clips = [];
  let filtered = [];
  let selectedIndex = 0;
  let fuse = null;
  let copiedMessage = '';
  let appWindow = getCurrentWebviewWindow(); // Tauri v2 API
  let clipAddedUnlisten = null;
  let inputEl;
  let listEl;

  // --- Helpers ------------------------------------------------

  function buildFuse(list) {
    fuse = new Fuse(list, {
      keys: ['content', 'app_name', 'window_title'],
      threshold: 0.3,
      includeScore: true,
    });
  }

  // async function safeFocus() {
  //   try {
  //     // show + focus may fail if window isn't ready; attempt and log
  //     await appWindow.show();
  //     await appWindow.setFocus();
  //   } catch (err) {
  //     // If setFocus fails, log to console — it's non-fatal.
  //     console.warn('Window focus failed:', err);
  //   }
  // }

  // Load recent clips from Rust backend
  async function loadClips() {
    try {
      const all = await invoke('get_recent_clips', { limit: 50 });
      // Ensure we have an array
      clips = Array.isArray(all) ? all : [];
      buildFuse(clips);
      // no query means first 10
      filterClips();
    } catch (err) {
      console.error('Failed to load clips:', err);
      clips = [];
      buildFuse(clips);
      filterClips();
    }
  }

  // Filter clips based on query (kept small & reactive)
  function filterClips() {
    if (!query || !query.trim()) {
      filtered = clips.slice(0, 10);
    } else if (fuse) {
      const results = fuse.search(query);
      filtered = results.map((r) => r.item).slice(0, 10);
    } else {
      filtered = clips.slice(0, 10);
    }
    selectedIndex = 0;
  }

  // keep results reactive if clips or query changes
  $: if (clips) {
    // rebuild fuse when clips changes
    buildFuse(clips);
    filterClips();
  }

  // Paste selected clip (cross-platform, no external tools)
  async function pasteClip(clip) {
    if (!clip) return;
    try {
      await invoke("ignore_next_clipboard_update");
      await writeText(clip.content);
      copiedMessage = 'Copied! Press Ctrl+V (Cmd+V on macOS) to paste.';
      setTimeout(() => (copiedMessage = ''), 2000);
    } catch (err) {
      console.error('Failed to write to clipboard:', err);
      copiedMessage = 'Failed to copy to clipboard';
      setTimeout(() => (copiedMessage = ''), 2000);
      return;
    }

    // Hide the quick picker window (defensive)
    try {
      await appWindow.hide();
    } catch (err) {
      console.warn('Failed to hide quick-picker window:', err);
    }
  }

  // Navigate through filtered clips
  function navigate(direction) {
    if (!filtered || filtered.length === 0) return;
    selectedIndex = (selectedIndex + direction + filtered.length) % filtered.length;
    // scroll item into view
    tick().then(() => {
      const sel = listEl?.querySelector('.clip-item.selected');
      if (sel && typeof sel.scrollIntoView === 'function') {
        sel.scrollIntoView({ block: 'nearest' });
      }
    });
  }

  function handleKeyDown(e) {
    if (e.key === 'ArrowUp') {
      e.preventDefault();
      navigate(-1);
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      navigate(1);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filtered[selectedIndex]) {
        pasteClip(filtered[selectedIndex]);
      }
    } else if (e.key === 'Escape') {
      e.preventDefault();
      appWindow.hide().catch((err) => console.warn('hide failed', err));
    }
  }

  // When clip-added arrives from Rust, add to the top of clips (dedupe optional)
  function handleClipAdded(event) {
    // event.payload should be the saved Clip
    const newClip = event.payload;
    if (!newClip || !newClip.content) return;

    // avoid exact duplicate at top
    if (clips.length > 0 && clips[0].content === newClip.content) {
      return;
    }
    clips = [newClip, ...clips].slice(0, 200); // keep a max cap in memory
    // fuse rebuilt by reactive statement above
    filterClips();
  }

  // Lifecycle
  onMount(async () => {
    // initial load
    await loadClips();

    // listen to events from Rust/tauri
    try {
      clipAddedUnlisten = await listen('clip-added', (e) => {
        handleClipAdded(e);
      });
    } catch (err) {
      console.warn('Failed to subscribe to clip-added event:', err);
    }

    // keyboard navigation
    window.addEventListener('keydown', handleKeyDown);

    // Focus the input once the window is visible
    // If this quick-picker was opened via shortcut, ensure focus and visibility:
    // await safeFocus();

    // small delay to ensure DOM is ready
    await tick();
    if (inputEl) inputEl.focus();
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeyDown);
    if (clipAddedUnlisten) {
      // Tauri v2 returns an unlisten function directly
      if (typeof clipAddedUnlisten === 'function') {
        clipAddedUnlisten();
      }
    }
  });
</script>

<div class="quick-picker">
  <input
    bind:this={inputEl}
    bind:value={query}
    on:input={filterClips}
    placeholder="Type to search..."
    class="search-input"
    autocomplete="off"
  />
  {#if copiedMessage}
    <div class="copied-message">{copiedMessage}</div>
  {/if}
  {#if filtered.length === 0}
    <div class="no-results">No clips found</div>
  {:else}
    <ul class="clip-list" bind:this={listEl}>
      {#each filtered as clip, i}
        <li
          class="clip-item {i === selectedIndex ? 'selected' : ''}"
          on:click={() => pasteClip(clip)}
        >
          <div class="content" title={clip.content}>
            {clip.content.length > 60 ? clip.content.substring(0, 60) + '…' : clip.content}
          </div>
          <div class="meta">[{clip.app_name || 'unknown'}]</div>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .quick-picker {
    padding: 8px;
    min-width: 300px;
    max-width: 600px;
  }
  .search-input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 0.95rem;
    outline: none;
  }
  .search-input:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59,130,246,0.08);
  }
  .copied-message {
    padding: 4px 0;
    color: #10b981;
    font-size: 0.85rem;
    text-align: center;
  }
  .clip-list {
    list-style: none;
    padding: 6px 0;
    margin: 8px 0 0 0;
    max-height: 300px;
    overflow-y: auto;
  }
  .clip-item {
    display: flex;
    justify-content: space-between;
    padding: 8px 10px;
    border-radius: 6px;
    cursor: pointer;
    gap: 12px;
    align-items: center;
  }
  .clip-item:hover, .clip-item.selected {
    background: #eef2ff;
  }
  .content {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 0.9rem;
  }
  .meta {
    color: #6b7280;
    font-size: 0.75rem;
    margin-left: 8px;
    flex-shrink: 0;
  }
  .no-results {
    padding: 12px;
    color: #9ca3af;
    font-size: 0.85rem;
    text-align: center;
  }
</style>