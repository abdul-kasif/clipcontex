<script>
    import { pinnedClips } from "$lib/stores/clips";
    import { togglePin, deleteClip } from "$lib/stores/clips";
    import ClipItem from "./ClipItem.svelte";
</script>

{#if $pinnedClips.length > 0}
    <section class="pinned-section">
        <div class="section-header">
            <h3 class="section-title">Pinned</h3>
            <span class="item-count">({$pinnedClips.length})</span>
        </div>
        <div class="clip-list">
            {#each $pinnedClips as clip (clip.id)}
                <ClipItem 
                    {clip} 
                    onPin={togglePin} 
                    onDelete={deleteClip}
                />
            {/each}
        </div>
    </section>
{/if}

<style>
    .pinned-section {
        margin-bottom: 16px;
    }

    .section-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 8px;
        padding: 0 2px;
    }

    .section-title {
        margin: 0;
        font-size: 0.8rem;
        font-weight: 600;
        color: #374151;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .item-count {
        font-size: 0.7rem;
        color: #9ca3af;
        background: #f1f5f9;
        padding: 1px 6px;
        border-radius: 12px;
    }

    .clip-list {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }
</style>