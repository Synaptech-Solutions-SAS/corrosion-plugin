use corrotion::presets::{Preset, PRESET_VERSION};
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
    };

    let path = unique_temp_path("roundtrip");
    preset.save(&path).unwrap();
    let loaded = Preset::load(&path).unwrap();
    assert_eq!(preset, loaded);
    let _ = fs::remove_file(path);
}
