use corrosion::dsp::ModalProfileId;
use corrosion::offline::render_behavior_metrics;
use corrosion::voice::Voice;

fn render_note_at_velocity(velocity: f32, frames: usize) -> f32 {
    let sample_rate = 48_000u32;
    let mut voice = Voice::new();

    voice.note_on(60, velocity, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut output = Vec::with_capacity(frames);
    for _ in 0..frames {
        output.push(voice.process_sample(sample_rate));
    }

    render_behavior_metrics(&output).brightness_proxy
}

fn level_normalize(samples: &mut [f32]) {
    let rms: f32 = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
    if rms > 1e-10 {
        let scale = 1.0 / rms;
        for s in samples.iter_mut() {
            *s *= scale;
        }
    }
}

fn render_and_normalize(velocity: f32, frames: usize) -> (f32, Vec<f32>) {
    let sample_rate = 48_000u32;
    let mut voice = Voice::new();

    voice.note_on(60, velocity, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut output = Vec::with_capacity(frames);
    for _ in 0..frames {
        output.push(voice.process_sample(sample_rate));
    }

    level_normalize(&mut output);
    let brightness = render_behavior_metrics(&output).brightness_proxy;
    (brightness, output)
}

#[test]
fn high_velocity_is_brighter_than_low_velocity() {
    let frames = 48_000;

    let (low_brightness, _) = render_and_normalize(0.2 * 127.0, frames);
    let (high_brightness, _) = render_and_normalize(1.0 * 127.0, frames);

    assert!(
        high_brightness > low_brightness * 1.01,
        "High velocity brightness ({:.6}) should be measurably > low velocity brightness ({:.6})",
        high_brightness,
        low_brightness
    );
}

#[test]
fn velocity_affects_brightness_monotonically() {
    let frames = 24_000;
    let velocities = [0.2, 0.4, 0.6, 0.8, 1.0];
    let mut brightness_values = Vec::new();

    for &vel_norm in &velocities {
        let velocity = vel_norm * 127.0;
        let brightness = render_note_at_velocity(velocity, frames);
        brightness_values.push(brightness);
    }

    let mut increasing_count = 0;
    let total_comparisons = brightness_values.len() * (brightness_values.len() - 1) / 2;

    for (i, &brightness) in brightness_values.iter().enumerate() {
        for &later_brightness in brightness_values.iter().skip(i + 1) {
            if later_brightness > brightness {
                increasing_count += 1;
            }
        }
    }

    let ratio = increasing_count as f32 / total_comparisons as f32;
    assert!(
        ratio >= 0.70,
        "Brightness should increase with velocity (got {:.0}% increasing, expected >= 70%)",
        ratio * 100.0
    );
}

#[test]
fn velocity_preserves_amplitude_scaling() {
    let sample_rate = 48_000u32;
    let frames = 100;

    let low_vel = 0.2 * 127.0;
    let high_vel = 1.0 * 127.0;

    let mut low_voice = Voice::new();
    low_voice.note_on(60, low_vel, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut high_voice = Voice::new();
    high_voice.note_on(60, high_vel, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut low_first_samples = Vec::new();
    let mut high_first_samples = Vec::new();

    for _ in 0..frames {
        low_first_samples.push(low_voice.process_sample(sample_rate));
        high_first_samples.push(high_voice.process_sample(sample_rate));
    }

    let low_peak = low_first_samples
        .iter()
        .take(20)
        .map(|s| s.abs())
        .fold(0.0f32, f32::max);
    let high_peak = high_first_samples
        .iter()
        .take(20)
        .map(|s| s.abs())
        .fold(0.0f32, f32::max);

    let ratio = high_peak / low_peak.max(1e-10);
    assert!(
        ratio > 1.0,
        "High velocity should produce higher amplitude than low velocity (ratio: {:.2})",
        ratio
    );
}

#[test]
fn velocity_affects_spectral_content() {
    let sample_rate = 48_000u32;
    let frames = 48_000;

    let mut low_voice = Voice::new();
    low_voice.note_on(60, 0.2 * 127.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut high_voice = Voice::new();
    high_voice.note_on(60, 1.0 * 127.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut low_samples = Vec::new();
    let mut high_samples = Vec::new();

    for _ in 0..frames {
        low_samples.push(low_voice.process_sample(sample_rate));
        high_samples.push(high_voice.process_sample(sample_rate));
    }

    level_normalize(&mut low_samples);
    level_normalize(&mut high_samples);

    let low_brightness = render_behavior_metrics(&low_samples).brightness_proxy;
    let high_brightness = render_behavior_metrics(&high_samples).brightness_proxy;

    assert!(
        high_brightness > low_brightness,
        "High velocity should produce brighter spectral content (high: {:.6}, low: {:.6})",
        high_brightness,
        low_brightness
    );
}
