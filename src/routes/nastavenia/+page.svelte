<script lang="ts">
  import Toggle from '$lib/components/Toggle.svelte';
  import { appSettings } from '$lib/settings.svelte';
</script>

<div class="page-container">
  <header class="page-header">
    <h1>Nastavenia</h1>
  </header>

  <div class="page-body">
    <section class="card">
      <h2 class="card-title">Vzhľad</h2>
      <Toggle 
        label="Tmavý režim:" 
        bind:checked={appSettings.is_dark_mode} 
      />
    </section>
  </div>
</div>
