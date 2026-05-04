use corrotion::voice::Voice;
use corrotion::dsp::ModalProfileId;
use corrotion::offline::render_behavior_metrics;

fn render_voice(profile: ModalProfileId, size: f32, rust: f32, damage: f32) -> Vec<f32> {
    let sample_rate = 48_000u32;
    let frames = 48_000;
    let mut voice = Voice::new();
    
    voice.note_on(60, 100.0, profile, 0, size, rust, damage, 0);
    
    let mut output = Vec::with_capacity(frames);
    for _ in 0..frames {
        output.push(voice.process_sample(sample_rate));
    }
    output
}

#[test]
fn family_differentiation_pipe_vs_plate_vs_tank() {
    let pipe = render_voice(ModalProfileId::Pipe, 1.0, 0.0, 0.0);
    let plate = render_voice(ModalProfileId::Plate, 1.0, 0.0, 0.0);
    let tank = render_voice(ModalProfileId::Tank, 1.0, 0.0, 0.0);
    
    let pipe_metrics = render_behavior_metrics(&pipe);
    let plate_metrics = render_behavior_metrics(&plate);
    let tank_metrics = render_behavior_metrics(&tank);
    
    // Plate brighter than Pipe, Pipe brighter than Tank
    assert!(plate_metrics.brightness_proxy > pipe_metrics.brightness_proxy);
    assert!(pipe_metrics.brightness_proxy > tank_metrics.brightness_proxy);
    
    // All should be different
    assert_ne!(pipe_metrics.zero_crossings, plate_metrics.zero_crossings);
    assert_ne!(pipe_metrics.zero_crossings, tank_metrics.zero_crossings);
}

#[test]
fn size_monotonicity_larger_size_lower_frequency() {
    let small = render_voice(ModalProfileId::Pipe, 0.5, 0.0, 0.0);
    let medium = render_voice(ModalProfileId::Pipe, 1.0, 0.0, 0.0);
    let large = render_voice(ModalProfileId::Pipe, 1.5, 0.0, 0.0);
    
    let small_metrics = render_behavior_metrics(&small);
    let medium_metrics = render_behavior_metrics(&medium);
    let large_metrics = render_behavior_metrics(&large);
    
    // Larger size = lower brightness (frequency)
    assert!(small_metrics.brightness_proxy > medium_metrics.brightness_proxy);
    assert!(medium_metrics.brightness_proxy > large_metrics.brightness_proxy);
}

#[test]
fn rust_monotonicity_more_rust_less_brightness() {
    let clean = render_voice(ModalProfileId::Pipe, 1.0, 0.0, 0.0);
    let medium = render_voice(ModalProfileId::Pipe, 1.0, 0.5, 0.0);
    let heavy = render_voice(ModalProfileId::Pipe, 1.0, 0.9, 0.0);
    
    let clean_metrics = render_behavior_metrics(&clean);
    let medium_metrics = render_behavior_metrics(&medium);
    let heavy_metrics = render_behavior_metrics(&heavy);
    
    // More rust = less brightness
    assert!(clean_metrics.brightness_proxy > medium_metrics.brightness_proxy);
    assert!(medium_metrics.brightness_proxy > heavy_metrics.brightness_proxy);
}

#[test]
fn damage_monotonicity_more_damage_more_roughness() {
    let clean = render_voice(ModalProfileId::Pipe, 1.0, 0.0, 0.0);
    let medium = render_voice(ModalProfileId::Pipe, 1.0, 0.0, 0.5);
    let heavy = render_voice(ModalProfileId::Pipe, 1.0, 0.0, 0.9);
    
    let clean_metrics = render_behavior_metrics(&clean);
    let medium_metrics = render_behavior_metrics(&medium);
    let heavy_metrics = render_behavior_metrics(&heavy);
    
    assert!(
        medium_metrics.roughness_proxy > clean_metrics.roughness_proxy,
        "medium roughness ({}) should be > clean roughness ({})",
        medium_metrics.roughness_proxy, clean_metrics.roughness_proxy
    );
    assert!(
        heavy_metrics.roughness_proxy > clean_metrics.roughness_proxy,
        "heavy roughness ({}) should be > clean roughness ({})",
        heavy_metrics.roughness_proxy, clean_metrics.roughness_proxy
    );
}
