<script>
  let { settings = $bindable(), onSave } = $props();

</script>

<div class="general-settings">
  <div class="settings-header">
    <h2 class="settings-title">General Settings</h2>
  </div>

  <section class="settings-section">
    <h3 class="section-title">Clipboard Management</h3>
    <div class="setting-item">
      <label for="auto-clean" class="setting-label"
        >Auto-clean clips after (days)</label
      >
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

  <section class="settings-section">
    <h3 class="section-title">Privacy</h3>
    <div class="setting-item">
      <label for="ignored-apps" class="setting-label">
        Ignore clipboard from these apps (comma-separated)
      </label>
      <input
        id="ignored-apps"
        type="text"
        bind:value={settings.ignoredApps}
        placeholder="Bitwarden,1Password"
        class="setting-input"
      />
    </div>
    <div class="setting-item">
      <label class="checkbox-label">
        <input type="checkbox" disabled class="setting-checkbox" />
        Enable image capture (version 2 feature)
      </label>
    </div>
  </section>

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
    font-size: 1rem;
    font-weight: 600;
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
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .setting-item {
    margin-bottom: 10px;
  }
  .setting-label {
    display: block;
    margin-bottom: 4px;
    font-size: 0.75rem;
    color: var(--text-secondary);
    font-weight: 500;
  }
  .setting-input {
    width: 100%;
    padding: 6px 8px;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    font-size: 0.8rem;
    color: var(--text-primary);
    background: var(--bg-primary);
  }
  .setting-input:focus {
    outline: none;
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--action-primary), transparent 90%);
  }
  .checkbox-label {
    display: flex;
    align-items: center;
    font-size: 0.75rem;
    color: var(--text-secondary);
    cursor: pointer;
  }
  .setting-checkbox {
    margin-right: 6px;
    width: 14px;
    height: 14px;
    border: 1px solid var(--border-color);
    border-radius: 3px;
    cursor: pointer;
  }
  .radio-label {
    display: flex;
    align-items: center;
    font-size: 0.75rem;
    color: var(--text-secondary);
    cursor: pointer;
    margin-bottom: 4px;
  }
  .setting-radio {
    margin-right: 6px;
    width: 14px;
    height: 14px;
    border: 1px solid var(--border-color);
    border-radius: 50%;
    cursor: pointer;
  }
  .actions {
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid var(--border-color);
  }
  .action-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 6px;
  }
  .save-btn {
    background: var(--action-primary);
    color: white;
    border: none;
    padding: 5px 14px;
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
  }
  .save-btn:hover:not(:disabled) {
    background: var(--action-primary-hover);
  }
  .save-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>