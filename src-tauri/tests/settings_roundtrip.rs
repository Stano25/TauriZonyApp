use zony_app_lib::managers::json_manager::{load_settings, save_settings, try_read_json};

/// Overí celý cyklus: prvý load (vytvorí default), uloženie, znova načítanie.
#[test]
fn settings_roundtrip() {
    // Unikátny názov priečinka, aby test nikdy neprepísal reálne nastavenia
    let folder = "Test-zony-roundtrip-temp";

    // 1. PRVÉ SPUSTENIE: súbor neexistuje -> load_settings musí vytvoriť default a vrátiť ho
    let defaults = load_settings(folder);
    assert_eq!(defaults.fiveg_rsrp, "SS_PBCH-RSRP");
    assert_eq!(defaults.is_terminal_open, true);

    // Súbor musel fyzicky vzniknúť
    let path = dirs::config_dir().unwrap().join(folder).join("config.json");
    assert!(path.exists(), "config.json sa nevytvoril pri prvom spustení");

    // 2. ULOŽENIE: zmeníme polia vrátane nových zony_* a fiveg_*
    let mut s = try_read_json(folder).unwrap();
    s.zone_file_path = "/data/zony.csv".into();
    s.zony_only_best_operator = true;
    s.zony_min_best_sinr = "-3.5".into();
    s.zony_input_path = "/data/meranie.csv".into();
    s.zony_output_path = "/data/vystup.csv".into();
    s.fiveg_rsrp = "DM_RS-RSRP".into();
    s.fiveg_sinr = "PSS-SINR".into();
    s.is_dark_mode = true;
    save_settings(folder, s).unwrap();

    // 3. ZNOVA NAČÍTANIE: všetky polia musia prežiť
    let reloaded = try_read_json(folder).unwrap();
    assert_eq!(reloaded.zone_file_path, "/data/zony.csv");
    assert_eq!(reloaded.zony_only_best_operator, true);
    assert_eq!(reloaded.zony_min_best_sinr, "-3.5");
    assert_eq!(reloaded.zony_input_path, "/data/meranie.csv");
    assert_eq!(reloaded.zony_output_path, "/data/vystup.csv");
    assert_eq!(reloaded.fiveg_rsrp, "DM_RS-RSRP");
    assert_eq!(reloaded.fiveg_sinr, "PSS-SINR");
    assert_eq!(reloaded.is_dark_mode, true);
    assert_eq!(reloaded.is_terminal_open, true); // pole so serde default

    // 4. BEZPEČNOSŤ: path traversal musí byť zamietnutý (nepadá na panic)
    let evil = try_read_json("../evil");
    assert!(evil.is_err(), "path traversal musí zlyhať");

    // 5. Upratanie po teste
    std::fs::remove_dir_all(dirs::config_dir().unwrap().join(folder)).unwrap();
}
