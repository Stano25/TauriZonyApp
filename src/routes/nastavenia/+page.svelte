<script lang="ts">
  import Toggle from '$lib/components/Toggle.svelte';
  import { appSettings } from '$lib/settings.svelte';
</script>

<div class="page-container">
  <header class="page-header">
    <h1>Nastavenia</h1>
  </header>

  <div class="page-body">
    <Toggle 
      label="Tmavý režim:" 
      bind:checked={appSettings.is_dark_mode} 
    />
  </div>
</div>

<style>
  .page-container {
    padding: 20px;
    height: 100%;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }
  .page-header {
    border-bottom: 2px solid var(--border-color);
    padding-bottom: 15px;
    margin-bottom: 25px;
  }
  .page-body {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 15px;
  }
</style>