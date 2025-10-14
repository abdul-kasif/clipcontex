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
    <input
        type="text"
        placeholder="Search clips..."
        bind:value={localQuery}
        on:input={handleSearch}
    />
    {#if localQuery}
        <button on:click={clearSearch} class="clear-btn">×</button>
    {/if}
</div>

<style>
    .search-bar {
        display: flex;
        margin: 16px 0;
    }
    input {
        flex: 1;
        padding: 8px 12px;
        border: 1px solid #ccc;
        border-radius: 6px 0 0 6px;
    }
    .clear-btn {
        border: 1px solid #ccc;
        border-left: none;
        background: #f5f5f5;
        border-radius: 0 6px 6px 0;
        cursor: pointer;
        padding: 0 12px;
    }
</style>
