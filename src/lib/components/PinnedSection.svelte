<script>
    import { pinnedClips } from "$lib/stores/clips";
    import { togglePin, deleteClip } from "$lib/stores/clips";
    import ClipItem from "./ClipItem.svelte";
</script>

{#if $pinnedClips.length > 0}
    <section class="pinned-section">
        <div class="section-header">
            <h3 class="section-title">Pinned Clips</h3>
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
        margin-bottom: 24px;
    }

    .section-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 12px;
    }

    .section-title {
        margin: 0;
        font-size: 1rem;
        font-weight: 600;
        color: #000;
    }

    .item-count {
        font-size: 0.8rem;
        color: #666;
        background: #f0f0f0;
        padding: 2px 8px;
        border-radius: 12px;
    }

    .clip-list {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }
</style>