<script>
  import { theme } from "$lib/stores/theme";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import toast, { Toaster } from "svelte-french-toast";

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
      <img src="src/assests/Square89x89Logo.png" alt="logo" />

      <h1 class="onboarding-title">Welcome to ClipContex</h1>
      <p class="onboarding-subtitle">
        ClipContex automatically remembers your clipboard with full context —
        quickly, privately, and intelligently.
      </p>
    </div>

    <!-- FEATURES OVERVIEW -->
    <div class="features-section">
      <ul class="features-list">
        <li class="feature-item">
          <div class="feature-icon">1.</div>
          <div class="feature-content">
            <h3>Smart Clipboard Capture</h3>
            <p>
              Everything you copy is automatically saved with a 300ms debounce
              and 10-second deduplication to avoid noise & duplicates.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">2.</div>
          <div class="feature-content">
            <h3>Context Awareness</h3>
            <p>
              ClipContex detects the app you copied from, reads the window
              title, and extracts project names (VS Code, terminals, browsers,
              etc.).
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">3.</div>
          <div class="feature-content">
            <h3>Auto-Tags</h3>
            <p>
              Every clip gets intelligent tags like <strong>#code</strong>,
              <strong>#url</strong>,
              <strong>#terminal</strong>, <strong>#text</strong> or even
              <strong>#project-name</strong>.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">4.</div>
          <div class="feature-content">
            <h3>Quick Picker</h3>
            <p>
              Press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd> anytime to
              instantly pick and paste from your most recent clips.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">5.</div>
          <div class="feature-content">
            <h3>Fast Search</h3>
            <p>
              Search across your entire clipboard history with lightweight fuzzy
              filtering.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">6.</div>
          <div class="feature-content">
            <h3>Pin Important Clips</h3>
            <p>
              Keep frequently used snippets and text anchored at the top of your
              list.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">7.</div>
          <div class="feature-content">
            <h3>Auto-Clean & History Control</h3>
            <p>
              ClipContex automatically trims old history based on your settings
              — completely customizable.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">8.</div>
          <div class="feature-content">
            <h3>Privacy-First Design</h3>
            <p>
              Everything is stored locally. No telemetry, no cloud, no
              analytics. Password manager clips (Bitwarden, 1Password) are
              automatically ignored.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">9.</div>
          <div class="feature-content">
            <h3>Optimized for Linux</h3>
            <p>
              Built using Rust + Tauri with jemalloc memory optimization and
              active window caching for ultra-low CPU usage.
            </p>
          </div>
        </li>

        <li class="feature-item">
          <div class="feature-icon">10.</div>
          <div class="feature-content">
            <h3>System Tray Access</h3>
            <p>Open ClipContex anytime via the tray menu.</p>
          </div>
        </li>
      </ul>
    </div>

    <div class="onboarding-actions">
      <button class="primary-btn" onclick={finishOnboarding}>
        Start Using ClipContex
      </button>
    </div>
  </div>
</div>

<style>
  .onboarding {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    background: var(--bg-secondary);
    min-height: 100vh;
  }

  .onboarding-container {
    max-width: 800px;
    width: 100%;
    background: var(--bg-primary);
    border-radius: var(--radius-lg);
    padding: 32px;
    box-shadow: var(--shadow-md);
    border: 1px solid var(--border-color);
  }

  .onboarding-header {
    text-align: center;
    margin-bottom: 28px;
  }

  .onboarding-title {
    margin: 0 0 10px 0;
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.3px;
  }

  .onboarding-subtitle {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.8rem;
    max-width: 580px;
    margin-left: auto;
    margin-right: auto;
    line-height: 1.6;
  }

  .features-section {
    margin-bottom: 28px;
  }

  .features-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .feature-item {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    margin-bottom: 16px;
    padding: 14px;
    border-radius: var(--radius-md);
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
  }

  .feature-icon {
    font-size: 0.8rem;
    color: var(--text-primary);
    flex-shrink: 0;
    margin-top: 4px;
    opacity: 0.9;
  }

  .feature-content h3 {
    margin: 0 0 4px 0;
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .feature-content p {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-secondary);
    line-height: 1.45;
  }

  kbd {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-family: monospace;
    font-size: 0.75rem;
    border: 1px solid var(--border-color);
  }

  .onboarding-actions {
    text-align: center;
    margin-top: 14px;
  }

  .primary-btn {
    background: var(--action-primary);
    color: white;
    border: none;
    padding: 10px 24px;
    border-radius: var(--radius-md);
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    box-shadow: var(--shadow-sm);
  }

  .primary-btn:hover {
    background: var(--action-primary-hover);
    box-shadow: 0 4px 14px rgba(0, 112, 243, 0.2);
  }

  @media (max-width: 640px) {
    .onboarding-container {
      padding: 24px;
    }

    .onboarding-title {
      font-size: 1.4rem;
    }

    .feature-item {
      padding: 12px;
    }
  }
</style>
