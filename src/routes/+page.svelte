<script>
    import { onMount } from "svelte";
    import { loadClips, isLoading, error, clips, pinnedClips } from "$lib/stores/clips";
    import SearchBar from "$lib/components/SearchBar.svelte";
    import PinnedSection from "$lib/components/PinnedSection.svelte";
    import TimelineSection from "$lib/components/TimelineSection.svelte";

    onMount(loadClips);

    async function handleClearAll() {
        if (confirm('Are you sure you want to clear all clips?')) {
            await import("$lib/stores/clips").then(({ clearAllClips }) => clearAllClips());
        }
    }
</script>

<div class="app-container">
    <header class="app-header">
        <div class="header-content">
            <h1 class="app-title">Clipboard Manager</h1>
            <div class="header-stats">
                <span class="stat-item">Total: {$clips.length + $pinnedClips.length}</span>
                <span class="stat-item">Pinned: {$pinnedClips.length}</span>
            </div>
        </div>
        <button class="clear-btn" on:click={handleClearAll} title="Clear all clips">
            Clear All
        </button>
    </header>

    <main class="app-main">
        <SearchBar />
        
        {#if $error}
            <div class="error-state">
                <h3>Something went wrong</h3>
                <p>{$error}</p>
                <button class="retry-btn" on:click={loadClips}>Try Again</button>
            </div>
        {:else if $isLoading}
            <div class="loading-state">
                <p>Loading your clips...</p>
            </div>
        {:else if ($clips.length === 0 && $pinnedClips.length === 0)}
            <div class="empty-state">
                <h3>No clips yet</h3>
                <p>Start copying text to see it appear here</p>
            </div>
        {:else}
            <PinnedSection />
            <TimelineSection />
        {/if}
    </main>
</div>

<style>
    .app-container {
        min-height: 100vh;
        background: #ffffff;
        padding: 20px;
    }

    .app-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 24px;
        padding: 0 8px;
    }

    .header-content {
        display: flex;
        align-items: center;
        gap: 24px;
    }

    .app-title {
        margin: 0;
        font-size: 1.3rem;
        font-weight: 600;
        color: #000;
    }

    .header-stats {
        display: flex;
        gap: 16px;
        font-size: 0.875rem;
        color: #666;
    }

    .stat-item {
        background: #f5f5f5;
        padding: 4px 12px;
        border-radius: 4px;
        border: 1px solid #ddd;
    }

    .clear-btn {
        background: white;
        color: #d32f2f;
        border: 1px solid #ddd;
        padding: 6px 12px;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .clear-btn:hover {
        background: #f5f5f5;
        border-color: #d32f2f;
    }

    .app-main {
        max-width: 800px;
        margin: 0 auto;
        background: white;
        border-radius: 0;
        padding: 0;
    }

    .error-state,
    .loading-state,
    .empty-state {
        text-align: center;
        padding: 40px 20px;
        color: #666;
    }

    .error-state h3,
    .empty-state h3 {
        margin: 0 0 8px 0;
        color: #333;
        font-size: 1.1rem;
    }

    .error-state p,
    .empty-state p {
        margin: 0 0 16px 0;
        color: #888;
    }

    .retry-btn {
        background: white;
        color: #000;
        border: 1px solid #ddd;
        padding: 8px 16px;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
        transition: all 0.2s ease;
    }

    .retry-btn:hover {
        background: #f5f5f5;
        border-color: #000;
    }

    .loading-state {
        color: #888;
    }

    @media (max-width: 768px) {
        .app-container {
            padding: 12px;
        }
        
        .app-header {
            flex-direction: column;
            gap: 12px;
            text-align: center;
        }
        
        .header-content {
            flex-direction: column;
            gap: 12px;
        }
        
        .header-stats {
            justify-content: center;
        }
    }
</style>