<script>
  import { searchClips, noResults } from "$lib/services/clips";
  let localQuery = "";

  function handleSearch() {
    searchClips(localQuery);
  }

  function clearSearch() {
    localQuery = "";
    searchClips("");
  }
</script>

<div class="search-bar">
  <div class="search-input-wrapper">
    <svg class="search-icon" viewBox="0 0 24 24" width="14" height="14">
      <path
        fill="currentColor"
        d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"
      />
    </svg>
    <input
      type="text"
      placeholder="Search clips..."
      bind:value={localQuery}
      on:input={handleSearch}
      class="search-input"
    />
    {#if localQuery}
      <button
        on:click={clearSearch}
        class="clear-btn"
        aria-label="Clear search"
      >
        ×
      </button>
    {/if}
  </div>

  {#if $noResults && localQuery.trim().length > 0}
    <p class="no-results-msg">No matches found.</p>
  {/if}
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
    transition: none;
  }

  .search-icon {
    position: absolute;
    left: 8px;
    color: var(--text-muted);
    z-index: 1;
  }

  .search-input {
    flex: 1;
    padding: 8px 8px 8px 30px;
    border: none;
    outline: none;
    font-size: 0.8rem;
    background: transparent;
    color: var(--text-primary);
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
    transition: none;
  }

  .clear-btn:hover {
    color: var(--text-primary);
    background: var(--bg-tertiary);
  }

  .no-results-msg {
    margin-top: 4px;
    font-size: 0.75rem;
    color: var(--text-muted);
    text-align: left;
    padding-left: 4px;
  }
</style>

