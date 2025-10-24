<script>
    import { onMount } from "svelte";
    import { loadClips, isLoading, error, clips, pinnedClips } from "$lib/services/clips";
    import SearchBar from "$lib/components/main/SearchBar.svelte";
    import PinnedSection from "$lib/components/main/PinnedSection.svelte";
    import TimelineSection from "$lib/components/main/TimelineSection.svelte";

    onMount(loadClips);

    async function handleClearAll() {
        if (confirm('Are you sure you want to clear all clips? This action cannot be undone.')) {
            await import("$lib/services/clips").then(({ clearAllClips }) => clearAllClips());
        }
    }
</script>

<div class="app-container">
    <header class="app-header">
        <h1 class="app-title">Clipboard</h1>
        <div class="header-right">
            <div class="header-stats">
                <span class="stat-item">Total: {$clips.length + $pinnedClips.length}</span>
                <span class="stat-item">Pinned: {$pinnedClips.length}</span>
            </div>
            <button class="clear-btn" on:click={handleClearAll} title="Clear all clips">
                Clear All
            </button>
        </div>
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
                <p>Loading...</p>
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
        padding: 12px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    }

    .app-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 16px;
        padding: 8px 12px;
        background: #f8fafc;
        border-radius: 6px;
        border: 1px solid #e2e8f0;
    }

    .app-title {
        margin: 0;
        font-size: 0.9rem;
        font-weight: 600;
        color: #374151;
    }

    .header-right {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .header-stats {
        display: flex;
        gap: 10px;
        font-size: 0.75rem;
        color: #6b7280;
    }

    .stat-item {
        background: #f1f5f9;
        padding: 2px 8px;
        border-radius: 12px;
        font-weight: 500;
    }

    .clear-btn {
        background: #fee2e2;
        color: #dc2626;
        border: 1px solid #fecaca;
        padding: 4px 12px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.75rem;
        font-weight: 500;
        white-space: nowrap;
    }

    .clear-btn:hover {
        background: #fecaca;
    }

    .app-main {
        max-width: 800px;
        margin: 0 auto;
    }

    .error-state,
    .loading-state,
    .empty-state {
        text-align: center;
        padding: 24px 16px;
        color: #6b7280;
        background: #f8fafc;
        border-radius: 6px;
        border: 1px solid #e2e8f0;
        margin-top: 12px;
    }

    .error-state h3,
    .empty-state h3 {
        margin: 0 0 6px 0;
        color: #374151;
        font-size: 0.9rem;
    }

    .error-state p,
    .empty-state p {
        margin: 0 0 12px 0;
        color: #9ca3af;
        font-size: 0.8rem;
    }

    .retry-btn {
        background: #3b82f6;
        color: white;
        border: none;
        padding: 6px 12px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.75rem;
        font-weight: 500;
    }

    .retry-btn:hover {
        background: #2563eb;
    }

    .loading-state {
        color: #9ca3af;
    }

    @media (max-width: 768px) {
        .app-header {
            flex-direction: column;
            gap: 8px;
            align-items: stretch;
        }
        
        .header-right {
            justify-content: space-between;
        }
        
        .header-stats {
            justify-content: center;
        }
    }
</style>