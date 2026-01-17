<script lang="ts">
  import { searchTerm } from "$lib/services/clip";
</script>

<div class="search-bar">
  <div class="search-input-wrapper">
    <!-- Icon -->
    <svg class="search-icon" viewBox="0 0 24 24" aria-hidden="true">
      <path
        fill="currentColor"
        d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5
           16 5.91 13.09 3 9.5 3S3 5.91 3 9.5
           5.91 16 9.5 16c1.61 0 3.09-.59
           4.23-1.57l.27.28v.79l5 4.99L20.49
           19l-4.99-5z"
      />
    </svg>

    <!-- Input -->
    <input
      type="text"
      class="search-input"
      placeholder="Search clips…"
      bind:value={$searchTerm}
      autocomplete="off"
      spellcheck="false"
    />

    <!-- Clear -->
    {#if $searchTerm}
      <button
        class="clear-btn"
        aria-label="Clear search"
        on:click={() => ($searchTerm = "")}
      >
        ×
      </button>
    {/if}
  </div>
</div>

<style>
  /* ===========================
     Search Bar Tokens
  ============================ */

  .search-bar {
    --search-height: 34px;
    --search-icon-size: 14px;
    --search-gap: 8px;
  }

  /* ===========================
     Layout
  ============================ */

  .search-bar {
    margin: 8px 0 16px;
  }

  .search-input-wrapper {
    display: grid;
    grid-template-columns: var(--search-icon-size) 1fr auto;
    align-items: center;
    gap: var(--search-gap);

    height: var(--search-height);
    padding: 0 8px;

    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);

    box-sizing: border-box;
  }

  /* ===========================
     Focus (Desktop-native)
  ============================ */

  .search-input-wrapper:focus-within {
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px var(--focus-ring-color);
  }

  /* ===========================
     Icon
  ============================ */

  .search-icon {
    width: var(--search-icon-size);
    height: var(--search-icon-size);

    color: var(--text-muted);
    pointer-events: none;
  }

  /* ===========================
     Input
  ============================ */

  .search-input {
    border: none;
    outline: none;
    background: transparent;

    font-family: inherit;
    font-size: var(--font-size-md);
    line-height: 1.35;

    color: var(--text-primary);

    width: 100%;
    min-width: 0;
  }

  .search-input::placeholder {
    color: var(--text-muted);
  }

  /* ===========================
     Clear Button
  ============================ */

  .clear-btn {
    width: 18px;
    height: 18px;

    display: flex;
    align-items: center;
    justify-content: center;

    border: none;
    background: transparent;

    font-size: 0.85rem;
    font-weight: var(--font-weight-semibold);

    color: var(--text-muted);
    cursor: pointer;

    border-radius: 3px;
    padding: 0;
  }

  .clear-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .clear-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
  }
</style>
