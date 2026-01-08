<script lang="ts">
  import { pinnedClips } from "$lib/services/clips";
  import { togglePin, deleteClip } from "$lib/services/clips";
  import ClipItem from "./ClipItem.svelte";
</script>

{#if $pinnedClips.length > 0}
  <section class="pinned-section" aria-labelledby="pinned-section-title">
    <div class="section-header">
      <h2 id="pinned-section-title" class="section-title">Pinned</h2>
      <span class="item-count">({$pinnedClips.length})</span>
    </div>
    <div class="clip-list">
      {#each $pinnedClips as clip (clip.id)}
        <ClipItem {clip} onPin={togglePin} onDelete={deleteClip} />
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
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .item-count {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    background: var(--bg-tertiary);
    padding: 1px 6px;
    border-radius: 12px;
    font-weight: var(--font-weight-normal);
  }

  .clip-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>
