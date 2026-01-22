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
        Your clipboard, now smarter. ClipContex remembers what you copy — and
        the context around it — so you never lose track again.
      </p>
    </header>

    <!-- Features -->
    <section class="features-section">
      <ul class="features-list">
        <FeatureItem
          index={1}
          heading="Smart Context Capture"
          description="Knows which app and window each clip came from — no more guessing."
        />

        <FeatureItem
          index={2}
          heading="Quick Picker (Ctrl+Shift+V)"
          description="Instantly search and paste from your history — without leaving your workflow."
        />

        <FeatureItem
          index={3}
          heading="Fast Fuzzy Search"
          description="Find anything you’ve copied, even if you only remember part of it."
        />

        <FeatureItem
          index={4}
          heading="Auto-Tags"
          description="Clips are automatically tagged: #code, #url, #email, #terminal, and more."
        />

        <FeatureItem
          index={5}
          heading="Pin Important Clips"
          description="Keep your go-to snippets always at the top."
        />

        <FeatureItem
          index={6}
          heading="Automatic Cleanup"
          description="Old clips are removed based on your preferences — no manual housekeeping."
        />

        <FeatureItem
          index={7}
          heading="100% Private & Offline"
          description="Your data never leaves your device. No cloud, no tracking, no compromises."
        />

        <FeatureItem
          index={8}
          heading="System Tray Access"
          description="Open ClipContex anytime from your system tray — fast and unobtrusive."
        />

        <FeatureItem
          index={9}
          heading="Skips Passwords Automatically"
          description="Ignores clipboard content from Bitwarden, 1Password, and other secure apps."
        />
      </ul>
    </section>

    <!-- Actions -->
    <footer class="onboarding-actions">
      <button class="primary-btn" on:click={finishOnboarding}>
        Start Using ClipContex →
      </button>
      <p class="onboarding-note">
        It runs quietly in the background. You can open it anytime with
        Ctrl+Shift+V.
      </p>
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

    --section-gap: 28px;
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
    animation: float 3s ease-in-out infinite;
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-6px);
    }
  }

  .onboarding-title {
    margin: 0 0 10px;
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--text-primary);
  }

  .onboarding-subtitle {
    max-width: 600px;
    margin: 0 auto;
    font-size: var(--font-size-md);
    line-height: 1.55;
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
    padding-top: 12px;
  }

  .primary-btn {
    background: var(--action-primary);
    color: #fff;

    padding: 12px 28px;
    border: none;
    border-radius: var(--radius-md);

    font-family: inherit;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);

    cursor: pointer;
    transition: background 0.2s ease;
  }

  .primary-btn:hover {
    background: var(--action-primary-hover);
    transform: translateY(-1px);
  }

  .onboarding-note {
    margin-top: 12px;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }

  /* ===========================
     Responsive
  ============================ */

  @media (max-width: 640px) {
    .onboarding-container {
      padding: 24px;
    }

    .onboarding-title {
      font-size: var(--font-size-lg);
    }

    .onboarding-subtitle {
      font-size: var(--font-size-sm);
    }
  }
</style>
