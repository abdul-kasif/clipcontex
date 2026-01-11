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
    <div class="onboarding-header">
      <img class="logo" src="/Square89x89Logo.png" alt="logo" />
      <h1 class="onboarding-title">Welcome to ClipContex</h1>
      <p class="onboarding-subtitle">
        ClipContex automatically remembers your clipboard with full context —
        quickly, privately, and intelligently.
      </p>
    </div>

    <!-- FEATURES OVERVIEW -->
    <div class="features-section">
      <ul class="features-list">
        <FeatureItem
          index={1}
          heading="Smart Clipboard Capture"
          description="Everything you copy is automatically saved and to avoid noise & duplicates, Clipcontex have in-built deduplication."
        />
        <FeatureItem
          index={2}
          heading="Context Awareness"
          description="ClipContex detects the app you copied from, reads the window title, and extracts project names (VS Code, terminals, browsers, etc.)."
        />
        <FeatureItem
          index={3}
          heading="Auto-Tags"
          description="Every clip gets intelligent tags like #code, #url, #terminal, #text or even #project-name."
        />
        <FeatureItem
          index={4}
          heading="Quick Picker"
          description="Press Ctrl + Shift + V anytime to instantly pick and paste from your most recent clips."
        />
        <FeatureItem
          index={5}
          heading="Fast Search"
          description="Search across your entire clipboard history with lightweight fuzzy filtering."
        />
        <FeatureItem
          index={6}
          heading="Pin Important Clips"
          description="Keep frequently used snippets and text anchored at the top of your list."
        />
        <FeatureItem
          index={7}
          heading="Auto-Clean & History Control"
          description="ClipContex automatically trims old history based on your settings — completely customizable."
        />
        <FeatureItem
          index={8}
          heading="Privacy-First Design"
          description="Everything is stored locally. No telemetry, no cloud, no analytics. Password manager clips (Bitwarden, 1Password) are automatically ignored."
        />
        <FeatureItem
          index={9}
          heading="System Tray Access"
          description="Open ClipContex anytime via the tray menu."
        />
      </ul>
    </div>

    <div class="onboarding-actions">
      <button class="primary-btn" onclick={finishOnboarding}>
        Start using ClipContex
      </button>
    </div>
  </div>
</div>

<style>
  :global(html),
  :global(body) {
    height: 10 h;
    margin: 0;
    padding: 0;
  }
  .onboarding {
    width: 100%;
    height: 100vh;
    margin: 0;
    padding: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-secondary);
    box-sizing: border-box;
  }
  .onboarding-container {
    width: 100%;
    max-width: 800px;
    max-height: 90vh;
    padding: 32px;
    background: var(--bg-primary);
    font-family: var(--font-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-md);
    box-sizing: border-box;
    overflow-y: auto;
  }
  .onboarding-header {
    margin: 0 0 16px 0;
    text-align: center;
  }
  .logo {
    margin: 16px 0 0 0;
  }
  .onboarding-title {
    margin: 8px 0 8px 0;
    color: var(--text-primary);
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
  }
  .onboarding-subtitle {
    margin: 0;
    margin-left: auto;
    margin-right: auto;
    max-width: 580px;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }
  .features-section {
    margin: 0 0 16px 0;
  }
  .features-list {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .onboarding-actions {
    text-align: center;
    margin: 32px 0 0 0;
  }
  .primary-btn {
    background: var(--action-primary);
    color: white;
    padding: 10px 20px;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    font-family: var(--font-primary);
    font-weight: var(--font-weight-semibold);
    font-size: var(--font-size-md);
    transition: background-color 0.2s ease;
  }

  .primary-btn:hover {
    background: var(--action-primary-hover);
  }

  @media (max-width: 640px) {
    .onboarding-container {
      padding: 24px;
    }
  }
</style>
