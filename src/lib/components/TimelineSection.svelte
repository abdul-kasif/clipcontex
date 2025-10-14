<script>
    import { clips, togglePin, deleteClip } from "$lib/stores/clips";
    import ClipItem from "./ClipItem.svelte";
    import { format, isToday } from "date-fns";

    $: grouped = groupByTime($clips);

    function groupByTime(list) {
        const groups = {};
        list.forEach((clip) => {
            const date = new Date(clip.created_at);
            const key = isToday(date)
                ? format(date, "h:mm a")
                : format(date, "MMM d, h:mm a");
            (groups[key] ||= []).push(clip);
        });
        return Object.entries(groups).sort(
            (a, b) => new Date(b[0]) - new Date(a[0]),
        );
    }
</script>

<section class="timeline-section">
    {#each grouped as [time, group]}
        <div class="time-group">
            <h4>{time}</h4>
            {#each group as clip (clip.id)}
                <ClipItem {clip} onPin={togglePin} onDelete={deleteClip} />
            {/each}
        </div>
    {/each}
</section>

<style>
    .time-group {
        margin-bottom: 16px;
    }
    h4 {
        margin: 8px 0;
        font-size: 0.9em;
        color: #444;
    }
</style>
