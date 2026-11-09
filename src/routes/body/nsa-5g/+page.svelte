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
    <div>
      <h1>NSA 5G Body</h1>
      <p class="page-subtitle">Generovanie bodov z dvojíc LTE + 5G meraní.</p>
    </div>
    <span class="page-badge">NSA 5G</span>
  </header>

  <div class="page-body">
    <section class="card">
      <h2 class="card-title">Vstupné a výstupné súbory</h2>
      <FolderPicker 
        label="Priečinok filtrov operátorov:"
        bind:folderPath={filterPath} 
      />

      <div class="files-box">
        <span class="files-box-title">NSA 5G body:</span>
        <div class="files-list">
          {#each mobilePaths as bod, i}
            <div class="bod-item">
              <div class="bod-header">
                <span class="bod-title">Bod {i + 1}</span>
                <button class="btn btn-danger" onclick={() => removeMobilePath(i)}>X</button>
              </div>
              <div class="bod-inputs">
                <div class="bod-row">
                  <span>LTE cesta:</span>
                  <input class="input" type="text" bind:value={bod.lte_path} placeholder="Cesta k LTE súboru..." />
                  <button class="btn btn-primary" onclick={() => pickLte(i)}>Vybrať</button>
                </div>
                <div class="bod-row">
                  <span>5G cesta:</span>
                  <input class="input" type="text" bind:value={bod.g5_path} placeholder="Cesta k 5G súboru..." />
                  <button class="btn btn-primary" onclick={() => pick5G(i)}>Vybrať</button>
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
    </section>

    <section class="card">
      <h2 class="card-title">5G stĺpce</h2>
      <Dropdown label="RSRP Stĺpec (5G):" options={fivegRsrpOptions} bind:value={appSettings.fiveg_rsrp} />
      <Dropdown label="SINR Stĺpec (5G):" options={fivegSinrOptions} bind:value={appSettings.fiveg_sinr} />
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

<style>
  .bod-item {
    background: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: 8px;
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .bod-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 600;
    font-size: 13px;
    color: var(--text-color);
  }

  .bod-inputs {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .bod-row {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 13px;
  }

  .bod-row > span {
    width: 72px;
    flex-shrink: 0;
    color: var(--gray-text);
  }
</style>
