<script>
  import { onMount } from "svelte";
  import { theme } from "$lib/stores/theme"; // ← add this
  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutSettings from "$lib/components/settings/AboutSettings.svelte";
  import { loadSettings, saveSettings } from "$lib/services/settings.js";
  import { goto } from "$app/navigation";

  let activeTab = "general";
  let settings = {
    autoCleanDays: 30,
    maxHistorySize: 200,
    darkMode: false,
    ignoredApps: "Bitwarden,1Password",
    isNewUser: true,
    isAutostartEnabled: true,
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
</div>

<style>
  .settings-layout {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    background: var(--bg-secondary);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
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
    font-size: 0.8rem;
    font-weight: 500;
    transition: none;
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .back-btn:hover {
    background: var(--border-color-light);
    color: var(--text-primary);
  }

  .page-title {
    margin: 0 0 0 12px;
    font-size: 1rem;
    font-weight: 600;
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
    font-size: 0.8rem;
    color: var(--text-secondary);
    transition: none;
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

  .nav-icon {
    margin-right: 8px;
    font-size: 0.9rem;
  }

  .nav-label {
    flex: 1;
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
