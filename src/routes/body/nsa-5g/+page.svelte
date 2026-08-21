<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FolderPicker from '$lib/components/FolderPicker.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import Dropdown from '$lib/components/Dropdown.svelte';
  import { fivegRsrpOptions, fivegSinrOptions } from '$lib/fiveg-modes';
  import { appSettings } from '$lib/settings.svelte';
  import { runMobileBodyGeneration } from '$lib/body-generator';

  let filterPath = $state("");
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

  let mobilePaths = $state<{lte_path: string, g5_path: string}[]>([]);

  function addMobilePath() {
    mobilePaths = [...mobilePaths, { lte_path: "", g5_path: "" }];
  }

  function removeMobilePath(index: number) {
    mobilePaths = mobilePaths.filter((_, i) => i !== index);
  }

  async function pickLte(index: number) {
    const selected = await open({ multiple: false, filters: [{ name: 'CSV Súbory', extensions: ['csv'] }]});
    if (selected) { mobilePaths[index].lte_path = selected as string; }
  }

  async function pick5G(index: number) {
    const selected = await open({ multiple: false, filters: [{ name: 'CSV Súbory', extensions: ['csv'] }]});
    if (selected) { mobilePaths[index].g5_path = selected as string; }
  }

  async function handleGenerate() {
    await runMobileBodyGeneration({
      inputEntries: mobilePaths,
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
    <h1>Konfigurácia NSA 5G Body Modulu</h1>
  </header>

  <div class="page-body">
    <FolderPicker 
      label="Priečinok filtrov operátorov:"
      bind:folderPath={filterPath} 
    />

    <div class="multiple-files-box">
      <span class="box-title">NSA 5G body:</span>
      <div class="files-list">
        {#each mobilePaths as bod, i}
          <div class="bod-item">
            <div class="bod-header">
              <span class="bod-title">Bod {i + 1}</span>
              <button class="remove-btn" onclick={() => removeMobilePath(i)}>X</button>
            </div>
            <div class="bod-inputs">
              <div class="bod-row">
                <span>LTE cesta:</span>
                <input type="text" bind:value={bod.lte_path} placeholder="Cesta k LTE súboru..." />
                <button onclick={() => pickLte(i)}>Pridať cestu</button>
              </div>
              <div class="bod-row">
                <span>5G cesta:</span>
                <input type="text" bind:value={bod.g5_path} placeholder="Cesta k 5G súboru..." />
                <button onclick={() => pick5G(i)}>Pridať cestu</button>
              </div>
            </div>
          </div>
        {/each}
      </div>
      <button class="add-btn" onclick={addMobilePath}>+ Pridať bod</button>
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

    <Dropdown label="RSRP Stlpec (5G):" options={fivegRsrpOptions} bind:value={appSettings.fiveg_rsrp} />
    <Dropdown label="SINR Stlpec (5G):" options={fivegSinrOptions} bind:value={appSettings.fiveg_sinr} />

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
  .files-list { max-height: 250px; overflow-y: auto; margin-bottom: 12px; display: flex; flex-direction: column; gap: 12px; padding-right: 8px;}
  .bod-item { background: var(--input-bg); border: 1px solid var(--input-border); border-radius: 4px; padding: 12px; display: flex; flex-direction: column; gap: 10px; }
  .bod-header { display: flex; justify-content: space-between; align-items: center; font-weight: bold; font-size: 14px; color: #fff;}
  .bod-row { display: flex; align-items: center; gap: 10px; font-size: 14px; }
  .bod-row span { width: 70px; color: #ccc; }
  .bod-row input { flex: 1; padding: 6px 10px; background: #2d2d2d; color: #ccc; border: 1px solid #555; border-radius: 4px; }
  .bod-row input:focus { outline: 1px solid var(--accent-outline); border-color: var(--accent-outline); }
  .bod-row button { padding: 6px 12px; background: var(--accent-color); color: white; border: none; border-radius: 4px; cursor: pointer; white-space: nowrap; }
  .bod-row button:hover { background: #1177bb; }
  
  .remove-btn { background: #d13438; color: white; border: none; border-radius: 4px; padding: 4px 8px; cursor: pointer; }
  .remove-btn:hover { background: #e81123; }
  .add-btn { width: 100%; padding: 8px; background: var(--input-bg); border: 1px solid var(--input-border); color: white; border-radius: 4px; cursor: pointer; }
  .add-btn:hover { background: #444444; }

  .generate-btn { padding: 12px 40px; background-color: var(--accent-color); color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: bold; width: fit-content; margin-top: 20px; transition: background-color 0.2s; }
  .generate-btn:hover { background: #1177bb; }
  .generate-btn:disabled { background-color: var(--input-border); cursor: not-allowed; }
</style>