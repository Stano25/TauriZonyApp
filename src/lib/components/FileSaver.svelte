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

<div class="field">
  <label>{label}</label>
  <div class="input-row">
    <input class="input" type="text" bind:value={savePath} placeholder="Cesta pre uloženie..." />
    <button type="button" class="btn btn-primary" onclick={handleSaveFile}>Uložiť súbor</button>
  </div>
</div>
