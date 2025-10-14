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
    ];

    $: relativeTime = formatDistanceToNow(new Date(clip.created_at), {
        addSuffix: true,
    });
    console.log(clip);
</script>

<div class="clip-item">
    <div class="clip-main">
        <div class="clip-content" title={clip.content}>{preview}</div>
        <div class="clip-meta">
            <span class="app">[{clip.app_name}]</span>
            {#each allTags as tag}
                <span class="tag">{tag}</span>
            {/each}
            <span class="time">{relativeTime}</span>
        </div>
    </div>
    <div class="actions">
        <button class="pin-btn" on:click={() => onPin(clip.id, clip.is_pinned)}>
            {clip.is_pinned ? "Unpin" : "Pin"}
        </button>
        <button class="delete-btn" on:click={() => onDelete(clip.id)}
            >Delete</button
        >
    </div>
</div>

<style>
    .clip-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 10px 0;
        border-bottom: 1px solid #eee;
    }
    .clip-main {
        flex: 1;
        overflow: hidden;
    }
    .clip-content {
        font-family: monospace;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    .clip-meta {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
        font-size: 0.8em;
        color: #666;
    }
    .tag {
        background: #eef;
        padding: 2px 6px;
        border-radius: 4px;
    }
    .actions {
        display: flex;
        gap: 6px;
    }
    button {
        font-size: 0.75em;
        padding: 4px 8px;
        border: 1px solid #ccc;
        border-radius: 4px;
        background: #fafafa;
        cursor: pointer;
        transition: 0.15s ease;
    }
    button:hover {
        background: #f0f0ff;
    }
    .time {
        color: #999;
    }
</style>
