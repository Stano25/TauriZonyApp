<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';

  let { 
    label, 
    folderPath = $bindable("") 
  } = $props<{
    label: string;
    folderPath?: string;
  }>();

  async function handleSelectFolder() {
    const selected = await open({
      directory: true,
      multiple: false
    });

    if (selected !== null) {
      folderPath = selected as string;
    }
  }
</script>

<div class="file-input-group">
  <label>{label}</label>
  <div class="input-row">
    <input type="text" bind:value={folderPath} placeholder="Cesta k priečinku..." />
    <button type="button" onclick={handleSelectFolder}>Vyber priečinok</button>
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
    background-color: var(--accent-color); /* VS Code button farba */
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