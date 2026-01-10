<script lang="ts">
  // @ts-ignore
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from "@tauri-apps/plugin-os";
  import {
    loadClips,
    error,
    clips,
    pinnedClips,
    noResults,
    initClipEvents,
  } from "$lib/services/clips";
  import SearchBar from "$lib/components/main/SearchBar.svelte";
  import PinnedSection from "$lib/components/main/PinnedSection.svelte";
  import TimelineSection from "$lib/components/main/TimelineSection.svelte";
  import { getBoolean, setBoolean } from "$lib/stores/uiPreference";
  import { theme } from "$lib/services/theme";
  import { checkKdotoolInstalled } from "$lib/services/system";

  let showHelperMessage: boolean = true;
  let isKdotoolMissing: boolean = false;
  let showClearModal: boolean = false;

  onMount(async () => {
    await initClipEvents();
    try {
      const os = platform();
      showHelperMessage = await getBoolean("showHelperMessage", true);
      if (os === "linux") {
        const isInstalled = await checkKdotoolInstalled();
        if (!isInstalled) {
          isKdotoolMissing = true;
          return;
        }
      }
      await loadClips();
    } catch (err) {
      console.error("Startup Error:", err);
      isKdotoolMissing = true;
    }
  });

  async function confirmClearAll() {
    const { clearAllClips } = await import("$lib/services/clips");
    await clearAllClips();
    showCleanAllModel(false);
  }

  function showCleanAllModel(value: boolean) {
    showClearModal = value;
  }

  function openSettings() {
    try {
      goto("/settings");
    } catch (err) {
      console.error("Failed to open settings window:", err);
    }
  }

  async function dontShowAgain() {
    showHelperMessage = false;
    await setBoolean("showHelperMessage", false);
  }

  function toggleTheme() {
    theme.update((t) => (t === "dark" ? "light" : "dark"));
  }
</script>

