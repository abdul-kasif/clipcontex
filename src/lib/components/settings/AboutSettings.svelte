<script>
  import { onMount } from 'svelte';
  import { getVersion, getName } from '@tauri-apps/api/app';

  let appName = '';
  let appVersion = '';

  onMount(async () => {
    appName = await getName();
    appVersion = await getVersion();
  });
</script>

<div class="about-settings">
  <div class="app-info">
    <div class="app-logo">
      <div class="logo-placeholder">📋</div>
    </div>
    <h1 class="app-name">{appName || 'clipcontex'}</h1>
    <p class="app-description">
      Your clipboard should remember <strong>what you were doing</strong>, not just <strong>what you copied</strong>.
    </p>
  </div>

  <section class="info-section">
    <h3 class="section-title">Core Philosophy</h3>
    <ul class="philosophy-list">
      <li><strong>Local-first</strong>: All data stays on your machine. No cloud, no tracking.</li>
      <li><strong>Context-aware</strong>: Knows which app, project, and task you’re in.</li>
      <li><strong>Developer-friendly</strong>: Built for coders, but useful for everyone.</li>
      <li><strong>Lightweight</strong>: Uses ~50MB RAM — no Electron bloat.</li>
    </ul>
  </section>

  <section class="info-section">
    <h3 class="section-title">v1 Features</h3>
    <ul class="features-list">
      <li>Auto-save clipboard with 300ms debounce & 10s deduplication</li>
      <li>Detect source app (VS Code, Terminal, Firefox, etc.)</li>
      <li>Extract project context from window titles</li>
      <li>Auto-tags: #code, #url, #email, #terminal, #project-name</li>
      <li>Pin clips, timeline view, fuzzy search</li>
      <li>Quick Picker: <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>V</kbd></li>
      <li>System tray, dark mode, auto-clean</li>
      <li>Privacy: skips clipboard from Bitwarden, 1Password</li>
    </ul>
  </section>

  <section class="info-section">
    <h3 class="section-title">Technical Stack</h3>
    <div class="tech-grid">
      <div class="tech-item"><strong>Frontend</strong>: Svelte + Vite</div>
      <div class="tech-item"><strong>Backend</strong>: Rust</div>
      <div class="tech-item"><strong>Framework</strong>: Tauri</div>
      <div class="tech-item"><strong>Storage</strong>: SQLite (~/.clipcontex/)</div>
      <div class="tech-item"><strong>Clipboard</strong>: clipboard manager (tauri) + xprop (Linux)</div>
    </div>
  </section>

  <section class="info-section">
    <h3 class="section-title">Privacy & Control</h3>
    <ul class="privacy-list">
      <li>✅ Zero internet permission required</li>
      <li>✅ No telemetry, no analytics</li>
      <li>✅ One-click clear history</li>
      <li>✅ Portable: runs from USB, no installer needed (version 2)</li>
    </ul>
  </section>

  <section class="info-section">
    <h3 class="section-title">Roadmap</h3>
    <ul class="roadmap-list">
      <li><strong>version 2</strong>: Snippets with placeholders, manual collections, edit clips</li>
      <li><strong>version 3</strong>: Plugin system, custom auto-tagging rules, export/import</li>
    </ul>
  </section>

  <section class="info-section">
    <h3 class="section-title">Support & Contribute</h3>
    <div class="support-info">
      <p class="support-text">Found a bug? Have a feature idea? We’d love your help!</p>
      <ul class="support-links">
        <li>
          <a href="https://github.com/abdul-kasif/clipcontex" target="_blank" class="support-link">
            📄 GitHub Repository
          </a>
        </li>
        <li>
          <a href="https://github.com/abdul-kasif/clipcontex/issues" target="_blank" class="support-link">
            🐞 Issue Tracker
          </a>
        </li>
      </ul>
    </div>
  </section>

  <section class="info-section app-meta">
    <div class="info-grid">
      <div class="info-item">
        <label for="version" class="info-label">Version</label>
        <span class="info-value">{appVersion || '1.0.0'}</span>
      </div>
      <div class="info-item">
        <label for="platform" class="info-label">Platform</label>
        <span class="info-value">Linux (Fedora KDE, Fedora Workstation 42)</span>
      </div>
    </div>
  </section>

  <footer class="app-footer">
    <p>© {new Date().getFullYear()} {appName || 'clipcontex'}. All rights reserved.<br/>
       Built with ❤️ using Tauri + Svelte + Rust.</p>
  </footer>
</div>

<style>
  .about-settings {
    max-width: 680px;
    margin: 0 auto;
    padding: 0 16px;
  }

  .app-info {
    text-align: center;
    margin-bottom: 24px;
    padding: 24px 0;
  }

  .logo-placeholder {
    width: 64px;
    height: 64px;
    margin: 0 auto 12px;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #3b82f6;
    color: white;
    border-radius: 8px;
  }

  .app-name {
    margin: 0 0 8px 0;
    color: #374151;
    font-size: 1.5rem;
    font-weight: 700;
  }

  .app-description {
    color: #6b7280;
    font-size: 0.95rem;
    line-height: 1.5;
    margin: 0;
    max-width: 500px;
    margin-left: auto;
    margin-right: auto;
  }

  .info-section {
    margin-bottom: 20px;
    padding: 16px;
    background: #f8fafc;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
  }

  .section-title {
    margin: 0 0 12px 0;
    color: #374151;
    font-size: 0.9rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .philosophy-list,
  .features-list,
  .privacy-list,
  .roadmap-list {
    list-style: none;
    padding: 0;
    margin: 0;
    font-size: 0.85rem;
    color: #4b5563;
    line-height: 1.6;
  }

  .philosophy-list li,
  .features-list li,
  .privacy-list li,
  .roadmap-list li {
    padding: 4px 0;
  }

  .tech-grid {
    display: grid;
    gap: 8px;
    font-size: 0.85rem;
    color: #4b5563;
  }

  .tech-item {
    display: flex;
    justify-content: space-between;
  }

  .tech-item strong {
    color: #374151;
  }

  .support-info p {
    margin: 0 0 12px 0;
    color: #4b5563;
    font-size: 0.85rem;
  }

  .support-links {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .support-links li {
    padding: 4px 0;
  }

  .support-link {
    color: #3b82f6;
    text-decoration: none;
    font-size: 0.85rem;
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }

  .support-link:hover {
    text-decoration: underline;
  }

  .info-grid {
    display: grid;
    gap: 12px;
  }

  .info-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 0;
    border-bottom: 1px solid #e2e8f0;
  }

  .info-item:last-child {
    border-bottom: none;
  }

  .info-label {
    font-weight: 500;
    color: #4b5563;
    font-size: 0.85rem;
  }

  .info-value {
    color: #6b7280;
    font-size: 0.85rem;
  }

  .app-meta {
    background: #f1f5f9;
  }

  .app-footer {
    text-align: center;
    padding-top: 16px;
    margin-top: 16px;
    border-top: 1px solid #e2e8f0;
    color: #9ca3af;
    font-size: 0.75rem;
    line-height: 1.5;
  }

  kbd {
    background: #f1f5f9;
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.8em;
    color: #4b5563;
  }
</style>