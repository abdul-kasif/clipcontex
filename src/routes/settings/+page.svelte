<script lang="ts">
  //@ts-ignore
  import { goto } from "$app/navigation";
  import { theme } from "$lib/services/theme";
  import { onMount } from "svelte";
  import { Toaster } from "svelte-french-toast";
  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutSettings from "$lib/components/settings/AboutSettings.svelte";
  import { loadSettings, saveSettings } from "$lib/services/settings";
  import type { AppSettings } from "$lib/stores/types";

  let activeTab = $state<"general" | "about">("general");
  let settings: AppSettings = $state({
    autoCleanDays: 30,
    maxHistorySize: 200,
    ignoredApps: ["Bitwarden", "1Password"],
    isNewUser: true,
    isAutostartEnabled: true,
  });

  const tabs = [
    { id: "general", label: "General" },
    { id: "about", label: "About" },
  ] as const;

  async function handleSave() {
    await saveSettings(settings);
  }

  onMount(async () => {
    const loaded: AppSettings = await loadSettings();
    Object.assign(settings, loaded);
  });
</script>

<Toaster />
<div class="settings-layout">
  <div class="header">
    <button class="back-btn" onclick={() => goto("/")}> ← Back </button>
    <h1 class="page-title">Settings</h1>
  </div>

  <div class="main-content">
    <aside class="sidebar">
      <nav class="nav">
        {#each tabs as tab}
          <button
            class="nav-item"
            class:active={activeTab === tab.id}
            onclick={() => (activeTab = tab.id)}
          >
            <span class="nav-label">{tab.label}</span>
          </button>
        {/each}
      </nav>
    </aside>

    <main class="content">
      {#if activeTab === "general"}
        <GeneralSettings bind:settings onSave={handleSave} />
      {:else if activeTab === "about"}
        <AboutSettings />
      {/if}
    </main>
  </div>
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

  .settings-layout {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background: var(--bg-secondary);
    font-family: var(--font-primary);
  }

  .header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background: var(--bg-primary);
    border-bottom: 1px solid var(--border-color);
    position: relative;
  }

  .back-btn {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 6px 12px;
    cursor: pointer;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .back-btn:hover {
    background: var(--border-color-light);
    color: var(--text-primary);
  }

  .back-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: var(--radius-md);
  }

  .page-title {
    margin: 0 0 0 12px;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
  }

  .main-content {
    display: flex;
    flex: 1;
  }

  .sidebar {
    width: 180px;
    background: var(--bg-primary);
    border-right: 1px solid var(--border-color);
    padding: 12px 0;
    flex-shrink: 0;
  }

  .nav {
    display: flex;
    flex-direction: column;
  }

  .nav-item {
    display: flex;
    align-items: center;
    padding: 8px 12px;
    border: none;
    background: transparent;
    cursor: pointer;
    text-align: left;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    border-left: 3px solid transparent;
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
    margin: 0 4px;
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
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  }

  .content {
    flex: 1;
    padding: 16px;
    overflow-y: auto;
  }

  @media (max-width: 768px) {
    .settings-layout {
      flex-direction: column;
    }

    .header {
      padding: 12px 12px 12px 48px;
    }

    .back-btn {
      position: absolute;
      left: 12px;
      top: 50%;
      transform: translateY(-50%);
    }

    .main-content {
      flex-direction: column;
    }

    .sidebar {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid var(--border-color);
    }

    .content {
      padding: 12px;
    }
  }
</style>
