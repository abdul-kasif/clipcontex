<script lang="ts">
    export let type: 'button' | 'submit' | 'reset' = 'button';
    export let variant: 'primary' | 'secondary' | 'danger' | 'ghost' = 'primary'
    export let disabled: boolean = false;
</script>

<button
    type={type}
    {disabled}
    class="btn btn-{variant}"
    on:click
>
<slot/>
</button>

<style>
  .btn {
    padding: 10px 20px;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    font-family: var(--font-primary);
    font-weight: var(--font-weight-semibold);
    font-size: var(--font-size-md);
    transition: background-color 0.2s ease;
  }
  .btn-primary {
    background-color: var(--action-primary);
    color: white;
  }

  .btn-primary:hover {
    background-color: var(--action-primary-hover);
  }
  .btn-secondary {
    background-color: #6c757d;
    color: white;
  }
  .btn-secondary:hover {
    background-color: #5a6268;
  }
  .btn-danger {
    background-color: #dc3545;
    color: white;
  }
  .btn-danger:hover {
    background-color: #c82333;
  }
  button:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
