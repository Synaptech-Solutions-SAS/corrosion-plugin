use corrotion::presets::{Preset, PresetParameters, PRESET_VERSION};
use corrotion::Object;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_path(name: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("corrosion_{name}_{nanos}.corrosion-preset"))
}

#[test]
fn preset_roundtrip_save_and_load() {
    let preset = Preset {
        name: "Iron Strike".to_string(),
        version: PRESET_VERSION.to_string(),
        object: Object::Pipe,
        exciter: 0,
        size: 1.0,
        rust: 0.3,
        damage: 0.1,
        drive: 0.5,
        output: 1.0,
        width: 0.5,
        body: 0.2,
        extra: PresetParameters::default(),
    };

    let path = unique_temp_path("roundtrip");
    preset.save(&path).unwrap();
    let loaded = Preset::load(&path).unwrap();
    assert_eq!(preset, loaded);
    let _ = fs::remove_file(path);
}

#[test]
fn legacy_preset_loads_with_default_extended_parameters() {
    let json = r#"
    {
        "name": "Legacy Iron Strike",
        "version": "1",
        "object": "Pipe",
        "exciter": 0,
        "size": 1.0,
        "rust": 0.3,
        "damage": 0.1,
        "drive": 0.5,
        "output": 1.0,
        "width": 0.5,
        "body": 0.2
    }
    "#;

    let preset: Preset = serde_json::from_str(json).unwrap();
    assert_eq!(preset.name, "Legacy Iron Strike");
    assert_eq!(preset.extra, PresetParameters::default());
}

#[test]
fn expanded_parameters_roundtrip() {
    let mut preset = Preset {
        name: "Expanded State".to_string(),
        version: PRESET_VERSION.to_string(),
        object: Object::Tank,
        exciter: 16,
        size: 7.5,
        rust: 4.0,
        damage: 8.0,
        drive: 4.5,
        output: 3.0,
        width: 2.0,
        body: 4.0,
        extra: PresetParameters::default(),
    };
    preset.extra.drive_amount = 3.2;
    preset.extra.space_mode = 2;
    preset.extra.space_amount = 0.8;
    preset.extra.analog_ceiling = 0.7;
    preset.extra.diode_softness = 0.9;

    let path = unique_temp_path("expanded_roundtrip");
    preset.save(&path).unwrap();
    let loaded = Preset::load(&path).unwrap();
    assert_eq!(preset, loaded);

    let params = loaded.into_params();
    assert_eq!(params.drive.value(), 4.5);
    assert_eq!(params.width.value(), 2.0);
    assert_eq!(params.space_mode.value(), 2);
    assert_eq!(params.space_amount.value(), 0.8);
    assert_eq!(params.analog_ceiling.value(), 0.7);
    assert_eq!(params.diode_softness.value(), 0.9);
    let _ = fs::remove_file(path);
}
