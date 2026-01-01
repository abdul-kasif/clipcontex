<script lang="ts">
  export let index: number = 1;
  export let heading: String;
  export let description: String;
</script>

<li class="feature-item">
  <div class="feature-icon">{index}</div>
  <div class="feature-content">
    <h3>{heading}</h3>
    <p>{description}</p>
  </div>
</li>

<style>
  .feature-item {
    margin: 0 0 16px 0;
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
  }

  .feature-icon {
    margin: 2px 0 0 0;
    font-size: var(--font-size-md);
    color: var(--text-primary);
  }

  .feature-content h3 {
    margin: 2px 0 2px 0;
    color: var(--text-primary);
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
  }

  .feature-content p {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }
</style>
