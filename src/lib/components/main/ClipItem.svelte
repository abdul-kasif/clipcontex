<script lang="ts">
  import type { Clip } from "$lib/stores/types";
  import { formatDistanceToNow } from "date-fns";

  export let clip: Clip;
  export let onPin: (id: number, pinned: boolean) => void;
  export let onDelete: (id: number) => void;

  $: limitedContent = clip.content.split("\n").slice(0, 3).join("\n");
  $: hasMoreLines = clip.content.split("\n").length > 3;

  $: autoTags = clip.auto_tags
    .split(",")
    .map((t) => t.trim())
    .filter(Boolean);
  $: allTags = [...autoTags];

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
        <span class="more-indicator">…</span>
      {/if}
    </div>

    <div class="clip-actions">
      <button
        class="action-btn pin-btn"
        class:pinned={clip.is_pinned}
        on:click={handlePin}
        aria-label={clip.is_pinned ? "Unpin clip" : "Pin clip"}
      >
        {clip.is_pinned ? "★" : "☆"}
      </button>
      <button
        class="action-btn delete-btn"
        on:click={handleDelete}
        aria-label="Delete clip"
      >
        ×
      </button>
    </div>
  </div>

  <div class="clip-meta">
    <div class="app-info">
      <span class="window-title">{clip.window_title || "Unknown"}</span>
      <span class="time-separator">•</span>
      <span class="time">{relativeTime}</span>
    </div>

    {#if allTags.length > 0}
      <div class="clip-tags">
        {#each allTags as tag}
          <span class="tag">#{tag}</span>
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
  }

  .clip-item.pinned {
    background: var(--warning-bg);
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
    font-family: var(--font-primary);
    font-size: var(--font-size-sm);
    line-height: 1.3;
    color: var(--text-primary);
    word-break: break-word;
    white-space: pre-wrap;
    padding: 2px 0;
    max-height: 3.9rem;
    overflow: hidden;
  }

  .more-indicator {
    color: var(--text-muted);
    font-weight: var(--font-weight-semibold);
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
    font-size: 0.7rem;
    border-radius: 3px;
    padding: 0;
    font-weight: normal;
  }

  .action-btn:hover {
    background: var(--bg-tertiary);
    border-color: var(--border-color-light);
    color: var(--text-primary);
  }

  .action-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: 3px;
  }

  .pin-btn.pinned {
    color: var(--warning);
    background: var(--warning-bg);
    border-color: var(--warning-border);
  }

  .delete-btn:hover,
  .delete-btn:focus-visible {
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
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .window-title {
    color: var(--action-primary);
    font-weight: var(--font-weight-semibold);
  }

  .time-separator {
    color: var(--text-muted);
    margin: 0 4px;
  }

  .time {
    color: var(--text-muted);
    font-weight: var(--font-weight-normal);
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
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    line-height: 1;
    white-space: nowrap;
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
