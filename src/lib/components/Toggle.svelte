<script lang="ts">
  let {
    label,
    help = "",
    disabled = false,
    checked = $bindable(false)
  } = $props<{
    label: string;
    help?: string;
    disabled?: boolean;
    checked?: boolean;
  }>();
</script>

<div class="toggle-group" class:disabled>
  <label class="toggle-container">
    <span class="label-text">
      {label}
      {#if help}
        <span class="label-help">{help}</span>
      {/if}
    </span>
    <div class="switch">
      <input type="checkbox" bind:checked={checked} {disabled} />
      <span class="slider"></span>
    </div>
  </label>
</div>

<style>
  .toggle-group {
    margin-bottom: 14px;
    transition: opacity 0.15s ease;
  }

  .toggle-group:last-child {
    margin-bottom: 0;
  }

  .toggle-group.disabled {
    opacity: 0.45;
  }

  .toggle-group.disabled .toggle-container {
    cursor: not-allowed;
  }

  .toggle-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    width: 100%;
    max-width: 640px;
    cursor: pointer;
    user-select: none;
  }

  .label-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    font-size: 14px;
    font-weight: 500;
    color: var(--gray-text);
  }

  .label-help {
    font-size: 12px;
    font-weight: 400;
    color: var(--gray-text);
    opacity: 0.75;
  }

  .switch {
    position: relative;
    flex-shrink: 0;
    display: inline-block;
    width: 40px;
    height: 22px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    inset: 0;
    background-color: var(--input-bg);
    transition: 0.2s;
    border-radius: 22px;
    border: 1px solid var(--input-border);
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 14px;
    width: 14px;
    left: 3px;
    bottom: 3px;
    background-color: var(--gray-text);
    transition: 0.2s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: var(--accent-color);
    border-color: var(--accent-color);
  }

  input:checked + .slider:before {
    transform: translateX(18px);
    background-color: #ffffff;
  }

  input:focus-visible + .slider {
    box-shadow: 0 0 0 3px var(--accent-soft);
  }
</style>
