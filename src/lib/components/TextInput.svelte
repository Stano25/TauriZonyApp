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

<div class="field">
  <label>{label}</label>
  <input class="input" type="text" bind:value={value} {placeholder} />
</div>
