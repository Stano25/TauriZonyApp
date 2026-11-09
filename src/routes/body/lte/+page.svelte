<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FolderPicker from '$lib/components/FolderPicker.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import { appSettings } from '$lib/settings.svelte';
  import { runBodyGeneration } from '$lib/body-generator';

  let filterPath = $state("");
  let multiplePaths = $state<string[]>([]);
  let outputPath = $state("");
  let protocolPointsPath = $state("");
  let secondOutputPath = $state("");
  
  let generateMissingOperators = $state(false);
  let useFilter = $state(false);
  let createDuplicates = $state(false);
  let useProtocolPoints = $state(false);

  let maxDistance = $state("1.5");
  let thresholdRsrp = $state("-20.0");
  let thresholdSinr = $state("-20.0");

  async function addFiles() {
    const selected = await open({
      multiple: true,
      filters: [{ name: 'CSV Súbory', extensions: ['csv'] }]
    });
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      const newPaths = paths.filter(p => !multiplePaths.includes(p));
      multiplePaths = [...multiplePaths, ...newPaths];
    }
  }

  function removeFile(index: number) {
    multiplePaths = multiplePaths.filter((_, i) => i !== index);
  }

  async function handleGenerate() {
    await runBodyGeneration({
      dataType: 'lte',
      inputPaths: multiplePaths.map((p) => p.trim()).filter((p) => p.length > 0),
      outputPath,
      filterPath,
      protocolPath: protocolPointsPath,
      secondOutputPath,
      useFilter,
      maxDistance,
      generateMissingOperators,
      thresholdSinr,
      thresholdRsrp,
      createDuplicates,
      useProtocolPoints,
    });
  }
</script>

<div class="page-container">
  <header class="page-header">
    <h1>LTE Body</h1>
  </header>

  <div class="page-body">
    <section class="card">
      <h2 class="card-title">Vstupné a výstupné súbory</h2>
      <FolderPicker 
        label="Priečinok filtrov operátorov:"
        bind:folderPath={filterPath} 
      />

      <div class="files-box">
        <span class="files-box-title">LTE súbory:</span>
        <div class="files-list">
          {#each multiplePaths as path, i}
            <div class="file-item">
              <span>{path}</span>
              <button class="btn btn-danger" onclick={() => removeFile(i)}>X</button>
            </div>
          {/each}
        </div>
        <button class="add-btn" onclick={addFiles}>+ Pridať súbor</button>
      </div>

      <FileSaver 
        label="Uložiť súbor ako:"
        filterName="CSV súbory"
        extensions={['csv']}
        bind:savePath={outputPath} 
      />

      <FilePicker 
        label="Súbor protokol:"
        filterName="XLSX Súbory"
        extensions={['xlsx']}
        bind:filePath={protocolPointsPath} 
      />

      <FileSaver 
        label="Uložiť protokol ako:"
        filterName="Excel súbory"
        extensions={['xlsx']}
        bind:savePath={secondOutputPath} 
      />
    </section>

    <section class="card">
      <h2 class="card-title">Možnosti</h2>
      <Toggle label="Vygenerovať chýbajúcich operátorov:" bind:checked={generateMissingOperators} />
      <Toggle label="Použiť filtrovanie:" bind:checked={useFilter} />
      <Toggle label="Vytvoriť duplikáty pri filtrovaní:" bind:checked={createDuplicates} />
      <Toggle label="Vygenerovať protokol:" bind:checked={useProtocolPoints} />
    </section>

    <section class="card">
      <h2 class="card-title">Parametre</h2>
      <TextInput label="Veľkosť bodu (polomer v metoch):" bind:value={maxDistance} />
      <div class="input-row-multi">
        <TextInput label="Minimálny RSRP (dB):" bind:value={thresholdRsrp} />
        <TextInput label="Minimálny SINR (dB):" bind:value={thresholdSinr} />
      </div>
    </section>

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'Generuje sa...' : 'Generovať'}
    </button>
  </div>
</div>
