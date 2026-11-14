<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import FolderPicker from '$lib/components/FolderPicker.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import Dropdown from '$lib/components/Dropdown.svelte';
  import { fivegRsrpOptions, fivegSinrOptions } from '$lib/fiveg-modes';
  import { appSettings } from '$lib/settings.svelte';
  import { runZonyGeneration } from '$lib/zony-generator';

  async function handleGenerate() {
    await runZonyGeneration({
      dataType: '5g',
      inputPath: appSettings.zony_input_path,
      outputPath: appSettings.zony_output_path,
      rsrpMode: appSettings.fiveg_rsrp,
      sinrMode: appSettings.fiveg_sinr,
      onlyBestOperator: appSettings.zony_only_best_operator,
      minBestOperatorSinr: appSettings.zony_min_best_sinr,
      rsrpOffset: appSettings.zony_rsrp_offset,
      sinrOffset: appSettings.zony_sinr_offset,
    });
  }
</script>

<div class="page-container">
  <header class="page-header">
    <h1>5G Zóny</h1>
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
        label="5G súbor:"
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
      <h2 class="card-title">5G stĺpce</h2>
      <Dropdown label="RSRP Stĺpec:" options={fivegRsrpOptions} bind:value={appSettings.fiveg_rsrp} />
      <Dropdown label="SINR Stĺpec:" options={fivegSinrOptions} bind:value={appSettings.fiveg_sinr} />
    </section>

    <section class="card">
      <h2 class="card-title">Možnosti</h2>
      <Toggle 
        label={`Najlepší operátor podľa ${appSettings.fiveg_rsrp}:`} 
        bind:checked={appSettings.zony_only_best_operator} 
      />

      {#if appSettings.zony_only_best_operator}
        <div class="threshold-box">
          <TextInput 
            label={`Minimálny ${appSettings.fiveg_sinr} pre najlepšieho operátora (dB):`} 
            bind:value={appSettings.zony_min_best_sinr} 
          />
        </div>
      {/if}

      <div class="threshold-box">
        <div class="input-row-multi">
          <TextInput 
            label={`Pripočítaná hodnota k ${appSettings.fiveg_rsrp} priemeru (dB):`} 
            bind:value={appSettings.zony_rsrp_offset} 
          />
          <TextInput 
            label={`Pripočítaná hodnota k ${appSettings.fiveg_sinr} priemeru (dB):`} 
            bind:value={appSettings.zony_sinr_offset} 
          />
        </div>
      </div>

      <Toggle 
        label="Vygenerovať chýbajúcich operátorov:" 
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
