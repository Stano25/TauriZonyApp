use std::error::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

fn default_is_terminal_open() -> bool { true }
fn default_fiveg_rsrp() -> String { String::from("SS_PBCH-RSRP") }
fn default_fiveg_sinr() -> String { String::from("SS_PBCH-SINR") }
fn default_zony_min_best_sinr() -> String { String::from("0") }

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppSettings {
    pub zone_file_path: String,
    pub filter_path: String,
    pub protokol_path: String,
    pub protocol_points_path: String,
    pub protocol_db_path: String,

    pub is_dark_mode: bool,
    #[serde(default = "default_is_terminal_open")]
    pub is_terminal_open: bool,
    pub generate_missing_operators: bool,
    pub use_filter: bool,
    pub use_protocol_points: bool,
    pub create_duplicates: bool,

    // Vybrane 5G stlpce (musia sa tiez ukladat, inak sa po restarte stracia)
    #[serde(default = "default_fiveg_rsrp")]
    pub fiveg_rsrp: String,
    #[serde(default = "default_fiveg_sinr")]
    pub fiveg_sinr: String,

    // Zony: ponechat iba najlepsieho operatora + minimalny SINR prah + cesty vstup/vystup
    pub zony_only_best_operator: bool,
    #[serde(default = "default_zony_min_best_sinr")]
    pub zony_min_best_sinr: String,
    pub zony_input_path: String,
    pub zony_output_path: String,
}

fn get_config_path(folder_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    // BEZPEČNOSTNÁ KONTROLA:
    // Názov priečinka prichádza z webview, takže musíme zablokovať cestové oddelovače
    // a odkazy na nadradený adresár (ochrana proti path traversal mimo config adresára).
    if folder_name.is_empty()
        || folder_name.contains('/')
        || folder_name.contains('\\')
        || folder_name.contains("..")
    {
        return Err("Neplatný názov priečinka nastavení.".into());
    }

    let config_base = dirs::config_dir().ok_or("Failed to find config directory")?;
    let mut path = config_base;
    path.push(folder_name);
    path.push("config.json");
    Ok(path)
}

#[tauri::command]
pub fn load_settings(folder_name: &str) -> AppSettings {
    match try_read_json(folder_name) {
        Ok(settings) => settings,
        Err(_) => {
            // Ak načítanie zlyhá (neexistuje, alebo iný problém), vytvoríme default a pokúsime sa uložiť
            let default_settings = AppSettings::default();
            let _ = save_json(folder_name, &default_settings);
            default_settings
        }
    }
}

#[tauri::command]
pub fn save_settings(folder_name: &str, settings: AppSettings) -> Result<(), String> {
    save_json(folder_name, &settings).map_err(|e| e.to_string())
}

pub fn try_read_json(folder_name: &str) -> Result<AppSettings, Box<dyn Error>>{
    let path = get_config_path(folder_name)?;
    
    // Ak súbor neexistuje, vrátime chybu alebo môžeme rovno vytvoriť default (záleží na logike)
    let data = fs::read_to_string(path)?;
    let config: AppSettings = serde_json::from_str(&data)?;
    Ok(config)
}

pub fn save_json(folder_name: &str, settings: &AppSettings) -> Result<(), Box<dyn Error>>{
    let path = get_config_path(folder_name)?;

    // BEZPEČNOSTNÝ KROK:
    // Zistíme rodičovský priečinok (t.j. .../App-zony-100m) a uistíme sa, že existuje.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json_string = serde_json::to_string_pretty(&settings)?;
    fs::write(path, json_string)?;
    
    Ok(())
}