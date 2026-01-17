<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion, getName } from "@tauri-apps/api/app";
  import { platform } from "@tauri-apps/plugin-os";

  let appName = "ClipContex";
  let appVersion = "";
  let platformName = "";

  onMount(async () => {
    appName = await getName();
    appVersion = await getVersion();

    const p = platform();
    platformName =
      p === "linux" ? "Linux" : p === "macos" ? "macOS" : "Windows";
  });
</script>

<div class="about-settings">
  <!-- App Header -->
  <header class="about-header">
    <img
      src="/Square89x89Logo.png"
      alt=""
      class="app-logo"
      width="72"
      height="72"
    />

    <h1 class="app-name">{appName}</h1>

    <p class="app-description">
      A smart clipboard that remembers not just <strong>what</strong> you
      copied, but also <strong>where</strong> and <strong>why</strong> — so you never
      lose context again.
    </p>
  </header>

  <!-- Philosophy -->
  <section class="info-section">
    <h2 class="section-title">Why We Built ClipContex</h2>

    <ul class="list">
      <li>
        <strong>Private by design</strong> — your clipboard never leaves your computer.
      </li>
      <li>
        <strong>Smart & helpful</strong> — knows which app you’re in and what you’re
        working on.
      </li>
      <li>
        <strong>Light and fast</strong> — uses minimal memory and starts instantly.
      </li>
      <li>
        <strong>Organized automatically</strong> — no manual sorting needed.
      </li>
    </ul>
  </section>

  <!-- Features -->
  <section class="info-section">
    <h2 class="section-title">What It Does</h2>

    <ul class="list">
      <li>Saves everything you copy — text only, for privacy</li>
      <li>Skips duplicates and password managers automatically</li>
      <li>Shows which app and window each clip came from</li>
      <li>Tags clips automatically: <em>#code, #url, #email, #terminal</em></li>
      <li>Pin important clips to keep them forever</li>
      <li>
        Quickly search or paste with <kbd>Ctrl</kbd> + <kbd>Shift</kbd> +
        <kbd>V</kbd>
      </li>
      <li>Cleans up old clips based on your preferences</li>
      <li>Works great on Linux and Windows</li>
    </ul>
  </section>

  <!-- Privacy -->
  <section class="info-section">
    <h2 class="section-title">Your Privacy Matters</h2>

    <ul class="list">
      <li>✅ 100% offline — no internet connection needed</li>
      <li>✅ No tracking, analytics, or hidden data collection</li>
      <li>✅ All data stays on your device — you own it</li>
      <li>✅ One-click history clear anytime</li>
    </ul>
  </section>

  <!-- Roadmap -->
  <section class="info-section">
    <h2 class="section-title">What’s Next</h2>

    <ul class="list">
      <li>
        <strong>Coming soon</strong> — Snippets, custom tags, and collections
      </li>
      <li><strong>Later</strong> — Rules, export/import, and plugin support</li>
    </ul>
  </section>

  <!-- Contribution -->
  <section class="info-section">
    <h2 class="section-title">Help Make It Better</h2>

    <p class="paragraph">
      Love ClipContex? Found a bug? Have an idea? We’d love to hear from you!
    </p>

    <ul class="links">
      <li>
        <a
          href="https://github.com/abdul-kasif/clipcontex"
          target="_blank"
          rel="noopener noreferrer"
        >
          GitHub Repository
        </a>
      </li>
      <li>
        <a
          href="https://github.com/abdul-kasif/clipcontex/issues"
          target="_blank"
          rel="noopener noreferrer"
        >
          Report an Issue or Suggest a Feature
        </a>
      </li>
    </ul>
  </section>

  <!-- Meta -->
  <section class="info-section meta">
    <h2 class="section-title">App Info</h2>

    <div class="meta-grid">
      <div>
        <span>Version</span>
        <strong>{appVersion}</strong>
      </div>
      <div>
        <span>Platform</span>
        <strong>{platformName}</strong>
      </div>
    </div>
  </section>

  <!-- Footer -->
  <footer class="about-footer">
    <p>
      © {new Date().getFullYear()}
      {appName}<br />
      Made with ❤️ for developers and power users.
    </p>
  </footer>
</div>

<style>
  /* ===========================
     Layout
  ============================ */

  .about-settings {
    max-width: 720px;
    margin: 0 auto;
    padding: 8px 16px;
    color: var(--text-primary);
  }

  /* ===========================
     Header
  ============================ */

  .about-header {
    text-align: center;
    padding: 24px 0;
  }

  .app-logo {
    margin-bottom: 12px;
  }

  .app-name {
    margin: 0 0 6px;
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
  }

  .app-description {
    max-width: 520px;
    margin: 0 auto;
    color: var(--text-secondary);
    font-size: var(--font-size-md);
    line-height: 1.5;
  }

  /* ===========================
     Sections
  ============================ */

  .info-section {
    margin-bottom: 20px;
    padding: 16px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
  }

  .section-title {
    margin: 0 0 12px;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    letter-spacing: 0.4px;
  }

  /* ===========================
     Text & Lists
  ============================ */

  .paragraph {
    margin: 0 0 12px;
    color: var(--text-secondary);
    font-size: var(--font-size-md);
    line-height: 1.6;
  }

  .list {
    list-style: none;
    padding: 0;
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-md);
    line-height: 1.6;
  }

  .list li {
    padding: 4px 0;
  }

  /* ===========================
     Tech & Meta
  ============================ */

  .meta-grid > div {
    display: flex;
    justify-content: space-between;
  }

  .meta {
    background: var(--bg-tertiary);
  }

  .meta-grid {
    display: grid;
    gap: 12px;
  }

  /* ===========================
     Links
  ============================ */

  .links {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .links a {
    color: var(--action-primary);
    text-decoration: none;
  }

  .links a:hover {
    text-decoration: underline;
  }

  .links a:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  /* ===========================
     Footer
  ============================ */

  .about-footer {
    text-align: center;
    padding: 16px 0;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  /* ===========================
     Keyboard Keys
  ============================ */

  kbd {
    background: var(--bg-tertiary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-family: monospace;
    font-size: 0.85em;
    border: 1px solid var(--border-color-light);
  }
</style>
