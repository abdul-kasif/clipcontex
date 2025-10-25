<script>
  import { onMount } from "svelte";
  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutSettings from "$lib/components/settings/AboutSettings.svelte";
  import { loadSettings, saveSettings } from "$lib/services/settings.js";

  let activeTab = "general";
  let settings = {
    autoCleanDays: 30,
    maxHistorySize: 200,
    darkMode: false,
    ignoredApps: "Bitwarden,1Password"
  };

  const tabs = [
    { id: "general", label: "General", icon: "⚙️" },
    { id: "about", label: "About", icon: "ℹ️" },
  ];

  async function handleSave() {
    await saveSettings(settings);
  }

  onMount(async () => {
    settings = await loadSettings();
  });
</script>

<div class="settings-layout">
  <aside class="sidebar">
    <nav class="nav">
      {#each tabs as tab}
        <button
          class="nav-item"
          class:active={activeTab === tab.id}
          onclick={() => (activeTab = tab.id)}
        >
          <span class="nav-icon">{tab.icon}</span>
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

<style>
  .settings-layout {
    display: flex;
    min-height: 100vh;
    background: #f8fafc;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  .sidebar {
    width: 180px;
    background: white;
    border-right: 1px solid #e2e8f0;
    padding: 12px 0;
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
    font-size: 0.8rem;
    color: #6b7280;
    transition: none;
    border-left: 3px solid transparent;
    border-radius: 0 4px 4px 0;
    margin: 0 4px;
  }

  .nav-item:hover {
    background: #f8fafc;
    color: #374151;
  }

  .nav-item.active {
    background: #eff6ff;
    color: #1d4ed8;
    border-left-color: #3b82f6;
  }

  .nav-icon {
    margin-right: 8px;
    font-size: 0.9rem;
  }

  .nav-label {
    flex: 1;
  }

  .content {
    flex: 1;
    padding: 12px;
    overflow-y: auto;
  }

  @media (max-width: 768px) {
    .settings-layout {
      flex-direction: column;
    }
    .sidebar {
      width: 100%;
      border-right: none;
      border-bottom: 1px solid #e2e8f0;
    }
  }
</style>