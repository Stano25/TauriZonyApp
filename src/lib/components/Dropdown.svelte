<script lang="ts">
  let {
    label,
    options = [],
    value = $bindable("")
  } = $props<{
    label: string;
    options: { value: string, label: string }[];
    value?: string;
  }>();
</script>

<div class="dropdown-group">
  <span class="label-text">{label}</span>
  <select class="styled-select" bind:value={value}>
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>

<style>
  .dropdown-group {
    margin-bottom: 16px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .label-text {
    font-size: 14px;
    font-weight: 500;
    color: var(--gray-text);
  }

  .styled-select {
    padding: 8px 12px;
    background-color: var(--input-bg);
    border: 1px solid var(--input-border);
    color: var(--gray-text);
    border-radius: 4px;
    font-size: 14px;
    outline: none;
    cursor: pointer;
    width: 250px;
    appearance: none;
  }

  .styled-select:focus {
    border-color: var(--accent-outline);
  }
</style>
