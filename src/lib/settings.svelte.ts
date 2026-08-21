import { invoke } from '@tauri-apps/api/core';

const FOLDER_NAME = 'App-zony-100m';

// Globálny stav pre všetky okná
export const appSettings: Record<string, any> = $state({
    zone_file_path: "",
    filter_path: "",
    protokol_path: "",
    protocol_points_path: "",
    protocol_db_path: "",
    is_dark_mode: false,
    is_terminal_open: false,
    generate_missing_operators: false,
    use_filter: false,
    use_protocol_points: false,
    create_duplicates: false,
    is_generating: false,
    
    fiveg_rsrp: "SS_PBCH-RSRP",
    fiveg_sinr: "SS_PBCH-SINR",

    is_loaded: false
});

export async function loadSettings() {
    try {
        const data = await invoke<any>('load_settings', { folderName: FOLDER_NAME });
        for (const key in data) {
            appSettings[key] = data[key];
        }
        appSettings.is_loaded = true;
    } catch (e) {
        console.error("Failed to load settings from Rust", e);
    }
}

export async function saveSettings() {
    if (!appSettings.is_loaded) return;
    try {
        // Vytvor kópiu a odstráň is_loaded pred odoslaním
        const payload = { ...appSettings };
        delete payload.is_loaded;
        delete payload.is_generating;
        delete payload.fiveg_rsrp;
        delete payload.fiveg_sinr;
        
        await invoke('save_settings', { folderName: FOLDER_NAME, settings: payload });
    } catch (e) {
        console.error("Failed to save settings to Rust", e);
    }
}
