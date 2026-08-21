<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { goto } from '$app/navigation';
  import { slide } from 'svelte/transition';

  import { appSettings, loadSettings, saveSettings } from '$lib/settings.svelte';
  import { darkModeColors, lightModeColors } from '$lib/constants/colors';

  let { children } = $props();

  $effect(() => {
     if (appSettings.is_loaded) {
       // Watch for any changes in appSettings
       JSON.stringify(appSettings);
       saveSettings();
     }
  });

  $effect(() => {
    if (typeof document !== 'undefined') {
      const colors = appSettings.is_dark_mode ? darkModeColors : lightModeColors;
      for (const [key, value] of Object.entries(colors)) {
        document.body.style.setProperty(key, value);
      }
      
      if (appSettings.is_dark_mode) {
        document.body.classList.add('dark-mode');
        document.body.classList.remove('light-mode');
      } else {
        document.body.classList.add('light-mode');
        document.body.classList.remove('dark-mode');
      }
    }
  });

  let selectedMenu = $state<string | null>(null);

  // Zoznam logov
  type Log = { message: string, log_type: 'normal' | 'success' | 'warning' | 'error' };
  let logs = $state<Log[]>([]);

  // Constants
  const HEADER_HEIGHT = 28; // The height of the terminal-header

  let terminalHeight = $state(200); // Starter fallback before JS mounts
  let isTerminalResizing = $state(false);
  let isTerminalOpen = $state(true);

  onMount(() => {
    // Načítanie globálnych nastavení
    loadSettings();

    // Ovládnutie výšky z properties reálneho okna (Window API)
    terminalHeight = window.innerHeight * 0.3;

    // Počúva eventy s názvom 'terminal-log' z Rust backendu
    const unlisten = listen<Log>('terminal-log', (event) => {
      logs.push(event.payload);
      
      // Automatické scrollovanie nižšie, ak sa pridá nový log
      setTimeout(() => {
        const terminalContent = document.querySelector('.terminal-content');
        if (terminalContent) {
          terminalContent.scrollTop = terminalContent.scrollHeight;
        }
      }, 10);
    });

    const unlistenClear = listen('terminal-clear', () => {
      logs = [];
    });

    return () => {
      unlisten.then(f => f());
      unlistenClear.then(f => f());
    };
  });

  function startTerminalResizing(e: MouseEvent) {
    if (!isTerminalOpen) return;
    isTerminalResizing = true;
    document.addEventListener("mousemove", handleTerminalMouseMove);
    document.addEventListener("mouseup", stopTerminalResizing);
  }

  function handleTerminalMouseMove(e: MouseEvent) {
    if (isTerminalResizing) {
      const newHeight = window.innerHeight - e.clientY;
      
      // Auto-collapse if dragged too small (e.g. below 40px)
      if (newHeight < HEADER_HEIGHT + 10) {
        isTerminalOpen = false;
        stopTerminalResizing();
        return;
      }

      // Clamp terminal height: max 50% of window height so it doesn't take over screen
      terminalHeight = Math.max(HEADER_HEIGHT, Math.min(window.innerHeight * 0.4, newHeight));
    }
  }

  function stopTerminalResizing() {
    isTerminalResizing = false;
    document.removeEventListener("mousemove", handleTerminalMouseMove);
    document.removeEventListener("mouseup", stopTerminalResizing);
  }

  function toggleTerminal() {
    isTerminalOpen = !isTerminalOpen;
    if (isTerminalOpen) {
      terminalHeight = window.innerHeight * 0.3; // Reset always on button open to default
    }
  }

  function toggleMenu(menu: string) {
    selectedMenu = selectedMenu === menu ? null : menu;
  }
</script>

