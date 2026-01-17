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
      A smart, lightweight clipboard manager that remembers
      <strong>what</strong> you copied and <strong>why</strong> you copied it — automatically.
    </p>
  </header>

  <!-- Philosophy -->
  <section class="info-section">
    <h2 class="section-title">Why ClipContex Exists</h2>

    <ul class="list">
      <li>
        <strong>Local-first & private</strong> — nothing leaves your device.
      </li>
      <li>
        <strong>Context-aware</strong> — understands apps, windows & projects.
      </li>
      <li>
        <strong>Lightweight</strong> — faster and leaner than Electron apps.
      </li>
      <li>
        <strong>Automatic organization</strong> — tags, dedupe & timelines.
      </li>
    </ul>
  </section>

  <!-- Features -->
  <section class="info-section">
    <h2 class="section-title">Features</h2>

    <ul class="list">
      <li>Real-time clipboard capture (300ms debounce)</li>
      <li>Smart deduplication</li>
      <li>Context detection (app, window, project)</li>
      <li>Auto-tags: <em>#code, #url, #terminal, #project</em></li>
      <li>Pin clips, timeline view & fast search</li>
      <li>
        Quick Picker:
        <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd>
      </li>
      <li>Auto-clean system based on your settings</li>
      <li>Ignores password managers automatically</li>
      <li>Optimized memory usage on Linux</li>
      <li>Tauri-based native performance</li>
    </ul>
  </section>

  <!-- Technology -->
  <section class="info-section">
    <h2 class="section-title">Technology</h2>

    <div class="tech-list">
      <div><strong>Frontend</strong><span>SvelteKit</span></div>
      <div><strong>Backend</strong><span>Rust</span></div>
      <div><strong>Framework</strong><span>Tauri</span></div>
      <div><strong>Database</strong><span>SQLite</span></div>
      <div><strong>Clipboard</strong><span>Tauri Plugin</span></div>
    </div>
  </section>

  <!-- Privacy -->
  <section class="info-section">
    <h2 class="section-title">Privacy</h2>

    <ul class="list">
      <li>100% offline — no internet required</li>
      <li>No telemetry, analytics, or tracking</li>
      <li>All data stored locally</li>
      <li>Clear history anytime</li>
    </ul>
  </section>

  <!-- Roadmap -->
  <section class="info-section">
    <h2 class="section-title">Roadmap</h2>

    <ul class="list">
      <li><strong>v2</strong> — Snippets, collections, manual tags</li>
      <li><strong>v3</strong> — Plugins, rules, export/import</li>
    </ul>
  </section>

  <!-- Contribution -->
  <section class="info-section">
    <h2 class="section-title">Contribute</h2>

    <p class="paragraph">
      Found a bug or have an idea? Your feedback helps improve ClipContex.
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
          Issue Tracker
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
      Built with ❤️ using Rust, Svelte & Tauri.
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

  .tech-list {
    display: grid;
    gap: 8px;
  }

  .tech-list > div,
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
