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
    <h1>Konfigurácia LTE Body Modulu</h1>
  </header>

  <div class="page-body">
    <FolderPicker 
      label="Priečinok filtrov operátorov:"
      bind:folderPath={filterPath} 
    />

    <div class="multiple-files-box">
      <span class="box-title">LTE súbory:</span>
      <div class="files-list">
        {#each multiplePaths as path, i}
          <div class="file-item">
            <span>{path}</span>
            <button class="remove-btn" onclick={() => removeFile(i)}>X</button>
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

    <Toggle label="Vygenerovať chýbajúcich operátorov:" bind:checked={generateMissingOperators} />
    <Toggle label="Použiť filtrovanie:" bind:checked={useFilter} />
    <Toggle label="Vytvoriť duplikáty pri filtrovaní:" bind:checked={createDuplicates} />
    <Toggle label="Vygenerovať protokol:" bind:checked={useProtocolPoints} />

    <TextInput label="Veľkosť bodu (polomer v metoch):" bind:value={maxDistance} />
    <TextInput label="Nastav minimálny RSRP:" bind:value={thresholdRsrp} />
    <TextInput label="Nastav minimálny SINR:" bind:value={thresholdSinr} />

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'GENERUJE SA...' : 'GENEROVAT'}
    </button>
  </div>
</div>

<style>
  .page-container { padding: 20px; height: 100%; box-sizing: border-box; display: flex; flex-direction: column; gap: 24px; overflow-y: auto; }
  .page-header h1 { margin: 0; color: var(--text-color);}
  .page-body { flex: 1; display: flex; flex-direction: column; }
  
  .multiple-files-box { border: 1px solid var(--input-border); border-radius: 4px; padding: 16px; margin-bottom: 24px; max-width: 600px; background-color: var(--btn-content-bg); }
  .box-title { font-size: 16px; font-weight: bold; color: var(--gray-text); display: block; margin-bottom: 12px; }
  .files-list { max-height: 150px; overflow-y: auto; margin-bottom: 12px; display: flex; flex-direction: column; gap: 8px; }
  .file-item { display: flex; justify-content: space-between; align-items: center; background: var(--input-bg); padding: 8px; border-radius: 4px; font-size: 14px; }
  .file-item span { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin-right: 10px; }
  .remove-btn { background: #d13438; color: white; border: none; border-radius: 4px; padding: 4px 8px; cursor: pointer; }
  .remove-btn:hover { background: #e81123; }
  .add-btn { width: 100%; padding: 8px; background: var(--input-bg); border: 1px solid var(--input-border); color: white; border-radius: 4px; cursor: pointer; }
  .add-btn:hover { background: #444444; }

  .generate-btn { padding: 12px 40px; background-color: var(--accent-color); color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: bold; width: fit-content; margin-top: 20px; transition: background-color 0.2s; }
  .generate-btn:hover { background: #1177bb; }
  .generate-btn:disabled { background-color: var(--input-border); cursor: not-allowed; }
</style>