<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let autoCleanDays = 30;
  let maxHistorySize = 200;
  let darkMode = false;
  let ignoredApps = "Bitwarden,1Password";

  async function loadSettings() {
    // TODO: Load from config file (v1 uses hardcoded defaults)
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    darkMode = isDark;
  }

  async function saveSettings() {
    // TODO: Save to config file
    alert('Settings saved! (v2 will persist these)');
  }

  onMount(loadSettings);
</script>

<div class="settings-container">
  <h2>Settings</h2>
  
  <section class="settings-section">
    <h3>General</h3>
    <div class="setting-item">
      <label for="auto-clean">Auto-clean clips after (days)</label>
      <input id="auto-clean" type="number" bind:value={autoCleanDays} min="1" max="365" />
    </div>
    <div class="setting-item">
      <label for="max-history">Max history size</label>
      <input id="max-history" type="number" bind:value={maxHistorySize} min="10" max="1000" />
    </div>
  </section>

  <section class="settings-section">
    <h3>Privacy</h3>
    <div class="setting-item">
      <label for="ignored-apps">Ignore clipboard from these apps (comma-separated)</label>
      <input id="ignored-apps" type="text" bind:value={ignoredApps} placeholder="Bitwarden,1Password" />
    </div>
    <div class="setting-item">
      <label>
        <input type="checkbox" disabled /> Enable image capture (v2 feature)
      </label>
    </div>
  </section>

  <section class="settings-section">
    <h3>Appearance</h3>
    <div class="setting-item">
      <label>
        <input type="checkbox" bind:checked={darkMode} /> Dark mode
      </label>
    </div>
  </section>

  <div class="actions">
    <button class="save-btn" on:click={saveSettings}>Save Settings</button>
  </div>
</div>

<style>
  .settings-container {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
  }
  .settings-section {
    margin-bottom: 24px;
    padding-bottom: 16px;
    border-bottom: 1px solid #e5e7eb;
  }
  .settings-section h3 {
    margin: 0 0 16px 0;
    color: #374151;
    font-size: 1.1rem;
  }
  .setting-item {
    margin-bottom: 12px;
  }
  .setting-item label {
    display: block;
    margin-bottom: 4px;
    font-size: 0.9rem;
    color: #4b5563;
  }
  .setting-item input[type="text"],
  .setting-item input[type="number"] {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 0.9rem;
  }
  .setting-item input[type="checkbox"] {
    margin-right: 8px;
  }
  .actions {
    text-align: right;
    margin-top: 16px;
  }
  .save-btn {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 8px 24px;
    border-radius: 4px;
    font-weight: 500;
    cursor: pointer;
  }
  .save-btn:hover {
    background: #2563eb;
  }
</style>