use corrotion::dsp::ModalProfileId;
use corrotion::voice::{VoiceManager, MAX_VOICES};

#[test]
fn rapid_width_and_body_changes_stay_finite() {
    let sample_rate = 48_000u32;
    let mut manager = VoiceManager::new();

    for i in 0..MAX_VOICES {
        manager.note_on(48 + i as u8, 100.0, ModalProfileId::Pipe, 1.0, 0.0, 0.0, 0);
    }

    let mut max_peak = 0.0f32;
    for frame in 0..48_000 {
        let width = (frame as f32 / 1000.0).sin() * 0.5 + 0.5;
        let (left, right) = manager.process_sample_stereo(sample_rate, width);

        assert!(left.is_finite(), "non-finite left at frame {frame}");
        assert!(right.is_finite(), "non-finite right at frame {frame}");

        max_peak = max_peak.max(left.abs()).max(right.abs());
    }

    assert!(max_peak > 0.001, "output should be audible");
}

#[test]
fn body_parameter_does_not_create_dc_offset() {
    let sample_rate = 48_000u32;
    let mut manager = VoiceManager::new();
    manager.note_on(60, 100.0, ModalProfileId::Tank, 1.0, 0.0, 0.0, 0);

    let mut sum = 0.0f32;
    for frame in 0..10_000 {
        let body = (frame as f32 / 500.0).sin() * 0.5 + 0.5;
        let (left, right) = manager.process_sample_stereo(sample_rate, body);
        sum += left + right;
    }

    let mean = sum / (10_000.0 * 2.0);
    assert!(mean.abs() < 0.01, "DC offset should be negligible, got {mean}");
}
