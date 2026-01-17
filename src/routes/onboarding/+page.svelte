<script>
  //@ts-ignore
  import { goto } from "$app/navigation";
  import { Toaster } from "svelte-french-toast";
  import FeatureItem from "$lib/components/onboarding/FeatureItem.svelte";
  import { theme } from "$lib/services/theme";
  import { completeOnboarding } from "$lib/services/settings";

  async function finishOnboarding() {
    try {
      const response = await completeOnboarding();
      if (response === "ok") {
        goto("/");
      }
    } catch (e) {
      console.error("Failed to complete onboarding:", e);
    }
  }
</script>

<Toaster />

<div class="onboarding">
  <div class="onboarding-container">
    <!-- Header -->
    <header class="onboarding-header">
      <img class="logo" src="/Square89x89Logo.png" alt="ClipContex logo" />

      <h1 class="onboarding-title">Welcome to ClipContex</h1>

      <p class="onboarding-subtitle">
        ClipContex automatically remembers your clipboard with full context —
        quickly, privately, and intelligently.
      </p>
    </header>

    <!-- Features -->
    <section class="features-section">
      <ul class="features-list">
        <FeatureItem
          index={1}
          heading="Smart Clipboard Capture"
          description="Everything you copy is automatically saved. Built-in deduplication avoids noise and repetition."
        />

        <FeatureItem
          index={2}
          heading="Context Awareness"
          description="Detects the app you copied from, reads window titles, and extracts project context."
        />

        <FeatureItem
          index={3}
          heading="Auto-Tags"
          description="Every clip is intelligently tagged like #code, #url, #terminal, or project-specific labels."
        />

        <FeatureItem
          index={4}
          heading="Quick Picker"
          description="Press Ctrl + Shift + V anytime to instantly paste from your recent clips."
        />

        <FeatureItem
          index={5}
          heading="Fast Search"
          description="Search across your entire clipboard history using fast fuzzy filtering."
        />

        <FeatureItem
          index={6}
          heading="Pin Important Clips"
          description="Keep frequently used text and snippets pinned to the top."
        />

        <FeatureItem
          index={7}
          heading="Auto-Clean & History Control"
          description="Automatically trims old history based on your preferences."
        />

        <FeatureItem
          index={8}
          heading="Privacy-First Design"
          description="Everything stays local. No cloud, no telemetry, no analytics."
        />

        <FeatureItem
          index={9}
          heading="System Tray Access"
          description="Open ClipContex anytime from the system tray."
        />
      </ul>
    </section>

    <!-- Actions -->
    <footer class="onboarding-actions">
      <button class="primary-btn" on:click={finishOnboarding}>
        Start using ClipContex
      </button>
    </footer>
  </div>
</div>

<style>
  /* ===========================
     Global Normalization
  ============================ */

  :global(html),
  :global(body) {
    width: 100%;
    height: 100%;
    margin: 0;
    overflow: hidden;

    font-family: var(--font-primary);
    line-height: 1.4;
    text-size-adjust: 100%;
    -webkit-text-size-adjust: 100%;
  }

  /* ===========================
     Design Tokens
  ============================ */

  .onboarding {
    --container-max-w: 820px;
    --container-pad: 32px;

    --section-gap: 24px;
    --item-gap: 16px;
  }

  /* ===========================
     Root Layout
  ============================ */

  .onboarding {
    width: 100%;
    height: 100%;

    display: flex;
    align-items: center;
    justify-content: center;

    padding: 16px;
    box-sizing: border-box;

    background: var(--bg-secondary);
  }

  /* ===========================
     Container
  ============================ */

  .onboarding-container {
    width: 100%;
    max-width: var(--container-max-w);
    max-height: 100%;

    display: flex;
    flex-direction: column;

    padding: var(--container-pad);
    box-sizing: border-box;

    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);

    overflow-y: auto;

    scrollbar-gutter: stable;
    scrollbar-width: thin;
    scrollbar-color: var(--border-color) transparent;
  }

  /* ===========================
     Header
  ============================ */

  .onboarding-header {
    text-align: center;
    margin-bottom: var(--section-gap);
  }

  .logo {
    width: 72px;
    height: 72px;
    margin-bottom: 12px;
  }

  .onboarding-title {
    margin: 0 0 8px;
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
  }

  .onboarding-subtitle {
    max-width: 600px;
    margin: 0 auto;
    font-size: var(--font-size-sm);
    line-height: 1.5;
    color: var(--text-secondary);
  }

  /* ===========================
     Features
  ============================ */

  .features-section {
    flex: 1;
    margin-bottom: var(--section-gap);
  }

  .features-list {
    list-style: none;
    margin: 0;
    padding: 0;

    display: grid;
    gap: var(--item-gap);
  }

  /* ===========================
     Actions
  ============================ */

  .onboarding-actions {
    text-align: center;
    padding-top: 8px;
  }

  .primary-btn {
    background: var(--action-primary);
    color: #fff;

    padding: 10px 22px;
    border: none;
    border-radius: var(--radius-md);

    font-family: inherit;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);

    cursor: pointer;
  }

  .primary-btn:hover {
    background: var(--action-primary-hover);
  }

  /* ===========================
     Responsive
  ============================ */

  @media (max-width: 640px) {
    .onboarding-container {
      padding: 24px;
    }
  }
</style>
