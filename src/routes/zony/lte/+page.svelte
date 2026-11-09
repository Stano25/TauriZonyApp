<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import FolderPicker from '$lib/components/FolderPicker.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import { appSettings } from '$lib/settings.svelte';
  import { runZonyGeneration } from '$lib/zony-generator';

  async function handleGenerate() {
    await runZonyGeneration({
      dataType: 'lte',
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
      <h1>LTE Zóny</h1>
      <p class="page-subtitle">Generovanie zón 100x100m z nameraných LTE dát.</p>
    </div>
    <span class="page-badge">LTE</span>
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
      
      <FolderPicker 
        label="Priečinok filtrov operátorov:"
        bind:folderPath={appSettings.filter_path} 
      />

      <FilePicker 
        label="LTE súbor:"
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
        help="Pre každý štvorec ponechá len jedného operátora. Ak najlepší (podľa RSRP) nemá dosiahnutý minimálny SINR, vyberie sa operátor s najlepším RSRP, ktorý SINR prah spĺňa."
        bind:checked={appSettings.zony_only_best_operator} 
      />

      {#if appSettings.zony_only_best_operator}
        <div class="threshold-box">
          <TextInput 
            label="Minimálny SINR pre najlepšieho operátora (dB):" 
            bind:value={appSettings.zony_min_best_sinr} 
          />
          <p class="threshold-hint">Ak najsilnejší operátor nemá tento SINR, použije sa vyhovujúci operátor s najlepším RSRP.</p>
        </div>
      {/if}

      <Toggle 
        label="Vygenerovať chýbajúcich operátorov:" 
        help="Nie je dostupné pri zapnutom režime „Iba najlepší operátor“."
        disabled={appSettings.zony_only_best_operator}
        bind:checked={appSettings.generate_missing_operators} 
      />

      <Toggle 
        label="Použiť filtrovanie:" 
        bind:checked={appSettings.use_filter} 
      />

      <Toggle 
        label="Vytvoriť duplikáty pri filtrovaní:" 
        bind:checked={appSettings.create_duplicates} 
      />
    </section>

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'Generuje sa...' : 'Generovať'}
    </button>
  </div>
</div>
