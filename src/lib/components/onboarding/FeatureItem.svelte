<script lang="ts">
  export let index: number;
  export let heading: string;
  export let description: string;
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
    display: grid;
    grid-template-columns: 32px 1fr;
    gap: 12px;

    padding: 16px;

    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);

    box-sizing: border-box;
  }

  .feature-icon {
    width: 32px;
    height: 32px;

    display: flex;
    align-items: center;
    justify-content: center;

    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);

    color: var(--action-primary);
    background: var(--bg-accent);
    border-radius: 50%;
  }

  .feature-content h3 {
    margin: 0 0 4px;
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--text-primary);
  }

  .feature-content p {
    margin: 0;
    font-size: var(--font-size-sm);
    line-height: 1.5;
    color: var(--text-secondary);
  }
</style>