{#snippet dropdownIcon(isOpen: boolean)}
  <span style="display: flex; align-items: center; justify-content: center;">
    <svg width="1.2em" height="1.2em" viewBox="0 0 512 512" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="48">
      {#if isOpen}
        <!-- caret-down / chevron-down -->
        <path d="M112 184l144 144 144-144"/>
      {:else}
        <!-- caret-forward / chevron-forward -->
        <path d="M184 112l144 144-144 144"/>
      {/if}
    </svg>
  </span>
{/snippet}

{#snippet closeIcon()}
  <span style="display: flex; align-items: center; justify-content: center;">
    <svg width="1.2em" height="1.2em" viewBox="0 0 512 512" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="48">
      <!-- close / cross -->
      <path d="M368 368L144 144M368 144L144 368"/>
    </svg>
  </span>
{/snippet}

{#snippet terminalIcon()}
  <span style="display: flex; align-items: center; justify-content: center;">
    <svg width="1.2em" height="1.2em" viewBox="0 0 512 512" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="48">
      <!-- chevron up (for reopening) -->
      <path d="M112 328l144-144 144 144"/>
    </svg>
  </span>
{/snippet}

<div class="app-container">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-content">
      <h2 class="tools-title">Nástroje</h2>
      
      <div class="menu">
        <div class="dropdown">
          <button class="dropdown-btn" onclick={() => toggleMenu('Zone')}>
            <span>Zóny</span>
            {@render dropdownIcon(selectedMenu === 'Zone')}
          </button>
          {#if selectedMenu === 'Zone'}
            <div class="dropdown-content" transition:slide={{ duration: 250 }}>
              <button onclick={() => { goto('/zony/gsm'); selectedMenu = null; }}>GSM</button>
              <button onclick={() => { goto('/zony/lte'); selectedMenu = null; }}>LTE</button>
              <button onclick={() => { goto('/zony/5g'); selectedMenu = null; }}>5G</button>
            </div>
          {/if}
        </div>

        <div class="dropdown">
          <button class="dropdown-btn" onclick={() => toggleMenu('Point')}>
            <span>Body</span>
            {@render dropdownIcon(selectedMenu === 'Point')}
          </button>
          {#if selectedMenu === 'Point'}
            <div class="dropdown-content" transition:slide={{ duration: 250 }}>
              <button onclick={() => { goto('/body/lte'); selectedMenu = null; }}>LTE</button>
              <button onclick={() => { goto('/body/5g'); selectedMenu = null; }}>5G</button>
              <button onclick={() => { goto('/body/nsa-5g'); selectedMenu = null; }}>NSA 5G</button>
            </div>
          {/if}
        </div>

        <div class="dropdown">
          <button class="dropdown-btn" onclick={() => toggleMenu('Protocol')}>
            <span>Protokoly</span>
            {@render dropdownIcon(selectedMenu === 'Protocol')}
          </button>
          {#if selectedMenu === 'Protocol'}
            <div class="dropdown-content" transition:slide={{ duration: 250 }}>
              <button onclick={() => { goto('/protokoly/lte'); selectedMenu = null; }}>LTE</button>
              <button onclick={() => { goto('/protokoly/5g'); selectedMenu = null; }}>5G</button>
              <button onclick={() => { goto('/protokoly/db'); selectedMenu = null; }}>DB</button>
            </div>
          {/if}
        </div>
      </div>
    </div>
    
    <div class="sidebar-bottom">
      <hr />
      <button class="bottom-btn">Informácie</button>
      <hr />
      <button class="bottom-btn" onclick={() => goto('/nastavenia')}>Nastavenia</button>
    </div>
  </aside>

  <!-- Divider -->
  <div class="sidebar-divider"></div>

  <!-- Right Area (Content + Terminal) -->
  <div class="right-area">
    <!-- Main Content -->
    <main class="main-content">
      {@render children()}
    </main>

    <!-- Terminal -->
    <div class="terminal" style="height: {isTerminalOpen ? terminalHeight + 'px' : HEADER_HEIGHT + 'px'};">
      
      {#if isTerminalOpen}
        <!-- Terminal Resizer is now INSIDE the terminal and pushed to top -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="terminal-resizer" onmousedown={startTerminalResizing}></div>
      {/if}

      <!-- Terminal Header -->
      <div class="terminal-header" style="height: {HEADER_HEIGHT}px; box-sizing: border-box;">
        <span class="terminal-title">TERMINAL</span>
        <div class="terminal-actions">
          <button class="icon-btn" onclick={(e) => { e.stopPropagation(); toggleTerminal(); }} title={isTerminalOpen ? "Close Panel" : "Open Panel"}>
            {#if isTerminalOpen}
              {@render closeIcon()}
            {:else}
              {@render terminalIcon()}
            {/if}
          </button>
        </div>
      </div>

      <!-- Terminal Content -->
      <div class="terminal-content-wrapper" style="display: {isTerminalOpen ? 'flex' : 'none'}; flex-direction: column; flex-grow: 1; overflow: hidden;">
        <div class="terminal-content">
          {#each logs as log}
            <div class="log {log.log_type}">{log.message}</div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    background-color: var(--bg-color);
    color: var(--text-color);
    overflow: hidden; /* Prevent full page scroll */
  }

  .app-container {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    width: 220px;
    flex-shrink: 0;
    background-color: var(--sidebar-bg);
  }

  .sidebar-content {
    flex-grow: 1;
    overflow-y: auto;
    padding: 16px;
  }

  .tools-title {
    text-align: center;
    font-size: 24px;
    font-weight: bold;
    margin-bottom: 30px;
    margin-top: 0;
  }

  .menu {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .dropdown-btn {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 10px 14px;
    background-color: var(--btn-bg);
    border: none;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
    color: inherit;
    font-size: 16px;
    cursor: pointer;
    text-align: left;
    transition: all 0.2s ease;
  }
  
  .dropdown-btn:hover {
    background-color: var(--btn-hover-bg);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.4);
    transform: translateY(-1px);
  }

  .dropdown-content {
    display: flex;
    flex-direction: column;
    padding-left: 12px;
    gap: 8px;
    margin-top: 8px;
  }

  .dropdown-content button {
    background-color: var(--btn-content-bg);
    border: none;
    border-radius: 6px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    color: inherit;
    font-size: 15px;
    text-align: left;
    padding: 8px 12px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .dropdown-content button:hover {
    background-color: var(--btn-content-hover-bg);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
    transform: translateX(2px);
  }

  .sidebar-bottom {
    padding-bottom: 0px;
  }

  hr {
    border: 0;
    height: 1px;
    background-color: var(--hr-color);
    margin: 0;
  }

  .bottom-btn {
    width: 100%;
    padding: 14px;
    background-color: transparent;
    border: none;
    color: inherit;
    font-weight: bold;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .bottom-btn:hover {
    background-color: rgba(128, 128, 128, 0.1);
  }

  .sidebar-divider {
    width: 1px;
    background-color: var(--divider-color);
    flex-shrink: 0;
  }

  .right-area {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    overflow: hidden;
  }

  .main-content {
    flex-grow: 1;
    overflow-y: auto;
    position: relative;
    background-color: var(--main-bg);
    color: var(--main-text);
  }

  /* Terminal CSS */
  .terminal-resizer {
    position: absolute;
    top: -2px; /* Let it sit just over the border */
    left: 0;
    width: 100%;
    height: 4px; /* Gripping area */
    cursor: row-resize;
    background-color: transparent;
    z-index: 10;
    transition: background-color 0.1s ease;
  }

  .terminal-resizer:hover, .terminal-resizer:active {
    background-color: var(--accent-outline); /* Modra pri uchopeni / hoveri (VS Code style) */
  }

  .terminal {
    background-color: #1e1e1e; /* Vždy tmavý */
    color: #cccccc;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    font-family: 'Consolas', 'Courier New', monospace;
    position: relative;
    border-top: 1px solid var(--border-color); /* Default border */
  }

  .terminal-header {
    font-size: 11px;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    text-transform: uppercase;
    letter-spacing: 1px;
    padding: 0 16px;
    color: #e7e7e7;
    background-color: #252526; /* Vždy tmavý hlavičkový pruh */
    border-bottom: 1px solid #333;
    display: flex;
    align-items: center;
    justify-content: space-between; /* title vlavo, akcie vpravo */
  }

  .terminal-title {
    flex-grow: 1;
    font-weight: bold;
    user-select: none;
  }

  .terminal-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: #cccccc; /* Pevná šedá farba pre tmavý terminál */
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .icon-btn:hover {
    background-color: #3e3e42; /* Odtieň tmavého hoveru */
    color: #ffffff;
  }

  .terminal-content {
    padding: 10px 16px;
    overflow-y: auto;
    flex-grow: 1;
    font-size: 13px;
    line-height: 1.5;
  }

  .log {
    margin-bottom: 4px;
    word-break: break-all;
  }
  
  .log.normal {
    color: #cccccc; /* Štandardný tmavý VS Code text */
  }
  
  .log.success {
    color: #4CAF50; /* Zelená */
  }
  
  .log.warning {
    color: #FFC107; /* Oranžová / žltá */
  }
  
  .log.error {
    color: #F44336; /* Červená */
  }
</style>
