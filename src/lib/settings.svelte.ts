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
    is_terminal_open: true,
    generate_missing_operators: false,
    use_filter: false,
    use_protocol_points: false,
    create_duplicates: false,
    is_generating: false,
    
    fiveg_rsrp: "SS_PBCH-RSRP",
    fiveg_sinr: "SS_PBCH-SINR",

    // Zóny: iba najlepší operátor + minimálny SINR prah + cesty vstup/výstup
    zony_only_best_operator: false,
    zony_min_best_sinr: "0",
    // Offsety pripočítané na konci k vypočítanému priemeru RSRP/SINR (zony LTE/5G)
    zony_rsrp_offset: "0",
    zony_sinr_offset: "0",
    zony_input_path: "",
    zony_output_path: "",

    is_loaded: false
});

export async function loadSettings() {
    try {
        const data = await invoke<any>('load_settings', { folderName: FOLDER_NAME });
        for (const key in data) {
            // Prázdne 5G režimy z configu ignoruj (napr. zo staršej verzie configu),
            // aby sa neprepísali predvolené hodnoty
            if ((key === 'fiveg_rsrp' || key === 'fiveg_sinr') && !data[key]) continue;
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
        // Vytvor kópiu a odstráň runtime príznaky pred odoslaním
        const payload = { ...appSettings };
        delete payload.is_loaded;
        delete payload.is_generating;
        
        await invoke('save_settings', { folderName: FOLDER_NAME, settings: payload });
    } catch (e) {
        console.error("Failed to save settings to Rust", e);
    }
}
