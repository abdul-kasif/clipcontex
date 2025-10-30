<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { loadClips, isLoading, error, clips, pinnedClips } from "$lib/services/clips";
  import SearchBar from "$lib/components/main/SearchBar.svelte";
  import PinnedSection from "$lib/components/main/PinnedSection.svelte";
  import TimelineSection from "$lib/components/main/TimelineSection.svelte";
  import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

  let kdotoolMissing = false;

  // Check for kdotool when the app starts
  onMount(async () => {
    try {
      const installed = await invoke("is_kdotool_installed");
      if (!installed) {
        kdotoolMissing = true;
      } else {
        await loadClips();
      }
    } catch (err) {
      console.error("Failed to check kdotool:", err);
      kdotoolMissing = true;
    }
  });

  async function handleClearAll() {
    if (confirm("Are you sure you want to clear all clips? This action cannot be undone.")) {
      const { clearAllClips } = await import("$lib/services/clips");
      await clearAllClips();
    }
  }

  async function openSettings() {
    try {
      const settingsWindow = new WebviewWindow("settings");

      if (settingsWindow) {
        const isVisible = await settingsWindow.isVisible();
        if (!isVisible) await settingsWindow.show();
        await settingsWindow.setFocus();
      } else {
        console.warn("Settings window not found in app; fallback to creating one.");
        const newSettings = new WebviewWindow("settings", {
          title: "Settings",
          url: "/settings",
          width: 800,
          height: 600,
          resizable: false,
          center: true
        });
        await newSettings.show();
        await newSettings.setFocus();
      }
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
        <span class="stat-item">Total: {$clips.length + $pinnedClips.length}</span>
        <span class="stat-item">Pinned: {$pinnedClips.length}</span>
      </div>

      <button class="icon-btn" title="Settings" on:click={openSettings}>
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

      <button class="clear-btn" on:click={handleClearAll} title="Clear all clips">
        Clear All
      </button>
    </div>
  </header>

  <main class="app-main">
    {#if kdotoolMissing}
      <div class="error-state">
        <h3>Missing Dependency</h3>
        <p>
          The required tool <strong>kdotool</strong> is not installed on your system.<br />
          Please install it using your package manager:
        </p>
        <pre>sudo dnf install kdotool</pre>
        <button class="retry-btn" on:click={() => location.reload()}>Retry</button>
      </div>
    {:else if $error}
      <div class="error-state">
        <h3>Something went wrong</h3>
        <p>{$error}</p>
        <button class="retry-btn" on:click={loadClips}>Try Again</button>
      </div>
    {:else if $isLoading}
      <div class="loading-state">
        <p>Loading...</p>
      </div>
    {:else if ($clips.length === 0 && $pinnedClips.length === 0)}
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
</div>

<style>
  .app-container {
    min-height: 100vh;
    background: #ffffff;
    padding: 12px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding: 8px 12px;
    background: #f8fafc;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
  }

  .app-title {
    margin: 0;
    font-size: 0.9rem;
    font-weight: 600;
    color: #374151;
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
    color: #6b7280;
  }

  .stat-item {
    background: #f1f5f9;
    padding: 2px 8px;
    border-radius: 12px;
    font-weight: 500;
  }

  .icon-btn {
    background: #f1f5f9;
    border: 1px solid #e2e8f0;
    border-radius: 4px;
    padding: 4px 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s;
  }

  .icon-btn:hover {
    background: #e2e8f0;
  }

  .icon {
    width: 18px;
    height: 18px;
    color: #374151;
  }

  .clear-btn {
    background: #fee2e2;
    color: #dc2626;
    border: 1px solid #fecaca;
    padding: 4px 10px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .clear-btn:hover {
    background: #fecaca;
  }

  .app-main {
    max-width: 800px;
    margin: 0 auto;
  }

  .error-state,
  .loading-state,
  .empty-state {
    text-align: center;
    padding: 24px 16px;
    color: #6b7280;
    background: #f8fafc;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
    margin-top: 12px;
  }

  .error-state h3,
  .empty-state h3 {
    margin: 0 0 6px 0;
    color: #374151;
    font-size: 0.9rem;
  }

  .error-state p,
  .empty-state p {
    margin: 0 0 12px 0;
    color: #9ca3af;
    font-size: 0.8rem;
  }

  pre {
    background: #f1f5f9;
    color: #374151;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 0.8rem;
    text-align: left;
    overflow-x: auto;
  }

  .retry-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .retry-btn:hover {
    background: #2563eb;
  }

  .loading-state {
    color: #9ca3af;
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
  }
</style>
