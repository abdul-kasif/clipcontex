<script>
  import { formatDistanceToNow } from "date-fns";
  export let clip;
  export let onPin;
  export let onDelete;

  // Limit content to first 3 lines
  $: limitedContent = clip.content.split("\n").slice(0, 3).join("\n");
  $: hasMoreLines = clip.content.split("\n").length > 3;

  $: autoTags = clip.auto_tags.split(",").filter((tag) => tag.trim());
  $: allTags = [...autoTags].filter((tag) => tag.trim());

  $: relativeTime = formatDistanceToNow(new Date(clip.created_at), {
    addSuffix: true,
  });

  function handlePin() {
    onPin(clip.id, !clip.is_pinned);
  }

  function handleDelete() {
    onDelete(clip.id);
  }
</script>

<div class="clip-item" class:pinned={clip.is_pinned}>
  <div class="clip-main">
    <div class="clip-content" title={clip.content}>
      {limitedContent}
      {#if hasMoreLines}
        <span class="more-indicator">...</span>
      {/if}
    </div>

    <div class="clip-actions">
      <button
        class="action-btn pin-btn"
        class:pinned={clip.is_pinned}
        on:click={handlePin}
        title={clip.is_pinned ? "Unpin" : "Pin"}
      >
        {clip.is_pinned ? "★" : "☆"}
      </button>
      <button
        class="action-btn delete-btn"
        on:click={handleDelete}
        title="Delete"
      >
        ×
      </button>
    </div>
  </div>

  <div class="clip-meta">
    <div class="app-info">
      {#if clip.window_title}
        <span class="window-title">{clip.window_title}</span>
      {:else}
        <span class="window-title">Unknown</span>
      {/if}
      <span class="time-separator">•</span>
      <span class="time">{relativeTime}</span>
    </div>

    {#if allTags.length > 0}
      <div class="clip-tags">
        {#each allTags as tag}
          <span class="tag">{tag.trim()}</span>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .clip-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 10px 12px;
    background: var(--bg-primary);
    margin-bottom: 8px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border-color);
    transition: none;
  }

  .clip-item:hover {
    background-color: var(--bg-tertiary);
    border-color: var(--border-color-light);
  }

  .clip-item.pinned {
    background-color: var(--warning-bg);
    border-left: 3px solid var(--warning);
    border-color: var(--warning-border);
  }

  .clip-main {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 8px;
  }

  .clip-content {
    flex: 1;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
    font-size: 0.85rem;
    line-height: 1.3;
    color: var(--text-primary);
    word-break: break-word;
    white-space: pre-wrap;
    cursor: pointer;
    padding: 2px 0;
    max-height: 3.9rem; /* 3 lines */
    overflow: hidden;
  }

  .more-indicator {
    color: var(--text-muted);
    font-weight: bold;
  }

  .clip-content:hover {
    background-color: var(--bg-tertiary);
  }

  .clip-actions {
    display: flex;
    gap: 2px;
    flex-shrink: 0;
  }

  .action-btn {
    width: 22px;
    height: 22px;
    border: 1px solid var(--border-color);
    background: var(--bg-primary);
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: none;
    font-size: 0.7rem;
    border-radius: 3px;
    font-weight: normal;
  }

  .action-btn:hover {
    background: var(--bg-tertiary);
    border-color: var(--border-color-light);
    color: var(--text-primary);
  }

  .pin-btn.pinned {
    color: var(--warning);
    background: var(--warning-bg);
  }

  .delete-btn:hover {
    color: var(--danger);
    border-color: var(--danger-border);
    background: var(--danger-bg);
  }

  .clip-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: 2px;
  }

  .app-info {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-wrap: wrap;
    font-size: 0.7rem;
    color: var(--text-secondary);
  }

  .window-title {
    color: var(--action-primary);
    font-weight: 600;
  }

  .time-separator {
    color: var(--text-muted);
    margin: 0 4px;
  }

  .time {
    color: var(--text-muted);
    font-weight: 500;
  }

  .clip-tags {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .tag {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    padding: 1px 8px;
    border-radius: 12px;
    font-size: 0.65rem;
    font-weight: 500;
    transition: all 0.2s ease-in-out;
  }

  @media (max-width: 768px) {
    .clip-main {
      flex-direction: column;
      gap: 6px;
    }

    .clip-actions {
      align-self: flex-end;
    }

    .app-info {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>
