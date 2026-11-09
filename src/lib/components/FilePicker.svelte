<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';

  // Svelte 5 Props (podobné ako destructuring v tvare: function FilePicker({ label, filterName, extensions, filePath }) v Reacte)
  let { 
    label, 
    filterName = 'Všetky súbory', 
    extensions = ['*'],
    filePath = $bindable("") // Allows two-way binding with parent (podobné onFilePathChange v Reacte)
  } = $props<{
    label: string;
    filterName?: string;
    extensions?: string[];
    filePath?: string;
  }>();

  async function handleSelectFile() {
    const selected = await open({
      multiple: false,
      filters: [{
        name: filterName,
        extensions: extensions
      }]
    });

    if (selected !== null) {
      filePath = selected as string;
    }
  }
</script>

<div class="field">
  <label>{label}</label>
  <div class="input-row">
    <input class="input" type="text" bind:value={filePath} placeholder="Cesta k súboru..." />
    <button type="button" class="btn btn-primary" onclick={handleSelectFile}>Vybrať súbor</button>
  </div>
</div>
