<script lang="ts">
  import { searchTerm } from "$lib/services/clip";
</script>

<div class="search-bar">
  <div class="search-input-wrapper">
    <svg
      class="search-icon"
      viewBox="0 0 24 24"
      width="14"
      height="14"
      aria-hidden="true"
    >
      <path
        fill="currentColor"
        d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"
      />
    </svg>
    <input
      type="text"
      placeholder="Search clips..."
      bind:value={$searchTerm}
      class="search-input"
    />
    {#if $searchTerm}
      <button
        on:click={() => ($searchTerm = "")}
        class="clear-btn"
        aria-label="Clear search"
      >
        ×
      </button>
    {/if}
  </div>
</div>

<style>
  .search-bar {
    margin: 8px 0 16px 0;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
  }

  .search-icon {
    position: absolute;
    left: 8px;
    color: var(--text-muted);
    pointer-events: none;
  }

  .search-input {
    flex: 1;
    padding: 8px 8px 8px 30px;
    border: none;
    outline: none;
    font-size: var(--font-size-md);
    background: transparent;
    color: var(--text-primary);
    width: 100%;
  }

  .search-input:focus {
    border-color: var(--action-primary);
    box-shadow: 0 0 0 3px var(--focus-ring-color);
    border-radius: var(--radius-sm);
  }

  .clear-btn {
    position: absolute;
    right: 8px;
    width: 18px;
    height: 18px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 0.8rem;
    padding: 0;
  }

  .clear-btn:hover {
    color: var(--text-primary);
    background: var(--bg-tertiary);
  }

  .clear-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: 3px;
  }
</style>
