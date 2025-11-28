<script>
  import { clips, togglePin, deleteClip } from "$lib/services/clips";
  import ClipItem from "./ClipItem.svelte";
  import { format, isToday, isYesterday } from "date-fns";

  $: grouped = groupByTime($clips);

  function groupByTime(list) {
    const groups = {};
    list.forEach((clip) => {
      const date = new Date(clip.created_at);
      let key;

      if (isToday(date)) {
        key = "Today";
      } else if (isYesterday(date)) {
        key = "Yesterday";
      } else {
        key = format(date, "MMMM d, yyyy");
      }

      (groups[key] ||= []).push(clip);
    });

    return Object.entries(groups).sort((a, b) => {
      const dateA = new Date(a[1][0].created_at);
      const dateB = new Date(b[1][0].created_at);
      return dateB - dateA;
    });
  }
</script>

<section class="timeline-section">
  {#each grouped as [time, group]}
    <div class="time-group">
      <div class="time-group-header">
        <h4 class="time-group-title">{time}</h4>
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
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .group-count {
    font-size: 0.65rem;
    color: var(--text-muted);
    background: var(--bg-tertiary);
    padding: 1px 6px;
    border-radius: 12px;
  }

  .group-clips {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
</style>

