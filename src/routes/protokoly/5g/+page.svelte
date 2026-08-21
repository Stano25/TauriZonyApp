<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import Dropdown from '$lib/components/Dropdown.svelte';
  import { fivegRsrpOptions, fivegSinrOptions } from '$lib/fiveg-modes';
  import { appSettings } from '$lib/settings.svelte';
  import { runProtocolGeneration } from '$lib/protocol-generator';

  let protokolPath = $state("");
  let firstPath = $state("");
  let secondPath = $state("");
  let thirdPath = $state("");
  let outputPath = $state("");

  let measuredCity = $state("");
  let totalPower = $state("-20.0");
  let sinr = $state("-20.0");
  let rsrp = $state("-20.0");
  let antennaHeight = $state("0.0");
  let internalEnvironment = $state("0.0");

  async function handleGenerate() {
    await runProtocolGeneration({
      protocolType: '5g',
      protocolPath: protokolPath,
      gsmPath: firstPath,
      ltePath: secondPath,
      fivegPath: thirdPath,
      outputPath,
      measuredCity,
      totalPower,
      sinr,
      rsrp,
      antennaHeight,
      internalEnvironment,
      rsrpMode: appSettings.fiveg_rsrp,
      sinrMode: appSettings.fiveg_sinr,
    });
  }
</script>

<div class="page-container">
  <header class="page-header">
    <h1>Konfigurácia Protokol 5G Modulu</h1>
  </header>

  <div class="page-body">
    <FilePicker 
      label="Súbor protokol:"
      filterName="XLSX Súbory"
      extensions={['xlsx']}
      bind:filePath={protokolPath} 
    />
    
    <FilePicker 
      label="Súbor GSM zóny:"
      filterName="CSV Súbory"
      extensions={['csv']}
      bind:filePath={firstPath} 
    />

    <FilePicker 
      label="Súbor LTE zóny:"
      filterName="CSV Súbory"
      extensions={['csv']}
      bind:filePath={secondPath} 
    />

    <FilePicker 
      label="Súbor 5G zóny:"
      filterName="CSV Súbory"
      extensions={['csv']}
      bind:filePath={thirdPath} 
    />

    <FileSaver 
      label="Uložiť protokol ako:"
      filterName="Excel súbory"
      extensions={['xlsx']}
      bind:savePath={outputPath} 
    />

    <TextInput label="Meraná obec:" bind:value={measuredCity} placeholder="Mesto/Obec..." />
    <TextInput label="Hodnota Total Power:" bind:value={totalPower} />
    
    <div class="input-row-multi">
      <TextInput label="Hodnota SINR:" bind:value={sinr} />
      <TextInput label="Hodnota RSRP:" bind:value={rsrp} />
    </div>

    <Dropdown label="RSRP Stlpec (5G):" options={fivegRsrpOptions} bind:value={appSettings.fiveg_rsrp} />
    <Dropdown label="SINR Stlpec (5G):" options={fivegSinrOptions} bind:value={appSettings.fiveg_sinr} />

    <TextInput label="Korekcia výšky antény:" bind:value={antennaHeight} />
    <TextInput label="Korekcia vnútorného prostredia:" bind:value={internalEnvironment} />

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'GENERUJE SA...' : 'GENEROVAT'}
    </button>
  </div>
</div>

<style>
  .page-container { padding: 20px; height: 100%; box-sizing: border-box; display: flex; flex-direction: column; gap: 24px; overflow-y: auto; }
  .page-header h1 { margin: 0; color: var(--text-color);}
  .page-body { flex: 1; display: flex; flex-direction: column; }
  
  .input-row-multi { display: flex; gap: 20px; max-width: 600px;}
  
  .generate-btn { padding: 12px 40px; background-color: var(--accent-color); color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: bold; width: fit-content; margin-top: 20px; transition: background-color 0.2s; }
  .generate-btn:hover { background: #1177bb; }
  .generate-btn:disabled { background-color: var(--input-border); cursor: not-allowed; }
</style>