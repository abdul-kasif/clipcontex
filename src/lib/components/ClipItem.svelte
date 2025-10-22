<script>
    import { formatDistanceToNow } from "date-fns";
    export let clip;
    export let onPin;
    export let onDelete;

    // Limit content to first 3 lines
    $: limitedContent = clip.content.split('\n').slice(0, 3).join('\n');
    $: hasMoreLines = clip.content.split('\n').length > 3;

    $: autoTags = clip.auto_tags.split(",").filter(tag => tag.trim());
    $: manualTags = clip.manual_tags.split(",").filter(tag => tag.trim());
    $: allTags = [...autoTags, ...manualTags].filter(tag => tag.trim());

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
                {clip.is_pinned ? '★' : '☆'}
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
            <span class="app-name">{clip.app_name}</span>
            {#if clip.window_title}
                <span class="window-separator"> - </span>
                <span class="window-title">{clip.window_title}</span>
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
        background: white;
        margin-bottom: 8px;
        border-radius: 4px;
        border: 1px solid #e5e7eb;
        transition: none;
    }

    .clip-item:hover {
        background-color: #f9fafb;
        border-color: #d1d5db;
    }

    .clip-item.pinned {
        background-color: #fefce8;
        border-left: 3px solid #f59e0b;
        border-color: #fbbf24;
    }

    .clip-main {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: 8px;
    }

    .clip-content {
        flex: 1;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 0.85rem;
        line-height: 1.3;
        color: #374151;
        word-break: break-word;
        white-space: pre-wrap;
        cursor: pointer;
        padding: 2px 0;
        max-height: 3.9rem; /* 3 lines */
        overflow: hidden;
    }

    .more-indicator {
        color: #9ca3af;
        font-weight: bold;
    }

    .clip-content:hover {
        background-color: #f3f4f6;
    }

    .clip-actions {
        display: flex;
        gap: 2px;
        flex-shrink: 0;
    }

    .action-btn {
        width: 22px;
        height: 22px;
        border: 1px solid #e5e7eb;
        background: white;
        color: #6b7280;
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
        background: #f3f4f6;
        border-color: #d1d5db;
        color: #374151;
    }

    .pin-btn.pinned {
        color: #f59e0b;
        background: #fffbeb;
    }

    .delete-btn:hover {
        color: #ef4444;
        border-color: #fca5a5;
        background: #fef2f2;
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
        color: #6b7280;
    }

    .app-name {
        font-weight: 500;
        color: #4b5563;
    }

    .window-separator {
        color: #9ca3af;
    }

    .window-title {
        color: #6b7280;
    }

    .time-separator {
        color: #9ca3af;
        margin: 0 4px;
    }

    .time {
        color: #9ca3af;
        font-weight: 500;
    }

    .clip-tags {
        display: flex;
        gap: 4px;
        flex-wrap: wrap;
    }

    .tag {
        background: #e0e7ff;
        color: #4f46e5;
        padding: 1px 6px;
        border-radius: 12px;
        font-size: 0.65rem;
        font-weight: 500;
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