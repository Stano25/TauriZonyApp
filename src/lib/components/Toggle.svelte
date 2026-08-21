<script lang="ts">
  let {
    label,
    checked = $bindable(false)
  } = $props<{
    label: string;
    checked?: boolean;
  }>();
</script>

<div class="toggle-group">
  <label class="toggle-container">
    <span class="label-text">{label}</span>
    <div class="switch">
      <input type="checkbox" bind:checked={checked} />
      <span class="slider"></span>
    </div>
  </label>
</div>

<style>
  .toggle-group {
    margin-bottom: 16px;
  }

  .toggle-container {
    display: inline-flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
    user-select: none;
  }

  .label-text {
    font-size: 14px;
    font-weight: 500;
    color: var(--gray-text);
  }

  .switch {
    position: relative;
    display: inline-block;
    width: 36px;
    height: 20px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--input-bg);
    transition: 0.2s;
    border-radius: 20px;
    border: 1px solid var(--input-border);
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 12px;
    width: 12px;
    left: 3px;
    bottom: 3px;
    background-color: var(--gray-text);
    transition: 0.2s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: var(--accent-color); /* VS Code active blue */
    border-color: var(--accent-color);
  }

  input:checked + .slider:before {
    transform: translateX(16px);
    background-color: #ffffff;
  }
</style>