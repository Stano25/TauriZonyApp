<script lang="ts">
  import FilePicker from '$lib/components/FilePicker.svelte';
  import FileSaver from '$lib/components/FileSaver.svelte';
  import Toggle from '$lib/components/Toggle.svelte';
  import { appSettings } from '$lib/settings.svelte';
  import { runZonyGeneration } from '$lib/zony-generator';

  let firstPath = $state("");
  let outputPath = $state("");

  async function handleGenerate() {
    await runZonyGeneration({
      dataType: 'gsm',
      inputPath: firstPath,
      outputPath,
    });
  }
</script>

<div class="page-container">
  <header class="page-header">
    <h1>Konfigurácia GSM Modulu</h1>
  </header>

  <div class="page-body">
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
      bind:filePath={firstPath} 
    />

    <FileSaver 
      label="Uložiť súbor ako:"
      filterName="CSV súbory"
      extensions={['csv']}
      bind:savePath={outputPath} 
    />
    
    <Toggle 
      label="Vygenerovať chýbajúcich operátorov:" 
      bind:checked={appSettings.generate_missing_operators} 
    />

    <button class="generate-btn" onclick={handleGenerate} disabled={appSettings.is_generating}>
      {appSettings.is_generating ? 'GENERUJE SA...' : 'GENEROVAT'}
    </button>
  </div>
</div>

<style>
  .page-container {
    padding: 20px;
    height: 100%;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 24px;
    overflow-y: auto;
  }

  .page-header h1 {
    margin: 0;
    color: var(--text-color);
  }

  .page-body {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .generate-btn {
    padding: 12px 40px;
    background-color: var(--accent-color); /* VS Code button farba */
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 16px;
    font-weight: bold;
    width: fit-content;
    margin-top: 20px;
    transition: background-color 0.2s;
  }

  .generate-btn:hover {
    background-color: var(--accent-hover);
  }

  .generate-btn:disabled {
    background-color: var(--input-border);
    cursor: not-allowed;
  }
</style>