<script lang="ts">
  import { clips, togglePin, deleteClip } from "$lib/services/clip";
  import type { Clip } from "$lib/stores/types";
  import ClipItem from "./ClipItem.svelte";
  import { format, isToday, isYesterday } from "date-fns";

  /**
   * Pure helper: groups clips by human-readable time buckets.
   * Easy to test, reuse, or move to a util later.
   */
  function groupClipsByTime(list: Clip[]) {
    const groups = new Map<string, Clip[]>();

    for (const clip of list) {
      const date = new Date(clip.created_at);
      let key: string;

      if (isToday(date)) {
        key = "Today";
      } else if (isYesterday(date)) {
        key = "Yesterday";
      } else {
        key = format(date, "MMMM d, yyyy");
      }

      if (!groups.has(key)) groups.set(key, []);
      groups.get(key)!.push(clip);
    }

    // Newest groups first
    return Array.from(groups.entries()).sort((a, b) => {
      const aTime = new Date(a[1][0].created_at).getTime();
      const bTime = new Date(b[1][0].created_at).getTime();
      return bTime - aTime;
    });
  }

  $: grouped = groupClipsByTime($clips);
</script>

<section class="timeline-section">
  {#each grouped as [label, group], i}
    <div class="time-group" aria-labelledby={"time-group-title-" + i}>
      <div class="section-header">
        <h2 id={"time-group-title-" + i} class="section-title">
          {label}
        </h2>
        <span class="item-count">({group.length})</span>
      </div>

      <div class="group-clips">
        {#each group as clip (clip.id)}
          <ClipItem {clip} onPin={togglePin} onDelete={deleteClip} />
        {/each}
      </div>
    </div>
  {/each}
</section>

<style>
  /* ===========================
     Section Tokens
  ============================ */

  .timeline-section {
    --section-gap: 16px;
    --section-header-gap: 6px;
  }

  /* ===========================
     Layout
  ============================ */

  .timeline-section {
    margin-top: 6px;
  }

  .time-group {
    margin-bottom: var(--section-gap);
  }

  /* ===========================
     Unified Section Header
     (Shared contract with PinnedSection)
  ============================ */

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;

    padding: 0 2px;
    margin-bottom: var(--section-header-gap);
  }

  .section-title {
    margin: 0;

    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    letter-spacing: 0.5px;

    color: var(--text-secondary);
  }

  .item-count {
    font-size: var(--font-size-sm);
    color: var(--text-muted);

    background: var(--bg-tertiary);
    padding: 1px 6px;
    border-radius: 12px;

    font-weight: var(--font-weight-normal);
  }

  /* ===========================
     Clips
  ============================ */

  .group-clips {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>
