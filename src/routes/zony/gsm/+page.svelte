<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import { appSettings } from '$lib/settings.svelte';
  import { runZonyGeneration } from '$lib/zony-generator';

  async function handleGenerate() {
    await runZonyGeneration({
      dataType: 'gsm',
      inputPath: appSettings.zony_input_path,
      outputPath: appSettings.zony_output_path,
      onlyBestOperator: appSettings.zony_only_best_operator,
      minBestOperatorSinr: appSettings.zony_min_best_sinr,
    });
  }
</script>

<div class="page-container">
  <header class="page-header">
    <div>
      <h1>GSM Zóny</h1>
      <p class="page-subtitle">Generovanie zón 100x100m z nameraných GSM dát.</p>
    </div>
    <span class="page-badge">GSM</span>
  </header>

  <div class="page-body">
    <section class="card">
      <h2 class="card-title">Vstupné a výstupné súbory</h2>
      <FilePicker 
        label="Súbor definovaných zón (100x100m):"
        filterName="CSV Súbory"
        extensions={['csv']}
        bind:filePath={appSettings.zone_file_path} 
      />

      <FilePicker 
        label="GSM súbor:"
        filterName="CSV Súbory"
        extensions={['csv']}
        bind:filePath={appSettings.zony_input_path} 
      />

      <FileSaver 
        label="Uložiť súbor ako:"
        filterName="CSV súbory"
        extensions={['csv']}
        bind:savePath={appSettings.zony_output_path} 
      />
    </section>

    <section class="card">
      <h2 class="card-title">Možnosti</h2>
      <Toggle 
        label="Iba najlepší operátor:"
        help="Pre každý štvorec ponechá len operátora s najväčšou silou signálu."
        bind:checked={appSettings.zony_only_best_operator} 
      />

      <Toggle 
        label="Vygenerovať chýbajúcich operátorov:" 
        help="Nie je dostupné pri zapnutom režime „Iba najlepší operátor“."
        disabled={appSettings.zony_only_best_operator}
        bind:checked={appSettings.generate_missing_operators} 
      />
    </section>

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'Generuje sa...' : 'Generovať'}
    </button>
  </div>
</div>
