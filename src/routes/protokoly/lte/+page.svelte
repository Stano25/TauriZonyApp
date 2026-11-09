<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import TextInput from '$lib/components/TextInput.svelte';
  import { appSettings } from '$lib/settings.svelte';
  import { runProtocolGeneration } from '$lib/protocol-generator';

  let protokolPath = $state("");
  let firstPath = $state("");
  let secondPath = $state("");
  let outputPath = $state("");

  let measuredCity = $state("");
  let totalPower = $state("-20.0");
  let sinr = $state("-20.0");
  let rsrp = $state("-20.0");
  let antennaHeight = $state("0.0");
  let internalEnvironment = $state("0.0");

  async function handleGenerate() {
    await runProtocolGeneration({
      protocolType: 'lte',
      protocolPath: protokolPath,
      gsmPath: firstPath,
      ltePath: secondPath,
      outputPath,
      measuredCity,
      totalPower,
      sinr,
      rsrp,
      antennaHeight,
      internalEnvironment,
    });
  }
</script>

<div class="page-container">
  <header class="page-header">
    <div>
      <h1>Protokol LTE</h1>
      <p class="page-subtitle">Vyplnenie protokolu z vygenerovaných zón.</p>
    </div>
    <span class="page-badge">LTE</span>
  </header>

  <div class="page-body">
    <section class="card">
      <h2 class="card-title">Súbory</h2>
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

      <FileSaver 
        label="Uložiť protokol ako:"
        filterName="Excel súbory"
        extensions={['xlsx']}
        bind:savePath={outputPath} 
      />
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
