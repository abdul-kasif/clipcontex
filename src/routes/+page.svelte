<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { platform } from "@tauri-apps/plugin-os";
  import { loadClips, error, clips, pinnedClips } from "$lib/services/clips";
  import SearchBar from "$lib/components/main/SearchBar.svelte";
  import PinnedSection from "$lib/components/main/PinnedSection.svelte";
  import TimelineSection from "$lib/components/main/TimelineSection.svelte";
  import { theme } from "$lib/stores/theme.js";
  import { goto } from "$app/navigation";

  let kdotoolMissing = false;
  let showClearModal = false;

  onMount(async () => {
    try {
      const os = platform();
      console.log("OS: ", os);
      if (os === "linux") {
        const is_installed = await invoke("is_kdotool_installed");
        if (!is_installed) {
          kdotoolMissing = true;
          return;
        }
      }

      // Windows load the clips
      await loadClips();
    } catch (err) {
      console.error("Startup Error:", err);
      kdotoolMissing = true;
    }
  });

  async function handleClearAll() {
    showClearModal = true;
  }

  async function confirmClearAll() {
    const { clearAllClips } = await import("$lib/services/clips");
    await clearAllClips();
    showClearModal = false;
  }

  function cancelClearAll() {
    showClearModal = false;
  }

  async function openSettings() {
    try {
      goto("/settings");
    } catch (err) {
      console.error("Failed to open settings window:", err);
    }
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
        onclick={handleClearAll}
        title="Clear all clips"
      >
        Clear All
      </button>
    </div>
  </header>

  <main class="app-main">
    {#if kdotoolMissing}
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
    {:else if $clips.length === 0 && $pinnedClips.length === 0}
      <div class="empty-state">
        <h3>No clips yet</h3>
        <p>Start copying text to see it appear here</p>
      </div>
    {:else}
      <SearchBar />
      <PinnedSection />
      <TimelineSection />
    {/if}
  </main>

  {#if showClearModal}
    <div class="modal-overlay" onclick={cancelClearAll}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h3 class="modal-title">Clear All Clips</h3>
        </div>
        <div class="modal-body">
          <p class="modal-text">
            Are you sure you want to clear all clips? This action cannot be
            undone.
          </p>
        </div>
        <div class="modal-footer">
          <button class="modal-cancel-btn" onclick={cancelClearAll}>
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
  }
  .app-container {
    min-height: 100vh;
    background: var(--bg-primary);
    padding: 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
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
    font-size: 0.9rem;
    font-weight: 600;
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
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .stat-item {
    background: var(--bg-tertiary);
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: 500;
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
    transition: background 0.2s;
  }

  .icon-btn:hover {
    background: var(--border-color-light);
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
    font-size: 0.75rem;
    font-weight: 500;
  }

  .clear-btn:hover {
    background: var(--danger-border);
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
    font-size: 0.9rem;
  }

  .error-state p,
  .empty-state p {
    margin: 0 0 12px 0;
    color: var(--text-muted);
    font-size: 0.8rem;
  }

  pre {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    padding: 8px 12px;
    border-radius: var(--radius-md);
    font-size: 0.8rem;
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
    font-size: 0.75rem;
    font-weight: 500;
  }

  .retry-btn:hover {
    background: var(--action-primary-hover);
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
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .modal-body {
    padding: 20px;
  }

  .modal-text {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.85rem;
    line-height: 1.5;
  }

  .modal-footer {
    padding: 16px 20px;
    border-top: 1px solid var(--border-color);
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .modal-cancel-btn {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .modal-cancel-btn:hover {
    background: var(--border-color-light);
  }

  .modal-confirm-btn {
    background: var(--danger);
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .modal-confirm-btn:hover {
    background: color-mix(in srgb, var(--danger), black 10%);
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
