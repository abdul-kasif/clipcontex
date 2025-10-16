<script>
    import { formatDistanceToNow } from "date-fns";
    export let clip;
    export let onPin;
    export let onDelete;

    $: preview =
        clip.content.length > 80
            ? clip.content.slice(0, 77) + "..."
            : clip.content;

    $: allTags = [
        ...clip.auto_tags.split(",").filter(Boolean),
        ...clip.manual_tags.split(",").filter(Boolean),
    ].filter(tag => tag.trim());

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

<div class="clip-item">
    <div class="clip-content-wrapper">
        <div class="clip-content" title={clip.content}>{preview}</div>
        <div class="clip-meta">
            <div class="meta-tags">
                <span class="app-tag">[{clip.app_name}]</span>
                {#each allTags as tag}
                    <span class="tag">{tag.trim()}</span>
                {/each}
            </div>
            <span class="time">{relativeTime}</span>
        </div>
    </div>
    <div class="actions">
        <button 
            class="action-btn pin-btn {clip.is_pinned ? 'pinned' : ''}" 
            on:click={handlePin}
            title={clip.is_pinned ? "Unpin" : "Pin"}
        >
            {clip.is_pinned ? '★' : '☆'}
        </button>
        <button 
            class="action-btn delete-btn" 
            on:click={handleDelete}
            title="Delete"
        >
            ✕
        </button>
    </div>
</div>

<style>
    .clip-item {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        padding: 12px 16px;
        background: white;
        transition: background-color 0.2s ease;
        margin-bottom: 2px;
        border-radius: 4px;
    }

    .clip-item:hover {
        background-color: #f8f8f8;
    }

    .clip-item.pinned {
        background-color: #f0f0f0;
        border-left: 3px solid #000;
    }

    .clip-content-wrapper {
        flex: 1;
        min-width: 0;
        padding-right: 12px;
    }

    .clip-content {
        font-family: 'SF Mono', 'Monaco', 'Inconsolata', monospace;
        font-size: 0.9rem;
        line-height: 1.4;
        color: #333;
        margin-bottom: 6px;
        word-break: break-word;
        white-space: pre-wrap;
    }

    .clip-meta {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-wrap: wrap;
        gap: 8px;
    }

    .meta-tags {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
        flex: 1;
    }

    .app-tag {
        background: #f5f5f5;
        color: #333;
        padding: 2px 6px;
        border-radius: 4px;
        font-size: 0.75rem;
        font-weight: 500;
        border: 1px solid #ddd;
    }

    .tag {
        background: #f8f8f8;
        color: #555;
        padding: 2px 6px;
        border-radius: 4px;
        font-size: 0.75rem;
        border: 1px solid #ddd;
    }

    .time {
        font-size: 0.75rem;
        color: #888;
        font-weight: 500;
    }

    .actions {
        display: flex;
        gap: 4px;
        margin-left: 8px;
    }

    .action-btn {
        width: 28px;
        height: 28px;
        border: 1px solid #ddd;
        background: white;
        color: #666;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s ease;
        font-size: 0.9rem;
        border-radius: 4px;
    }

    .action-btn:hover {
        background: #f0f0f0;
        border-color: #bbb;
        color: #333;
    }

    .pin-btn.pinned {
        color: #000;
        background: #f0f0f0;
    }

    .delete-btn:hover {
        color: #d32f2f;
        border-color: #d32f2f;
    }
</style>