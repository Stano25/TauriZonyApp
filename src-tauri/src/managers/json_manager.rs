use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct AppSettings {
    pub zone_file_path: String,
    pub filter_path: String,
    pub protokol_path: String,
    pub protocol_points_path: String,
    pub protocol_db_path: String,

    pub is_dark_mode: bool,
    pub is_terminal_open: bool,
    pub generate_missing_operators: bool,
    pub use_filter: bool,
    pub use_protocol_points: bool,
    pub create_duplicates: bool,
}

fn get_config_path(folder_name: &str) -> Result<PathBuf, Box<dyn Error>> {
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

pub fn create_blank_json(folder_name: &str) -> Result<(), Box<dyn Error>>{
    let config_base = dirs::config_dir().ok_or("Failed to find config directory")?;
    let mut dir_path = PathBuf::from(&config_base);
    dir_path.push(folder_name);

    // 2. Vytvoríme priečinok (ak neexistuje)
    fs::create_dir_all(&dir_path)?;

    // 3. Pripravíme cestu k súboru (priečinok + config.json)
    let file_path = dir_path.join("config.json");

    // 4. Vytvoríme defaultné dáta
    let default_data = AppSettings::default();

    // 5. Serializujeme dáta do Stringu (pretty = pekne formátovaný JSON)
    let json_string = serde_json::to_string_pretty(&default_data)?;

    // 6. Zapíšeme do súboru
    fs::write(file_path, json_string)?;

    Ok(())
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