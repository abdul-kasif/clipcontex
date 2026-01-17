<script lang="ts">
  import type { Clip } from "$lib/stores/types";
  import { formatDistanceToNow } from "date-fns";

  export let clip: Clip;
  export let onPin: (id: number, pinned: boolean) => void;
  export let onDelete: (id: number) => void;

  $: relativeTime = formatDistanceToNow(new Date(clip.created_at), {
    addSuffix: true,
  });

  $: tags = clip.auto_tags
    .split(",")
    .map((t) => t.trim())
    .filter(Boolean);

  function handlePin() {
    onPin(clip.id, !clip.is_pinned);
  }

  function handleDelete() {
    onDelete(clip.id);
  }
</script>

<article class="clip-item" class:pinned={clip.is_pinned} title={clip.content}>
  <!-- Main Row -->
  <div class="clip-main">
    <div class="clip-content">
      {clip.content}
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

  <!-- Meta -->
  <footer class="clip-meta">
    <div class="app-info">
      <span class="window-title">
        {clip.window_title || "Unknown"}
      </span>

      <span class="time-separator">•</span>

      <span class="time">
        {relativeTime}
      </span>
    </div>

    {#if tags.length}
      <div class="clip-tags">
        {#each tags as tag}
          <span class="tag">{tag}</span>
        {/each}
      </div>
    {/if}
  </footer>
</article>

<style>
  /* ===========================
     Clip Item Tokens
  ============================ */

  .clip-item {
    --clip-lines: 3;
    --clip-line-height: 1.3;
    --clip-content-height: calc(
      var(--clip-lines) * var(--clip-line-height) * 1em
    );

    --clip-pad-y: 8px;
    --clip-pad-x: 12px;
  }

  /* ===========================
     Root
  ============================ */

  .clip-item {
    display: flex;
    flex-direction: column;
    gap: 6px;

    padding: var(--clip-pad-y) var(--clip-pad-x);
    margin-bottom: 8px;

    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);

    box-sizing: border-box;
  }

  .clip-item.pinned {
    background: var(--warning-bg);
    border-left: 3px solid var(--warning);
    border-color: var(--warning-border);
  }

  /* ===========================
     Main Content Row
  ============================ */

  .clip-main {
    display: flex;
    align-items: flex-start;
    gap: 8px;
  }

  .clip-content {
    flex: 1;

    font-size: var(--font-size-sm);
    line-height: var(--clip-line-height);

    height: var(--clip-content-height);

    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: var(--clip-lines);
    overflow: hidden;

    white-space: pre-wrap;
    word-break: break-word;

    color: var(--text-primary);
  }

  /* ===========================
     Actions
  ============================ */

  .clip-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }

  .action-btn {
    width: 22px;
    height: 22px;

    display: flex;
    align-items: center;
    justify-content: center;

    border: 1px solid var(--border-color);
    border-radius: 3px;

    background: var(--bg-primary);
    color: var(--text-secondary);

    cursor: pointer;
    padding: 0;

    font-size: 0.7rem;
    line-height: 1;
  }

  .action-btn:hover {
    background: var(--bg-tertiary);
    border-color: var(--border-color-light);
    color: var(--text-primary);
  }

  .action-btn:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
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

  /* ===========================
     Meta
  ============================ */

  .clip-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
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
    font-weight: var(--font-weight-semibold);
    color: var(--action-primary);
  }

  .time-separator {
    color: var(--text-muted);
  }

  .time {
    color: var(--text-muted);
  }

  .clip-tags {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .tag {
    padding: 1px 8px;
    border-radius: 12px;

    border: 1px solid var(--border-color);
    background: transparent;

    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    line-height: 1;

    color: var(--text-primary);
    white-space: nowrap;
  }

  /* ===========================
     Responsive
  ============================ */

  @media (max-width: 768px) {
    .clip-main {
      flex-direction: column;
      gap: 6px;
    }

    .clip-actions {
      align-self: flex-end;
    }
  }
</style>
