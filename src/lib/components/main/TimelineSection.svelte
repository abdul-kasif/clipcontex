<script lang="ts">
  import { clips, togglePin, deleteClip } from "$lib/services/clips";
  import type { Clip } from "$lib/stores/types";
  import ClipItem from "./ClipItem.svelte";
  import { format, isToday, isYesterday } from "date-fns";

  $: grouped = groupByTime($clips);

  function groupByTime(list: Clip[]) {
    const groups = new Map<string, Clip[]>();
    list.forEach((clip: Clip) => {
      const date = new Date(clip.created_at);
      let key: string;

      if (isToday(date)) {
        key = "Today";
      } else if (isYesterday(date)) {
        key = "Yesterday";
      } else {
        key = format(date, "MMMM d, yyyy");
      }

      if (!groups.has(key)) {
        groups.set(key, []);
      }
      groups.get(key)!.push(clip);
    });

    return Array.from(groups.entries()).sort((a, b) => {
      const dateA = new Date(a[1][0].created_at).getTime();
      const dateB = new Date(b[1][0].created_at).getTime();
      return dateB - dateA; // newest first
    });
  }
</script>

<section class="timeline-section">
  {#each grouped as [time, group], i}
    <div class="time-group" aria-labelledby="time-group-title-{i}">
      <div class="time-group-header">
        <h2 id="time-group-title-{i}" class="time-group-title">{time}</h2>
        <span class="group-count">({group.length})</span>
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
  .timeline-section {
    margin-top: 6px;
  }

  .time-group {
    margin-bottom: 16px;
  }

  .time-group-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 6px;
    padding: 0 2px;
  }

  .time-group-title {
    margin: 0;
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .group-count {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    background: var(--bg-tertiary);
    padding: 1px 6px;
    border-radius: 12px;
    font-weight: var(--font-weight-normal);
  }

  .group-clips {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>
