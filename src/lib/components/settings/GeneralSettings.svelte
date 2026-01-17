<script lang="ts">
  import type { AppSettings } from "$lib/stores/types";
  import ShortcutInput from "./ShortcutInput.svelte";

  let { settings = $bindable<AppSettings>(), onSave } = $props();

  /**
   * Local editable proxy for ignored apps input.
   * Keeps text input UX smooth while syncing back to settings.
   */
  let ignoredAppsText = $state("");

  $effect(() => {
    ignoredAppsText = settings.ignoredApps.join(", ");
  });

  function syncIgnoredApps() {
    const apps = ignoredAppsText
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);

    settings.ignoredApps = apps;
  }
</script>

<div class="general-settings">
  <!-- Page Header -->
  <header class="settings-header">
    <h2 class="settings-title">General Settings</h2>
  </header>

  <!-- Quick Picker -->
  <section class="settings-section">
    <h3 class="section-title">Quick Picker</h3>

    <div class="field">
      <label class="field-label" for="shortcut"> Keyboard shortcut </label>

      <div class="field-control">
        <ShortcutInput bind:value={settings.quickPickerShortcut} />
      </div>

      <p class="field-hint">
        Press this shortcut to open the quick picker anywhere.
      </p>
    </div>
  </section>

  <!-- Clipboard -->
  <section class="settings-section">
    <h3 class="section-title">Clipboard Management</h3>

    <div class="field">
      <label for="auto-clean" class="field-label">
        Auto-clean clips after (days)
      </label>

      <div class="field-control">
        <input
          id="auto-clean"
          type="number"
          min="1"
          max="365"
          bind:value={settings.autoCleanDays}
          class="field-input"
        />
      </div>
    </div>

    <div class="field">
      <label for="max-history" class="field-label"> Max history size </label>

      <div class="field-control">
        <input
          id="max-history"
          type="number"
          min="10"
          max="1000"
          bind:value={settings.maxHistorySize}
          class="field-input"
        />
      </div>
    </div>
  </section>

  <!-- Privacy -->
  <section class="settings-section">
    <h3 class="section-title">Privacy</h3>

    <div class="field">
      <label for="ignored-apps" class="field-label">
        Ignore clipboard from these apps
      </label>

      <div class="field-control">
        <input
          id="ignored-apps"
          type="text"
          placeholder="Bitwarden, 1Password"
          bind:value={ignoredAppsText}
          onblur={syncIgnoredApps}
          class="field-input"
        />
      </div>

      <p class="field-hint">
        Comma-separated list. Clipboard from these apps will be ignored.
      </p>
    </div>
  </section>

  <!-- Startup -->
  <section class="settings-section">
    <h3 class="section-title">Startup</h3>

    <label class="checkbox-field">
      <input
        type="checkbox"
        class="checkbox"
        bind:checked={settings.isAutostartEnabled}
      />
      <span> Launch ClipContex automatically when the system starts </span>
    </label>
  </section>

  <!-- Actions -->
  <footer class="settings-actions">
    <button class="save-btn" onclick={() => onSave?.()}> Save Settings </button>
  </footer>
</div>

<style>
  /* ===========================
     Layout
  ============================ */

  .general-settings {
    max-width: 520px;
    margin: 0 auto;
  }

  /* ===========================
     Header
  ============================ */

  .settings-header {
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

  /* ===========================
     Sections
  ============================ */

  .settings-section {
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .settings-section:last-of-type {
    border-bottom: none;
  }

  .section-title {
    margin: 0 0 12px;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
    letter-spacing: 0.4px;
  }

  /* ===========================
     Fields
  ============================ */

  .field {
    margin-bottom: 14px;
  }

  .field-label {
    display: block;
    margin-bottom: 4px;

    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-normal);
    color: var(--text-secondary);
  }

  .field-control {
    max-width: 260px;
  }

  .field-input {
    width: 100%;
    padding: 6px 8px;

    font-size: var(--font-size-sm);
    font-family: inherit;

    background: var(--bg-primary);
    color: var(--text-primary);

    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
  }

  .field-input:focus {
    outline: none;
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px var(--focus-ring-color);
  }

  .field-hint {
    margin: 4px 0 0;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    line-height: 1.4;
  }

  /* ===========================
     Checkbox
  ============================ */

  .checkbox-field {
    display: flex;
    align-items: center;
    gap: 8px;

    font-size: var(--font-size-sm);
    color: var(--text-secondary);

    cursor: pointer;
    user-select: none;
  }

  .checkbox {
    width: 16px;
    height: 16px;
    accent-color: var(--action-primary);
  }

  .checkbox:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  /* ===========================
     Actions
  ============================ */

  .settings-actions {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid var(--border-color);

    display: flex;
    justify-content: flex-end;
  }

  .save-btn {
    min-height: 32px;
    padding: 8px 16px;

    background: var(--action-primary);
    color: white;

    border: none;
    border-radius: var(--radius-sm);

    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);

    cursor: pointer;
  }

  .save-btn:hover {
    background: var(--action-primary-hover);
  }

  .save-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }
</style>
