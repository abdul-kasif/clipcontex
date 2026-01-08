<script lang="ts">
  import type { AppSettings } from "$lib/stores/types";

  let { settings = $bindable<AppSettings>(), onSave } = $props();

  let ignoredAppsInput = $state("");

  $effect(() => {
    ignoredAppsInput = settings.ignoredApps.join(", ");
  });

  $effect(() => {
    const apps = ignoredAppsInput
      .split(",")
      .map((s) => s.trim())
      .filter((s) => s !== "");
    if (JSON.stringify(apps) !== JSON.stringify(settings.ignoredApps)) {
      settings.ignoredApps = apps;
    }
  });
</script>

<div class="general-settings">
  <div class="settings-header">
    <h2 class="settings-title">General Settings</h2>
  </div>

  <!-- Clipboard Management -->
  <section class="settings-section">
    <h3 class="section-title">Clipboard Management</h3>
    <div class="setting-item">
      <label for="auto-clean" class="setting-label">
        Auto-clean clips after (days)
      </label>
      <input
        id="auto-clean"
        type="number"
        bind:value={settings.autoCleanDays}
        min="1"
        max="365"
        class="setting-input"
      />
    </div>

    <div class="setting-item">
      <label for="max-history" class="setting-label">Max history size</label>
      <input
        id="max-history"
        type="number"
        bind:value={settings.maxHistorySize}
        min="10"
        max="1000"
        class="setting-input"
      />
    </div>
  </section>

  <!-- Privacy -->
  <section class="settings-section">
    <h3 class="section-title">Privacy</h3>
    <div class="setting-item">
      <label for="ignored-apps" class="setting-label">
        Ignore clipboard from these apps (comma-separated)
      </label>
      <input
        id="ignored-apps"
        type="text"
        bind:value={ignoredAppsInput}
        placeholder="Bitwarden,1Password"
        class="setting-input"
      />
    </div>
  </section>

  <!-- Appearance -->
  <section class="settings-section">
    <h3 class="section-title">Appearance</h3>
    <div class="setting-item">
      <label class="radio-label">
        <input
          type="radio"
          name="theme"
          bind:group={settings.darkMode}
          value={false}
          class="setting-radio"
        />
        Light mode
      </label>
    </div>

    <div class="setting-item">
      <label class="radio-label">
        <input
          type="radio"
          name="theme"
          bind:group={settings.darkMode}
          value={true}
          class="setting-radio"
        />
        Dark mode
      </label>
    </div>
  </section>

  <!-- Startup -->
  <section class="settings-section">
    <h3 class="section-title">Startup</h3>
    <div class="setting-item">
      <label class="checkbox-label">
        <input
          type="checkbox"
          class="setting-checkbox"
          bind:checked={settings.isAutostartEnabled}
        />
        Launch ClipContex automatically when system starts
      </label>
    </div>
  </section>

  <!-- Save Button -->
  <div class="actions">
    <div class="action-buttons">
      <button class="save-btn" onclick={() => onSave?.()}>
        Save Settings
      </button>
    </div>
  </div>
</div>

<style>
  .general-settings {
    max-width: 500px;
    margin: 0 auto;
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border-color);
  }

  .settings-title {
    margin: 0;
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
  }

  .settings-section {
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .section-title {
    margin: 0 0 12px 0;
    color: var(--text-primary);
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    letter-spacing: 0.5px;
  }

  .setting-item {
    margin-bottom: 10px;
  }

  .setting-label {
    display: block;
    margin-bottom: 4px;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    font-weight: var(--font-weight-normal);
  }

  .setting-input {
    width: 100%;
    padding: 6px 8px;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    color: var(--text-primary);
    background: var(--bg-primary);
  }

  .setting-input:focus {
    outline: none;
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px var(--focus-ring-color);
  }

  .checkbox-label,
  .radio-label {
    display: flex;
    align-items: center;
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
  }

  .setting-checkbox,
  .setting-radio {
    margin-right: 6px;
    width: 16px;
    height: 16px;
    cursor: pointer;
    accent-color: var(
      --action-primary
    ); /* modern way to style checkbox/radio color */
  }

  .setting-checkbox:focus-visible,
  .setting-radio:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  .actions {
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid var(--border-color);
  }

  .action-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .save-btn {
    background: var(--action-primary);
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    cursor: pointer;
    min-height: 32px;
  }

  .save-btn:hover:not(:disabled) {
    background: var(--action-primary-hover);
  }

  .save-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }

  .save-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
