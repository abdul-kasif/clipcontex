<script>
    import { onMount } from "svelte";
    import { loadClips, isLoading, error } from "$lib/stores/clips";
    import SearchBar from "$lib/components/SearchBar.svelte";
    import PinnedSection from "$lib/components/PinnedSection.svelte";
    import TimelineSection from "$lib/components/TimelineSection.svelte";
    import { invoke } from "@tauri-apps/api/core";

    onMount(loadClips);
</script>

<main>
    <SearchBar />
    {#if $error}
        <div class="error">❌ {$error}</div>
    {:else if $isLoading}
        <div class="loading">Loading clips...</div>
    {:else}
        <PinnedSection />
        <TimelineSection />
    {/if}
</main>
<button on:click={() => {
  invoke('capture_current_clip').then(clip => console.log('Captured:', clip));
}}>
  Capture Current Clipboard
</button>

<style>
    main {
        padding: 16px;
        max-width: 800px;
        margin: 0 auto;
        font-family: system-ui, sans-serif;
    }
    .error {
        color: red;
        padding: 8px;
    }
    .loading {
        text-align: center;
        color: #666;
    }
</style>
