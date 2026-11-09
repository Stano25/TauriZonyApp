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

<div class="field">
  <label>{label}</label>
  <div class="input-row">
    <input class="input" type="text" bind:value={folderPath} placeholder="Cesta k priečinku..." />
    <button type="button" class="btn btn-primary" onclick={handleSelectFolder}>Vybrať priečinok</button>
  </div>
</div>
