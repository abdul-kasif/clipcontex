<script>
  import { onMount } from "svelte";
  import { getVersion, getName } from "@tauri-apps/api/app";
  import { platform } from "@tauri-apps/plugin-os";

  let appName = "";
  let appVersion = "";
  let osName = "";

  onMount(async () => {
    appName = await getName();
    appVersion = await getVersion();
    const p = platform();
    osName = p === "linux" ? "Linux" : p === "macos" ? "macOS" : "Windows";
  });
</script>

<div class="about-settings">
  <!-- HEADER -->
  <div class="app-info">
    <div class="app-logo">
      <img src="/Square89x89Logo.png" alt="" width="89" height="89" />
    </div>
    <h1 class="app-name">{appName || "ClipContex"}</h1>
    <p class="app-description">
      A smart, lightweight clipboard that remembers <strong>what</strong> you
      copied and <strong>why</strong> you copied it — automatically.
    </p>
  </div>

  <!-- CORE PHILOSOPHY -->
  <section class="info-section">
    <h2 class="section-title">Why ClipContex Exists</h2>
    <ul class="philosophy-list">
      <li>
        <strong>Local-first & private</strong> — nothing leaves your device.
      </li>
      <li>
        <strong>Context-aware</strong> — detects source app, window title & project.
      </li>
      <li>
        <strong>Lightweight</strong> — ~60MB RAM; faster than Electron apps.
      </li>
      <li>
        <strong>Automatic organization</strong> — smart tags, dedupe & history.
      </li>
    </ul>
  </section>

  <!-- FEATURES -->
  <section class="info-section">
    <h2 class="section-title">Features</h2>
    <ul class="features-list">
      <li>✔ Real-time clipboard saving (300ms debounce)</li>
      <li>✔ Smart deduplication (within 10 seconds)</li>
      <li>✔ Detects app, window title & project context</li>
      <li>✔ Auto-tags: <em>#code, #url, #email, #terminal, #project</em></li>
      <li>✔ Pin clips, timeline view, fuzzy search</li>
      <li>✔ Quick Picker: <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>V</kbd></li>
      <li>✔ Auto-clean system based on your settings</li>
      <li>✔ Ignores password managers (Bitwarden, 1Password)</li>
      <li>✔ Memory-optimized using jemalloc (Linux)</li>
      <li>✔ Tauri-based native performance</li>
    </ul>
  </section>

  <!-- TECH STACK -->
  <section class="info-section">
    <h2 class="section-title">Technology</h2>
    <div class="tech-grid">
      <div class="tech-item"><strong>Frontend</strong>: SvelteKit</div>
      <div class="tech-item"><strong>Backend</strong>: Rust</div>
      <div class="tech-item"><strong>Framework</strong>: Tauri</div>
      <div class="tech-item"><strong>Database</strong>: SQLite</div>
      <div class="tech-item">
        <strong>Clipboard Engine</strong>: Tauri plugin
      </div>
    </div>
  </section>

  <!-- PRIVACY -->
  <section class="info-section">
    <h2 class="section-title">Privacy</h2>
    <ul class="privacy-list">
      <li>100% offline — no internet permission required</li>
      <li>No telemetry, analytics, or tracking</li>
      <li>All data stays locally on your machine</li>
      <li>Clear clipboard history at any time</li>
    </ul>
  </section>

  <!-- ROADMAP -->
  <section class="info-section">
    <h2 class="section-title">Roadmap</h2>
    <ul class="roadmap-list">
      <li><strong>v2</strong>: Snippets, collections, manual tags</li>
      <li>
        <strong>v3</strong>: Plugin system, custom auto-tag rules, export/import
      </li>
    </ul>
  </section>

  <!-- CONTRIBUTION -->
  <section class="info-section">
    <h2 class="section-title">Contribute</h2>
    <p class="support-text">
      Found a bug or have a suggestion? Your feedback makes ClipContex better.
    </p>
    <ul class="support-links">
      <li>
        <a
          href="https://github.com/abdul-kasif/clipcontex"
          target="_blank"
          rel="noopener noreferrer"
          class="support-link"
          aria-label="Visit ClipContex GitHub repository (opens in new tab)"
        >
          GitHub Repository
        </a>
      </li>
      <li>
        <a
          href="https://github.com/abdul-kasif/clipcontex/issues"
          target="_blank"
          rel="noopener noreferrer"
          class="support-link"
          aria-label="Report an issue on GitHub (opens in new tab)"
        >
          Issue Tracker
        </a>
      </li>
    </ul>
  </section>

  <!-- META -->
  <section class="info-section app-meta">
    <h2 class="section-title">App Info</h2>
    <div class="info-grid">
      <div class="info-item">
        <span class="info-label">Version</span>
        <span class="info-value">{appVersion}</span>
      </div>
      <div class="info-item">
        <span class="info-label">Platform</span>
        <span class="info-value">{osName}</span>
      </div>
    </div>
  </section>

  <!-- FOOTER -->
  <footer class="app-footer">
    <p>
      © {new Date().getFullYear()}
      {appName || "ClipContex"}<br />
      Built with ❤️ using Rust + Svelte + Tauri.
    </p>
  </footer>
