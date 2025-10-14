<script>
    import { pinnedClips } from "$lib/stores/clips";
    import { togglePin, deleteClip } from "$lib/stores/clips";
    import ClipItem from "./ClipItem.svelte";
</script>

{#if $pinnedClips.length > 0}
    <section class="pinned-section">
        <h3>⭐ Pinned Clips</h3>
        <div class="clip-list">
            {#each $pinnedClips as clip (clip.id)}
                <ClipItem {clip} onPin={togglePin} onDelete={deleteClip} />
            {/each}
        </div>
    </section>
{/if}

<style>
    .pinned-section {
        margin-bottom: 24px;
    }
    h3 {
        font-size: 1rem;
        font-weight: 600;
        color: #333;
        margin-bottom: 8px;
    }
</style>
