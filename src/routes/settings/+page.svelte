<script lang="ts">
  //@ts-ignore
  import { goto } from "$app/navigation";
  import { theme } from "$lib/stores/theme";
  import { onMount } from "svelte";
  import { Toaster } from "svelte-french-toast";
  import GeneralSettings from "$lib/components/settings/GeneralSettings.svelte";
  import AboutSettings from "$lib/components/settings/AboutSettings.svelte";
  import { loadSettings, saveSettings } from "$lib/services/settings";
  import "./styles.css";
  import type { AppSettings } from "$lib/stores/types";

  let activeTab = $state<"general" | "about">("general");
  let settings: AppSettings = $state({
    autoCleanDays: 30,
    maxHistorySize: 200,
    darkMode: false,
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