</div>

<style>
  .about-settings {
    max-width: 680px;
    margin: 0 auto;
    padding: 0 16px;
    color: var(--text-primary);
  }

  /* Header */
  .app-info {
    text-align: center;
    margin-bottom: 24px;
    padding: 24px 0;
  }
  .app-name {
    margin: 0 0 8px 0;
    color: var(--text-primary);
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
  }
  .app-description {
    color: var(--text-secondary);
    font-size: var(--font-size-md);
    line-height: 1.5;
    max-width: 500px;
    margin: 0 auto;
  }

  /* Section layout */
  .info-section {
    margin-bottom: 20px;
    padding: 16px;
    background: var(--bg-secondary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }
  .section-title {
    margin: 0 0 12px 0;
    color: var(--text-primary);
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    letter-spacing: 0.5px;
  }

  /* Lists */
  .philosophy-list,
  .features-list,
  .roadmap-list,
  .privacy-list {
    padding: 0;
    margin: 0;
    font-size: var(--font-size-md);
    color: var(--text-secondary);
    line-height: 1.6;
  }

  .philosophy-list,
  .features-list,
  .roadmap-list {
    list-style: none;
  }

  .privacy-list {
    list-style: disc;
    margin-left: 16px;
  }

  .philosophy-list li,
  .features-list li,
  .roadmap-list li,
  .privacy-list li {
    padding: 4px 0;
  }

  /* Tech grid */
  .tech-grid {
    display: grid;
    gap: 8px;
    font-size: var(--font-size-md);
    color: var(--text-secondary);
  }
  .tech-item {
    display: flex;
    justify-content: space-between;
  }
  .tech-item strong {
    color: var(--text-primary);
  }

  /* Support links */
  .support-text {
    margin: 0 0 12px 0;
    color: var(--text-secondary);
    font-size: var(--font-size-md);
  }
  .support-links {
    list-style: none;
    padding: 0;
  }
  .support-link {
    color: var(--action-primary);
    text-decoration: none;
    font-size: var(--font-size-md);
    display: inline-block;
    padding: 2px 0;
  }
  .support-link:hover {
    text-decoration: underline;
  }
  .support-link:focus-visible {
    outline: 2px solid var(--focus-ring-color);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  /* Meta section */
  .app-meta {
    background: var(--bg-tertiary);
  }
  .info-grid {
    display: grid;
    gap: 12px;
  }
  .info-item {
    display: flex;
    justify-content: space-between;
    padding: 6px 0;
    border-bottom: 1px solid var(--border-color);
  }
  .info-item:last-child {
    border-bottom: none;
  }
  .info-label {
    font-weight: var(--font-weight-semibold);
    font-size: var(--font-size-md);
    color: var(--text-primary);
  }
  .info-value {
    font-size: var(--font-size-md);
    color: var(--text-muted);
  }

  /* Footer */
  .app-footer {
    text-align: center;
    padding-top: 16px;
    border-top: 1px solid var(--border-color);
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    margin-top: 16px;
  }

  /* Keyboard key styling */
  kbd {
    background: var(--bg-tertiary);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-family: monospace;
    font-size: 0.85em;
    color: var(--text-secondary);
    border: 1px solid var(--border-color-light);
  }
</style>
