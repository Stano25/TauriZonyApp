pub mod managers;

use serde::Deserialize;
use std::path::PathBuf;
use crate::managers::data_manager::{
    create_protocol,
    create_protocol_db,
    process_dataset,
    process_multiple_points_dataset,
    process_point_dataset,
    set_fiveg_modes,
    FiveGRecord,
    GsmRecord,
    LteRecord,
    MobilePathProvider,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            managers::json_manager::load_settings,
            managers::json_manager::save_settings,
            generate_zony_data,
            generate_body_data,
            generate_mobile_body_data,
            generate_protocol_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, Deserialize)]
struct MobileInputPaths {
    #[serde(alias = "ltePath")]
    lte_path: String,
    #[serde(alias = "g5Path")]
    g5_path: String,
}

impl MobilePathProvider for MobileInputPaths {
    fn lte_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.lte_path)
    }

    fn g5_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.g5_path)
    }
}

#[tauri::command]
async fn generate_zony_data(
    data_type: String, // "gsm", "lte", "5g"
    grid_path: String,
    input_path: String,
    output_path: String,
    generate_missing_operators: bool,
    only_best_operator: bool,
    min_best_operator_sinr: Option<f32>,
    use_filter: bool,
    filter_path: String,
    create_duplicates: bool,
    rsrp_mode: Option<String>,
    sinr_mode: Option<String>
) -> Result<String, String> {
    
    // Nacitame mody pre 5G ak boli odoslane
    if data_type == "5g" {
        if let (Some(rsrp), Some(sinr)) = (rsrp_mode, sinr_mode) {
            set_fiveg_modes(&rsrp, &sinr);
        }
    }

    // Prah SINR pre rezim "iba najlepsieho operatora" ma zmysel len pri LTE a 5G
    // (GSM nema SINR). Pri 5G sa berie SINR z pouzivatelom zvoleneho stlpca.
    let best_operator_sinr = if only_best_operator && (data_type == "lte" || data_type == "5g") {
        min_best_operator_sinr
    } else {
        None
    };

    let t_grid = PathBuf::from(grid_path);
    let t_in = PathBuf::from(input_path);
    let t_out = PathBuf::from(output_path);
    let t_filter = PathBuf::from(filter_path);

    tokio::task::spawn_blocking(move || {
        let result = match data_type.as_str() {
            "gsm" => process_dataset::<GsmRecord>(t_grid, t_in, t_out, generate_missing_operators, only_best_operator, best_operator_sinr, use_filter, t_filter, create_duplicates),
            "lte" => process_dataset::<LteRecord>(t_grid, t_in, t_out, generate_missing_operators, only_best_operator, best_operator_sinr, use_filter, t_filter, create_duplicates),
            "5g" => process_dataset::<FiveGRecord>(t_grid, t_in, t_out, generate_missing_operators, only_best_operator, best_operator_sinr, use_filter, t_filter, create_duplicates),
            _ => Err("Neznamy typ siete".to_string()),
        };

        match result {
            Ok(_) => Ok("Subor vygenerovany.".to_string()),
            Err(e) => Err(e),
        }
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn generate_body_data(
    data_type: String, // "lte", "5g"
    input_paths: Vec<String>,
    output_path: String,
    filter_path: String,
    protocol_path: Option<String>,
    second_output_path: String,
    use_filter: bool,
    max_distance: f64,
    generate_missing_operators: bool,
    threshold_sinr: f32,
    threshold_rsrp: f32,
    create_duplicates: bool,
    rsrp_mode: Option<String>,
    sinr_mode: Option<String>,
) -> Result<String, String> {
    if input_paths.is_empty() {
        return Err("Nezadali ste vstupne CSV subory.".to_string());
    }

    if data_type == "5g" {
        let rsrp = rsrp_mode.unwrap_or_else(|| "SS_PBCH-RSRP".to_string());
        let sinr = sinr_mode.unwrap_or_else(|| "SS_PBCH-SINR".to_string());
        set_fiveg_modes(&rsrp, &sinr);
    }

    let t_inputs: Vec<PathBuf> = input_paths.into_iter().map(PathBuf::from).collect();
    let t_out = PathBuf::from(output_path);
    let t_filter = PathBuf::from(filter_path);
    let t_protocol = protocol_path
        .and_then(|p| if p.trim().is_empty() { None } else { Some(PathBuf::from(p)) });
    let t_second_output = PathBuf::from(second_output_path);

    tokio::task::spawn_blocking(move || {
        let result = match data_type.as_str() {
            "lte" => process_point_dataset::<LteRecord>(
                t_inputs,
                t_out,
                t_filter,
                t_protocol,
                t_second_output,
                use_filter,
                max_distance,
                generate_missing_operators,
                threshold_sinr,
                threshold_rsrp,
                create_duplicates,
            ),
            "5g" => process_point_dataset::<FiveGRecord>(
                t_inputs,
                t_out,
                t_filter,
                t_protocol,
                t_second_output,
                use_filter,
                max_distance,
                generate_missing_operators,
                threshold_sinr,
                threshold_rsrp,
                create_duplicates,
            ),
            _ => Err("Neznamy typ body datasetu".to_string()),
        };

        match result {
            Ok(_) => Ok("Body boli uspesne vygenerovane.".to_string()),
            Err(e) => Err(e),
        }
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn generate_mobile_body_data(
    input_entries: Vec<MobileInputPaths>,
    output_path: String,
    filter_path: String,
    protocol_path: Option<String>,
    second_output_path: String,
    use_filter: bool,
    max_distance: f64,
    generate_missing_operators: bool,
    threshold_sinr: f32,
    threshold_rsrp: f32,
    create_duplicates: bool,
    rsrp_mode: Option<String>,
    sinr_mode: Option<String>,
) -> Result<String, String> {
    if input_entries.is_empty() {
        return Err("Nezadali ste ziadne NSA 5G body.".to_string());
    }

    let rsrp = rsrp_mode.unwrap_or_else(|| "SS_PBCH-RSRP".to_string());
    let sinr = sinr_mode.unwrap_or_else(|| "SS_PBCH-SINR".to_string());
    set_fiveg_modes(&rsrp, &sinr);

    let t_out = PathBuf::from(output_path);
    let t_filter = PathBuf::from(filter_path);
    let t_protocol = protocol_path
        .and_then(|p| if p.trim().is_empty() { None } else { Some(PathBuf::from(p)) });
    let t_second_output = PathBuf::from(second_output_path);

    tokio::task::spawn_blocking(move || {
        let result = process_multiple_points_dataset::<MobileInputPaths>(
            input_entries,
            t_out,
            t_filter,
            t_protocol,
            t_second_output,
            use_filter,
            max_distance,
            generate_missing_operators,
            threshold_sinr,
            threshold_rsrp,
            create_duplicates,
        );

        match result {
            Ok(_) => Ok("NSA 5G body boli uspesne vygenerovane.".to_string()),
            Err(e) => Err(e),
        }
    }).await.map_err(|e| e.to_string())?
}

#[tauri::command]
async fn generate_protocol_data(
    protocol_type: String, // "lte", "5g", "db"
    protocol_path: String,
    gsm_path: String,
    lte_path: String,
    fiveg_path: Option<String>,
    output_path: String,
    measured_city: String,
    total_power: f32,
    sinr: f32,
    rsrp: f32,
    antenna_height: f32,
    internal_environment: f32,
    rsrp_mode: Option<String>,
    sinr_mode: Option<String>,
) -> Result<String, String> {
    if protocol_type == "5g" || protocol_type == "db" {
        let rsrp = rsrp_mode.unwrap_or_else(|| "SS_PBCH-RSRP".to_string());
        let sinr = sinr_mode.unwrap_or_else(|| "SS_PBCH-SINR".to_string());
        set_fiveg_modes(&rsrp, &sinr);
    }

    let t_protocol = PathBuf::from(protocol_path);
    let t_gsm = PathBuf::from(gsm_path);
    let t_lte = PathBuf::from(lte_path);
    let t_out = PathBuf::from(output_path);

    tokio::task::spawn_blocking(move || {
        let result = match protocol_type.as_str() {
            "lte" => create_protocol(
                t_protocol,
                t_gsm,
                t_lte,
                None,
                t_out,
                measured_city,
                total_power,
                sinr,
                rsrp,
                antenna_height,
                internal_environment,
            ),
            "5g" => {
                let fiveg = fiveg_path
                    .and_then(|p| if p.trim().is_empty() { None } else { Some(PathBuf::from(p)) })
                    .ok_or_else(|| "Pre 5G protokol chyba 5G CSV subor.".to_string())?;
                create_protocol(
                    t_protocol,
                    t_gsm,
                    t_lte,
                    Some(fiveg),
                    t_out,
                    measured_city,
                    total_power,
                    sinr,
                    rsrp,
                    antenna_height,
                    internal_environment,
                )
            }
            "db" => {
                let fiveg = fiveg_path
                    .and_then(|p| if p.trim().is_empty() { None } else { Some(PathBuf::from(p)) })
                    .ok_or_else(|| "Pre DB protokol chyba 5G CSV subor.".to_string())?;
                create_protocol_db(
                    t_protocol,
                    t_gsm,
                    t_lte,
                    fiveg,
                    t_out,
                    measured_city,
                    total_power,
                    sinr,
                    rsrp,
                    antenna_height,
                    internal_environment,
                )
            }
            _ => Err("Neznamy typ protokolu".to_string()),
        };

        match result {
            Ok(_) => Ok("Protokol bol uspesne vygenerovany.".to_string()),
            Err(e) => Err(e),
        }
    }).await.map_err(|e| e.to_string())?
}

