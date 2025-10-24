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
    <h1 class="app-name">{appName || 'Clipboard Manager'}</h1>
    <p class="app-description">
      A powerful clipboard manager application built with Tauri, SvelteKit, and Rust.
    </p>
  </div>

  <section class="info-section">
    <h3 class="section-title">Application Information</h3>
    <div class="info-grid">
      <div class="info-item">
        <label for="version" class="info-label">Version</label>
        <span class="info-value">{appVersion || '1.0.0'}</span>
      </div>
    </div>
  </section>

  <section class="info-section">
    <h3 class="section-title">Features</h3>
    <ul class="features-list">
      <li class="feature-item">📋 Advanced clipboard history tracking</li>
      <li class="feature-item">🔒 Privacy-focused with app filtering</li>
      <li class="feature-item">⚡ Fast and lightweight</li>
      <li class="feature-item">🎨 Dark mode support</li>
      <li class="feature-item">🔄 Auto-cleanup of old entries</li>
    </ul>
  </section>

  <section class="info-section">
    <h3 class="section-title">Support</h3>
    <div class="support-info">
      <p class="support-text">For support, bug reports, or feature requests:</p>
      <ul class="support-links">
        <li><a href="#" target="_blank" class="support-link">Documentation</a></li>
        <li><a href="#" target="_blank" class="support-link">GitHub Repository</a></li>
        <li><a href="#" target="_blank" class="support-link">Issue Tracker</a></li>
      </ul>
    </div>
  </section>

  <footer class="app-footer">
    <p>© {new Date().getFullYear()} {appName || 'Clipboard Manager'}. All rights reserved.</p>
  </footer>
</div>

<style>
  .about-settings {
    max-width: 600px;
    margin: 0 auto;
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
    font-size: 1.4rem;
    font-weight: 600;
  }

  .app-description {
    color: #6b7280;
    font-size: 0.9rem;
    line-height: 1.5;
    margin: 0;
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

  .features-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .feature-item {
    padding: 6px 0;
    color: #4b5563;
    font-size: 0.85rem;
    border-bottom: 1px solid #e2e8f0;
  }

  .feature-item:last-child {
    border-bottom: none;
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
  }

  .support-link:hover {
    text-decoration: underline;
  }

  .app-footer {
    text-align: center;
    padding-top: 16px;
    margin-top: 16px;
    border-top: 1px solid #e2e8f0;
    color: #9ca3af;
    font-size: 0.75rem;
  }
</style>