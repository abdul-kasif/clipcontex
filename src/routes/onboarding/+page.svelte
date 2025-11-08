<script>
  import { theme } from "$lib/stores/theme";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  const appWindow = getCurrentWebviewWindow();

  async function finishOnboarding() {
    try {
      await invoke("complete_onboarding");
    } catch (e) {
      console.error("Failed to complete onboarding:", e);
    }
  }
</script>

<div class="onboarding">
  <div class="onboarding-container">
    <div class="onboarding-header">
      <div class="logo-placeholder">📋</div>
      <h1 class="onboarding-title">Welcome to ClipContex</h1>
      <p class="onboarding-subtitle">You're all set up! Here's what you can do:</p>
    </div>

    <div class="features-section">
      <ul class="features-list">
        <li class="feature-item">
          <div class="feature-icon">1.</div>
          <div class="feature-content">
            <h3>Automatic clipboard capture</h3>
            <p>Every copied text is automatically saved to your history</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">2.</div>
          <div class="feature-content">
            <h3>Quick Picker</h3>
            <p>Use <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd> to open the quick picker</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">3.</div>
          <div class="feature-content">
            <h3>System tray access</h3>
            <p>Access the main window from the system tray icon</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">4.</div>
          <div class="feature-content">
            <h3>Pin favorite clips</h3>
            <p>Pin important clips for quick access</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">5.</div>
          <div class="feature-content">
            <h3>Powerful search</h3>
            <p>Find any clip with smart search and filtering</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">6.</div>
          <div class="feature-content">
            <h3>Theme switching</h3>
            <p>Switch between light and dark themes</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">7.</div>
          <div class="feature-content">
            <h3>Smart tagging</h3>
            <p>Auto-tagged clips help you find content instantly</p>
          </div>
        </li>
        <li class="feature-item">
          <div class="feature-icon">8.</div>
          <div class="feature-content">
            <h3>Customizable settings</h3>
            <p>Adjust auto-clean days, history size, and privacy options</p>
          </div>
        </li>
      </ul>
    </div>

    <div class="onboarding-actions">
      <button class="primary-btn" onclick={finishOnboarding}>
        Get Started
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
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
    margin-bottom: 24px;
  }

  .logo-placeholder {
    width: 64px;
    height: 64px;
    margin: 0 auto 16px;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--action-primary);
    color: white;
    border-radius: var(--radius-md);
  }

  .onboarding-title {
    margin: 0 0 8px 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.5px;
  }

  .onboarding-subtitle {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.95rem;
    line-height: 1.5;
  }

  .features-section {
    margin-bottom: 24px;
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
    padding: 12px;
    border-radius: var(--radius-md);
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
  }

  .feature-item:last-child {
    margin-bottom: 0;
  }

  .feature-icon {
    font-size: 0.85rem;
    color: var(--text-primary);
    flex-shrink: 0;
    margin-top: 5px;
  }

  .feature-content h3 {
    margin: 0 0 4px 0;
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .feature-content p {
    margin: 0;
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  kbd {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-family: monospace;
    font-size: 0.75rem;
    font-weight: 600;
    border: 1px solid var(--border-color);
  }

  .onboarding-actions {
    text-align: center;
    margin-top: 8px;
  }

  .primary-btn {
    background: var(--action-primary);
    color: white;
    border: none;
    padding: 10px 24px;
    border-radius: var(--radius-md);
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: none;
    box-shadow: var(--shadow-sm);
  }

  .primary-btn:hover {
    background: var(--action-primary-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 112, 243, 0.2);
  }

  .primary-btn:active {
    transform: translateY(0);
  }

  @media (max-width: 640px) {
    .onboarding-container {
      padding: 24px;
      margin: 16px;
    }

    .onboarding-title {
      font-size: 1.3rem;
    }

    .feature-item {
      padding: 10px;
    }
  }
</style>