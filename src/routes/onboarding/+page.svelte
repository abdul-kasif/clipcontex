<script>
  import { theme } from "$lib/stores/theme";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import toast, { Toaster } from "svelte-french-toast";
  import "./styles.css";
  import Button from "$lib/components/UI/Button.svelte";
  import FeatureItem from "$lib/components/onboarding/FeatureItem.svelte";

  async function finishOnboarding() {
    try {
      const response = await invoke("complete_onboarding");
      if (response === "success") {
        goto("/");
      } else {
        toast.error("Failed to save settings. Please try again", {
          duration: 1500,
          style:
            "background: var(--bg-primary); border: 1px var(--border-colour); font-size: 0.75rem; color: var(--text-primary); font-weight: 500;",
        });
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
      <Button variant="primary" on:click={finishOnboarding}>
        Start Using ClipContex
      </Button>
    </div>
  </div>
</div>