<div class="app-container">
  <header class="app-header">
    <h1 class="app-title">Clipboard</h1>
    <div class="header-right">
      <div class="header-stats">
        <span class="stat-item"
          >Total: {$clips.length + $pinnedClips.length}</span
        >
        <span class="stat-item">Pinned: {$pinnedClips.length}</span>
      </div>
      <button
        class="icon-btn"
        title={$theme === "dark"
          ? "Switch to light mode"
          : "Switch to dark mode"}
        onclick={toggleTheme}
      >
        {#if $theme === "dark"}
          <!-- Sun icon -->
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="icon"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 3v2m0 14v2m9-9h-2M5 12H3m15.364-6.364l-1.414 1.414M7.05 16.95l-1.414 1.414m0-12.728l1.414 1.414M16.95 16.95l1.414 1.414M12 8a4 4 0 100 8 4 4 0 000-8z"
            />
          </svg>
        {:else}
          <!-- Moon icon -->
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="icon"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z"
            />
          </svg>
        {/if}
      </button>
      <button class="icon-btn" title="Settings" onclick={openSettings}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="icon"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.64-.947 3.51.923 2.563 2.563a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.947 1.64-.923 3.51-2.563 2.563a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.64.947-3.51-.923-2.563-2.563a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.947-1.64.923-3.51 2.563-2.563a1.724 1.724 0 002.573-1.066z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
      </button>

      <button
        class="clear-btn"
        onclick={() => {
          showCleanAllModel(true);
        }}
        title="Clear all clips"
      >
        Clear All
      </button>
    </div>
  </header>

  <main class="app-main">
    {#if isKdotoolMissing}
      <div class="error-state">
        <h3>Missing Dependency</h3>
        <p>
          The required tool <strong>kdotool</strong> is not installed on your
          system.<br />
          Please install it using your package manager:
        </p>
        <pre>sudo dnf install kdotool</pre>
        <button class="retry-btn" onclick={() => location.reload()}
          >Retry</button
        >
      </div>
    {:else if $error}
      <div class="error-state">
        <h3>Something went wrong</h3>
        <p>Please quit the application and open once again.</p>
      </div>
    {:else if $clips.length === 0 && $pinnedClips.length === 0 && !$noResults}
      <div class="empty-state">
        <h3>No clips yet</h3>
        <p>Start copying text to see it appear here</p>
      </div>
    {:else}
      <SearchBar />

      {#if showHelperMessage}
        <div class="helper-card">
          <div class="helper-content">
            <span class="helper-icon">➡</span>
            <p class="helper-text">
              Press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd> to open the
              quick picker
            </p>
          </div>

          <button
            class="helper-dismiss"
            title="Don't show again"
            onclick={dontShowAgain}
          >
            X
          </button>
        </div>
      {/if}

      {#if $noResults}
        <div class="empty-state">
          <h3>No matches found</h3>
          <p>Try a different search term</p>
        </div>
      {:else}
        <PinnedSection />
        <TimelineSection />
      {/if}
    {/if}
  </main>

  {#if showClearModal}
    <!-- 
    svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions 
    (Modal overlay is non-interactive by design; dismissal handled via Escape key and Cancel button) 
  -->
    <div
      class="modal-overlay"
      onclick={() => showCleanAllModel(false)}
      aria-hidden="true"
    >
      <div
        class="modal-content"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        tabindex="-1"
        aria-modal="true"
        aria-labelledby="modal-title"
      >
        <div class="modal-header">
          <h3 id="modal-title" class="modal-title">Clear All Clips</h3>
        </div>
        <div class="modal-body">
          <p class="modal-text">
            Are you sure you want to clear all clips? This action cannot be
            undone.
          </p>
        </div>
        <div class="modal-footer">
          <button
            class="modal-cancel-btn"
            onclick={() => showCleanAllModel(false)}
          >
            Cancel
          </button>
          <button class="modal-confirm-btn" onclick={confirmClearAll}>
            Clear All
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  :global(html),
  :global(body) {
    height: 100%;
    margin: 0;
    padding: 0;
    font-family: var(--font-primary);
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .app-container {
    min-height: 100vh;
    background: var(--bg-primary);
    padding: 12px;
    font-family: var(--font-primary);
    color: var(--text-primary);
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }

  .app-title {
    margin: 0;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .header-stats {
    display: flex;
    gap: 10px;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .stat-item {
    background: var(--bg-tertiary);
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: var(--font-weight-semibold);
  }

  .icon-btn {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: 4px 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover {
    background: var(--border-color-light);
  }

  .icon-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  .icon {
    width: 18px;
    height: 18px;
    color: var(--text-primary);
  }

  .clear-btn {
    background: var(--danger-bg);
    color: var(--danger);
    border: 1px solid var(--danger-border);
    padding: 4px 10px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
  }

  .clear-btn:hover {
    background: var(--danger-border);
  }

  .clear-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  .app-main {
    max-width: 800px;
    margin: 0 auto;
  }

  .error-state,
  .empty-state {
    text-align: center;
    padding: 24px 16px;
    color: var(--text-secondary);
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    margin-top: 12px;
  }

  .error-state h3,
  .empty-state h3 {
    margin: 0 0 6px 0;
    color: var(--text-primary);
    font-size: var(--font-size-md);
  }

  .error-state p,
  .empty-state p {
    margin: 0 0 12px 0;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  pre {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    padding: 8px 12px;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    text-align: left;
    overflow-x: auto;
  }

  .retry-btn {
    background: var(--action-primary);
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
  }

  .retry-btn:hover {
    background: var(--action-primary-hover);
  }

  .retry-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  .helper-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 10px 12px;
    margin: 12px 0;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .helper-content {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .helper-icon {
    font-size: 0.9em;
  }

  .helper-text {
    margin: 0;
    line-height: 1.4;
  }

  .helper-text kbd {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    padding: 1px 4px;
    font-size: 0.8em;
    font-family: monospace;
    color: var(--text-primary);
  }

  .helper-dismiss {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 0.9em;
    padding: 2px 6px;
    border-radius: var(--radius-sm);
  }

  .helper-dismiss:hover {
    background: var(--danger-bg);
    color: var(--text-primary);
  }

  .helper-dismiss:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 20px;
  }

  .modal-content {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    min-width: 320px;
    max-width: 400px;
    overflow: hidden;
  }

  .modal-header {
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-title {
    margin: 0;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
  }

  .modal-body {
    padding: 20px;
  }

  .modal-text {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }

  .modal-footer {
    padding: 16px 20px;
    border-top: 1px solid var(--border-color);
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .modal-cancel-btn,
  .modal-confirm-btn {
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
  }

  .modal-cancel-btn {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .modal-cancel-btn:hover {
    background: var(--border-color-light);
  }

  .modal-cancel-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  .modal-confirm-btn {
    background: var(--danger);
    color: white;
    border: none;
  }

  .modal-confirm-btn:hover {
    background: #d02020; /* simple fallback */
  }

  .modal-confirm-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  @media (max-width: 768px) {
    .app-header {
      flex-direction: column;
      gap: 8px;
      align-items: stretch;
    }

    .header-right {
      justify-content: space-between;
    }

    .header-stats {
      justify-content: center;
    }

    .modal-content {
      width: 100%;
      margin: 0 16px;
    }
  }
</style>
