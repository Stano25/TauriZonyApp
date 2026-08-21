<script lang="ts">
  import { save } from '@tauri-apps/plugin-dialog';

  let { 
    label, 
    filterName = 'Všetky súbory', 
    extensions = ['*'],
    savePath = $bindable("")
  } = $props<{
    label: string;
    filterName?: string;
    extensions?: string[];
    savePath?: string;
  }>();

  async function handleSaveFile() {
    const selected = await save({
      filters: [{
        name: filterName,
        extensions: extensions
      }]
    });

    if (selected !== null) {
      savePath = selected as string;
    }
  }
</script>

<div class="file-input-group">
  <label>{label}</label>
  <div class="input-row">
    <input type="text" bind:value={savePath} placeholder="Cesta pre uloženie..." />
    <button type="button" onclick={handleSaveFile}>Uložiť súbor</button>
  </div>
</div>

<style>
  .file-input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 600px;
    margin-bottom: 16px;
  }

  .file-input-group label {
    font-size: 14px;
    font-weight: 500;
    color: var(--gray-text);
  }

  .input-row {
    display: flex;
    gap: 10px;
  }

  .input-row input {
    flex: 1;
    padding: 8px 12px;
    background-color: var(--input-bg);
    color: var(--gray-text);
    border: 1px solid var(--input-border);
    border-radius: 4px;
    font-size: 14px;
  }

  .input-row input:focus {
    outline: 1px solid var(--accent-outline);
    border-color: var(--accent-outline);
  }

  .input-row button {
    padding: 8px 16px;
    background-color: var(--accent-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.2s;
  }

  .input-row button:hover {
    background-color: var(--accent-hover);
  }
</style>