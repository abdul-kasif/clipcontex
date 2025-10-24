<script>
    import { searchClips } from "$lib/services/clips";
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
        margin: 8px 0 16px 0;
    }

    .search-input-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        background: white;
        border: 1px solid #e5e7eb;
        border-radius: 4px;
        transition: none;
    }

    .search-input-wrapper:focus-within {
        border-color: #3b82f6;
        outline: none;
    }

    .search-icon {
        position: absolute;
        left: 8px;
        color: #9ca3af;
        z-index: 1;
    }

    .search-input {
        flex: 1;
        padding: 8px 8px 8px 30px;
        border: none;
        outline: none;
        font-size: 0.8rem;
        background: transparent;
        color: #374151;
    }

    .search-input::placeholder {
        color: #d1d5db;
    }

    .clear-btn {
        position: absolute;
        right: 8px;
        width: 18px;
        height: 18px;
        border: none;
        background: transparent;
        color: #9ca3af;
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
        color: #374151;
        background: #f3f4f6;
    }
</style>