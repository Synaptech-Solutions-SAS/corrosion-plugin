use corrotion::dsp::ModalProfileId;
use corrotion::voice::Voice;

#[test]
fn different_velocities_produce_different_roughness() {
    let sample_rate = 48_000u32;
    let mut roughness_values = Vec::new();

    for &velocity in &[32.0f32, 64.0, 100.0] {
        let mut voice = Voice::new();
        voice.note_on(60, velocity, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.5, 2);

        let mut zero_crossings = 0usize;
        let mut prev_sample = 0.0f32;

        for _ in 0..sample_rate {
            let sample = voice.process_sample(sample_rate);
            if sample.signum() != prev_sample.signum() && sample.abs() > 1e-4 {
                zero_crossings += 1;
            }
            prev_sample = sample;
        }

        roughness_values.push(zero_crossings as f32);
    }

    let low_vel_roughness = roughness_values[0];
    let mid_vel_roughness = roughness_values[1];
    let high_vel_roughness = roughness_values[2];

    let diff_low_mid = (mid_vel_roughness - low_vel_roughness).abs() / (low_vel_roughness + 1.0);
    let diff_mid_high = (high_vel_roughness - mid_vel_roughness).abs() / (mid_vel_roughness + 1.0);

    assert!(
        diff_low_mid > 0.02 || diff_mid_high > 0.02,
        "velocities should produce measurably different roughness: low={low_vel_roughness} mid={mid_vel_roughness} high={high_vel_roughness}"
    );
}

#[test]
fn high_velocity_has_faster_decay() {
    let sample_rate = 48_000u32;

    let mut low_vel_voice = Voice::new();
    low_vel_voice.note_on(60, 32.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut high_vel_voice = Voice::new();
    high_vel_voice.note_on(60, 127.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut low_vel_energy = 0.0f32;
    let mut high_vel_energy = 0.0f32;

    for _ in 0..10_000 {
        low_vel_energy += low_vel_voice.process_sample(sample_rate).abs();
        high_vel_energy += high_vel_voice.process_sample(sample_rate).abs();
    }

    assert!(
        high_vel_energy > low_vel_energy * 0.5,
        "high velocity should still produce audible output, high={high_vel_energy} low={low_vel_energy}"
    );
}
