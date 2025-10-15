<script>
    import { searchQuery, searchClips } from "$lib/stores/clips";
    let localQuery = "";

    $: if (localQuery !== $searchQuery) localQuery = $searchQuery;

    function handleSearch(e) {
        searchClips(localQuery);
    }

    function clearSearch() {
        localQuery = "";
        searchClips("");
    }
</script>

<div class="search-bar">
    <div class="search-input-wrapper">
        <svg class="search-icon" viewBox="0 0 24 24" width="16" height="16">
            <path fill="currentColor" d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
        </svg>
        <input
            type="text"
            placeholder="Search clips..."
            bind:value={localQuery}
            on:input={handleSearch}
            class="search-input"
        />
        {#if localQuery}
            <button on:click={clearSearch} class="clear-btn" aria-label="Clear search">
                ×
            </button>
        {/if}
    </div>
</div>

<style>
    .search-bar {
        margin: 16px 0 24px 0;
    }

    .search-input-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        background: white;
        border: 2px solid #ddd;
        border-radius: 6px;
        transition: border-color 0.2s ease;
    }

    .search-input-wrapper:focus-within {
        border-color: #000;
    }

    .search-icon {
        position: absolute;
        left: 12px;
        color: #888;
        z-index: 1;
    }

    .search-input {
        flex: 1;
        padding: 10px 12px 10px 40px;
        border: none;
        outline: none;
        font-size: 1rem;
        background: transparent;
    }

    .search-input::placeholder {
        color: #aaa;
    }

    .clear-btn {
        position: absolute;
        right: 10px;
        width: 24px;
        height: 24px;
        border: none;
        background: transparent;
        color: #888;
        cursor: pointer;
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        transition: color 0.2s ease;
    }

    .clear-btn:hover {
        color: #000;
        background: #f0f0f0;
    }
</style>