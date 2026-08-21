<script lang="ts">
  let { 
    label, 
    placeholder = "",
    value = $bindable("")
  } = $props<{
    label: string;
    placeholder?: string;
    value?: string;
  }>();
</script>

<div class="text-input-group">
  <label>{label}</label>
  <input type="text" bind:value={value} {placeholder} />
</div>

<style>
  .text-input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 600px;
    margin-bottom: 16px;
  }

  .text-input-group label {
    font-size: 14px;
    font-weight: 500;
    color: var(--gray-text);
  }

  .text-input-group input {
    padding: 8px 12px;
    background-color: var(--input-bg);
    color: var(--gray-text);
    border: 1px solid var(--input-border);
    border-radius: 4px;
    font-size: 14px;
  }

  .text-input-group input:focus {
    outline: 1px solid var(--accent-outline);
    border-color: var(--accent-outline);
  }
</style>