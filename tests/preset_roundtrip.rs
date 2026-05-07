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
        exciter: 2,
        size: 1.0,
        rust: 0.3,
        damage: 0.1,
        drive: 0.5,
        output: 1.0,
        width: 0.5,
        body: 0.2,
        quality_mode: 1,
        complex_algo: 0,
        extra: PresetParameters::default(),
    };

    let path = unique_temp_path("roundtrip");
    preset.save(&path).unwrap();
    let loaded = Preset::load(&path).unwrap();
    assert_eq!(preset, loaded);
    let _ = fs::remove_file(path);
}

#[test]
fn version_1_preset_loads_with_default_extended_parameters() {
    let json = r#"
    {
        "name": "Iron Strike",
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
    assert_eq!(preset.name, "Iron Strike");
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
        quality_mode: 3,
        complex_algo: 1,
        extra: PresetParameters::default(),
    };
    preset.extra.drive_amount = 3.2;
    preset.extra.space_mode = 2;
    preset.extra.space_amount = 0.8;
    preset.extra.analog_ceiling = 0.7;
    preset.extra.diode_softness = 0.9;
    preset.extra.hand_mass = 2.4;
    preset.extra.pipe_ring_decay = 0.991;
    preset.extra.ridge_spacing = 0.08;
    preset.extra.flow_rate = 2.2;

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
    assert_eq!(params.complex_algo.value(), 1);
    assert_eq!(params.hand_mass.value(), 2.4);
    assert_eq!(params.pipe_ring_decay.value(), 0.991);
    assert_eq!(params.ridge_spacing.value(), 0.08);
    assert_eq!(params.flow_rate.value(), 2.2);
    let _ = fs::remove_file(path);
}

#[test]
fn invalid_preset_values_are_sanitized_on_load() {
    let json = r#"
    {
        "name": "Broken State",
        "version": "999",
        "object": "Pipe",
        "exciter": 999,
        "size": -100.0,
        "rust": 999.0,
        "damage": -5.0,
        "drive": 999.0,
        "output": 999.0,
        "width": -999.0,
        "body": 999.0,
        "extra": {
            "ui_scale": 99,
            "loop_mode": 99,
            "loop_start_stage": -99,
            "loop_end_stage": 99,
            "space_mode": 99,
            "filter_cutoff": -10.0,
            "analog_ceiling": 9.0,
            "diode_softness": -3.0
        }
    }
    "#;

    let path = unique_temp_path("sanitized_load");
    fs::write(&path, json).unwrap();
    let loaded = Preset::load(&path).unwrap();
    let params = loaded.into_params();

    assert_eq!(params.exciter.value(), 2);
    assert_eq!(params.size.value(), 0.05);
    assert_eq!(params.rust.value(), 5.0);
    assert_eq!(params.damage.value(), 0.0);
    assert_eq!(params.drive.value(), 5.0);
    assert_eq!(params.output.value(), 100.0);
    assert_eq!(params.width.value(), -2.0);
    assert_eq!(params.body.value(), 5.0);
    assert_eq!(params.ui_scale.value(), 4);
    assert_eq!(params.loop_mode.value(), 2);
    assert_eq!(params.loop_start_stage.value(), 0);
    assert_eq!(params.loop_end_stage.value(), 5);
    assert_eq!(params.space_mode.value(), 3);
    assert_eq!(params.filter_cutoff.value(), 20.0);
    assert_eq!(params.analog_ceiling.value(), 1.0);
    assert_eq!(params.diode_softness.value(), 0.0);
    assert_eq!(params.quality_mode.value(), 1);

    let _ = fs::remove_file(path);
}

#[test]
fn quality_mode_preset_roundtrip() {
    let preset = Preset {
        name: "Quality Test".to_string(),
        version: PRESET_VERSION.to_string(),
        object: Object::Plate,
        exciter: 5,
        size: 1.5,
        rust: 0.5,
        damage: 0.5,
        drive: 0.5,
        output: 1.0,
        width: 0.0,
        body: 0.0,
        quality_mode: 0,
        complex_algo: 1,
        extra: PresetParameters::default(),
    };

    let path = unique_temp_path("quality_roundtrip");
    preset.save(&path).unwrap();
    let loaded = Preset::load(&path).unwrap();
    assert_eq!(preset.quality_mode, loaded.quality_mode);
    assert_eq!(preset.complex_algo, loaded.complex_algo);

    let params = loaded.into_params();
    assert_eq!(params.quality_mode.value(), 0);
    assert_eq!(params.complex_algo.value(), 1);
    let _ = fs::remove_file(path);
}
