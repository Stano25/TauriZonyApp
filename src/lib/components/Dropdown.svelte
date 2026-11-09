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

<div class="field">
  <span class="input-label">{label}</span>
  <select class="styled-select" bind:value={value}>
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>
</div>
