<script lang="ts">
  import type { ShortcutConfig } from "$lib/stores/types";
  import { validateShortcut } from "$lib/utils/shortcut";

  let { value = $bindable<ShortcutConfig>() } = $props();

  let recording = $state(false);
  let error = $state<string | null>(null);

  function startRecording() {
    recording = true;
    error = null;
  }

  function cancelRecording() {
    recording = false;
    error = null;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!recording) return;

    // Cancel recording on Escape
    if (e.key === "Escape") {
      e.preventDefault();
      cancelRecording();
      return;
    }

    // Ignore if the pressed key is a modifier key itself (Control, Alt, etc.)
    if (["Control", "Alt", "Shift", "Meta"].includes(e.key)) {
      return; // Don't preventDefault here — allow combo building
    }

    e.preventDefault();

    const modifiers: string[] = [];
    if (e.ctrlKey) modifiers.push("Ctrl");
    if (e.shiftKey) modifiers.push("Shift");
    if (e.altKey) modifiers.push("Alt");
    if (e.metaKey) modifiers.push("Super");

    // Normalize the main key
    let key = e.key;
    if (key.length === 1) {
      key = key.toUpperCase();
    }

    const validationError = validateShortcut(modifiers, key);
    if (validationError) {
      error = validationError;
      recording = false;
      return;
    }

    // Update bound value

    value = {
      modifiers,
      key: key.toLowerCase(),
    };

    recording = false;
  }

  function displayShortcut() {
    if (value.modifiers.length === 0 && !value.key) {
      return "Click ✏️ to set";
    }
    return [...value.modifiers, value.key].join(" + ");
  }
</script>

<div class="shortcut-row">
  <span class="shortcut-display" class:recording>
    {recording ? "Recording…" : displayShortcut()}
  </span>
  <button class="edit-btn" aria-label="edit" onclick={startRecording}>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
      ></path>
      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
    </svg>
  </button>
</div>

{#if recording}
  <p class="hint">Press a key combination (e.g. Ctrl+Shift+V)…</p>
{/if}

{#if error}
  <p class="error">{error}</p>
{/if}

<svelte:window on:keydown={handleKeydown} />

<style>
  .shortcut-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .shortcut-display {
    padding: 6px 10px;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    background: var(--bg-primary);
    font-size: var(--font-size-sm);
    min-width: 140px;
    user-select: none;
  }

  .shortcut-display.recording {
    border-color: var(--action-primary);
    color: var(--action-primary);
  }

  .edit-btn {
    color: var(--text-secondary);
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 14px;
    opacity: 0.7;
    transition: opacity 0.2s;
  }

  .edit-btn:hover {
    color: var(--text-primary);
    opacity: 1;
  }

  .hint {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 4px;
    font-style: italic;
  }

  .error {
    font-size: 12px;
    color: var(--error-color, #e5484d);
    margin-top: 4px;
  }
</style>
