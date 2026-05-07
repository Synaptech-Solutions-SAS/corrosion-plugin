use corrotion::dsp::ModalProfileId;
use corrotion::voice::{VoiceManager, MAX_VOICES};
use corrotion::Object;

#[test]
fn rapid_width_and_body_changes_stay_finite() {
    let sample_rate = 48_000u32;
    let mut manager = VoiceManager::new();

    for i in 0..MAX_VOICES {
        manager.note_on(48 + i as u8, 100.0, ModalProfileId::Pipe, 1.0, 0.0, 0.0, 2);
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
    manager.note_on(60, 100.0, ModalProfileId::Tank, 1.0, 0.0, 0.0, 2);

    let mut sum = 0.0f32;
    for frame in 0..10_000 {
        let body = (frame as f32 / 500.0).sin() * 0.5 + 0.5;
        let (left, right) = manager.process_sample_stereo(sample_rate, body);
        sum += left + right;
    }

    let mean = sum / (10_000.0 * 2.0);
    assert!(
        mean.abs() < 0.01,
        "DC offset should be negligible, got {mean}"
    );
}

#[test]
fn exciter_and_object_matrix_stays_finite() {
    let sample_rate = 48_000u32;

    for exciter in 1..=16 {
        for object in 0..=8 {
            let profile = match Object::from_int(object) {
                Object::Pipe => ModalProfileId::Pipe,
                Object::Plate => ModalProfileId::Plate,
                Object::Tank => ModalProfileId::Tank,
                Object::Chain => ModalProfileId::Chain,
                Object::IBeam => ModalProfileId::IBeam,
                Object::TautCable => ModalProfileId::TautCable,
                Object::CoilSpring => ModalProfileId::CoilSpring,
                Object::SheetMetal => ModalProfileId::SheetMetal,
                Object::IndustrialCog => ModalProfileId::IndustrialCog,
            };

            let mut manager = VoiceManager::new();
            manager.note_on(60, 110.0, profile, 1.0, 0.25, 0.25, exciter);

            for frame in 0..2_048 {
                let width = ((frame as f32) * 0.01).sin();
                let (left, right) = manager.process_sample_stereo(sample_rate, width);
                assert!(
                    left.is_finite(),
                    "non-finite left for exciter={exciter} object={object}"
                );
                assert!(
                    right.is_finite(),
                    "non-finite right for exciter={exciter} object={object}"
                );
            }
        }
    }
}
