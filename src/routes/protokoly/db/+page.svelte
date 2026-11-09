<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import Dropdown from '$lib/components/Dropdown.svelte';
  import { fivegRsrpOptions, fivegSinrOptions } from '$lib/fiveg-modes';
  import { appSettings } from '$lib/settings.svelte';
  import { runProtocolGeneration } from '$lib/protocol-generator';

  let protokolDbPath = $state("");
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
      protocolType: 'db',
      protocolPath: protokolDbPath,
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
    <div>
      <h1>Protokol DB</h1>
      <p class="page-subtitle">Vyplnenie DB protokolu z vygenerovaných zón.</p>
    </div>
    <span class="page-badge">DB</span>
  </header>

  <div class="page-body">
    <section class="card">
      <h2 class="card-title">Súbory</h2>
      <FilePicker 
        label="Súbor protokol:"
        filterName="XLSX Súbory"
        extensions={['xlsx']}
        bind:filePath={protokolDbPath} 
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
    </section>

    <section class="card">
      <h2 class="card-title">5G stĺpce</h2>
      <Dropdown label="RSRP Stĺpec (5G):" options={fivegRsrpOptions} bind:value={appSettings.fiveg_rsrp} />
      <Dropdown label="SINR Stĺpec (5G):" options={fivegSinrOptions} bind:value={appSettings.fiveg_sinr} />
    </section>

    <section class="card">
      <h2 class="card-title">Parametre merania</h2>
      <TextInput label="Meraná obec:" bind:value={measuredCity} placeholder="Mesto/Obec..." />
      <div class="input-row-multi">
        <TextInput label="Hodnota Total Power:" bind:value={totalPower} />
      </div>
      <div class="input-row-multi">
        <TextInput label="Hodnota SINR:" bind:value={sinr} />
        <TextInput label="Hodnota RSRP:" bind:value={rsrp} />
      </div>
      <div class="input-row-multi">
        <TextInput label="Korekcia výšky antény:" bind:value={antennaHeight} />
        <TextInput label="Korekcia vnútorného prostredia:" bind:value={internalEnvironment} />
      </div>
    </section>

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'Generuje sa...' : 'Generovať'}
    </button>
  </div>
</div>
