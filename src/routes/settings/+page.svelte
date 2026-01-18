<script lang="ts">
  //@ts-ignore
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { Toaster } from "svelte-french-toast";

  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutUs from "$lib/components/settings/AboutUs.svelte";

  import { loadSettings, saveSettings } from "$lib/services/settings";
  import type { AppSettings } from "$lib/stores/types";

  let activeTab: "general" | "about" = "general";

  let settings: AppSettings = {
    autoCleanDays: 30,
    maxHistorySize: 200,
    ignoredApps: ["Bitwarden", "1Password"],
    isNewUser: true,
    isAutostartEnabled: true,
    quickPickerShortcut: {
      modifiers: ["Ctrl", "Shift"],
      key: "v",
    },
  };

  const tabs = [
    { id: "general", label: "General" },
    { id: "about", label: "About" },
  ] as const;

  async function handleSave() {
    await saveSettings(settings);
  }

  onMount(async () => {
    const loaded = await loadSettings();
    Object.assign(settings, loaded);
  });
</script>

<Toaster />

<div class="settings-layout">
  <!-- Header -->
  <header class="settings-header">
    <button class="back-btn" on:click={() => goto("/")}> ← Back </button>
    <h1 class="page-title">Settings</h1>
  </header>

  <!-- Main -->
  <div class="settings-main">
    <!-- Sidebar -->
    <aside class="settings-sidebar">
      <nav class="settings-nav">
        {#each tabs as tab}
          <button
            class="nav-item"
            class:active={activeTab === tab.id}
            on:click={() => (activeTab = tab.id)}
          >
            {tab.label}
          </button>
        {/each}
      </nav>
    </aside>

    <!-- Scrollable Content -->
    <main class="settings-content">
      {#if activeTab === "general"}
        <GeneralSettings bind:settings onSave={handleSave} />
      {:else}
        <AboutUs />
      {/if}
    </main>
  </div>
</div>

<style>
  /* ===========================
     Global (Settings Page Only)
  ============================ */

  :global(html),
  :global(body) {
    width: 100%;
    height: 100%;
    margin: 0;
    overflow: hidden;

    font-family: var(--font-primary);
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  /* ===========================
     Layout Tokens
  ============================ */

  .settings-layout {
    --header-height: 52px;
    --sidebar-width: 180px;
    --page-padding: 16px;
  }

  /* ===========================
     Root Layout
  ============================ */

  .settings-layout {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;

    background: var(--bg-secondary);
    overflow: hidden;
  }

  /* ===========================
     Header (Fixed)
  ============================ */

  .settings-header {
    flex-shrink: 0;

    height: var(--header-height);
    display: flex;
    align-items: center;
    gap: 12px;

    padding: 0 var(--page-padding);
    background: var(--bg-primary);

    border-bottom: 1px solid var(--border-color);
  }

  .back-btn {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);

    padding: 6px 12px;
    cursor: pointer;

    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--text-secondary);
  }

  .back-btn:hover {
    background: var(--border-color-light);
    color: var(--text-primary);
  }

  .back-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  .page-title {
    margin: 0;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
  }

  /* ===========================
     Main Area
  ============================ */

  .settings-main {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  /* ===========================
     Sidebar (Fixed)
  ============================ */

  .settings-sidebar {
    width: var(--sidebar-width);
    flex-shrink: 0;

    background: var(--bg-primary);
    border-right: 1px solid var(--border-color);
    padding: 12px 0;
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
  }

  .nav-item {
    padding: 8px 12px;
    margin: 0 6px;

    border: none;
    background: transparent;
    text-align: left;
    cursor: pointer;

    font-size: var(--font-size-sm);
    color: var(--text-secondary);

    border-left: 3px solid transparent;
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  }

  .nav-item:hover {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--bg-accent);
    color: var(--action-primary-hover);
    border-left-color: var(--action-primary);
  }

  .nav-item:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: -2px;
  }

  /* ===========================
     Content (Scrollable)
  ============================ */

  .settings-content {
    flex: 1;
    overflow-y: auto;

    scroll-behavior: smooth;
    scrollbar-gutter: stable;
    scrollbar-width: thin;
    scrollbar-color: var(--border-color) transparent;
    padding: var(--page-padding);
  }

  /* ===========================
     Responsive (Optional)
  ============================ */

  @media (max-width: 768px) {
    .settings-main {
      flex-direction: column;
    }

    .settings-sidebar {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid var(--border-color);
    }
  }
</style>
