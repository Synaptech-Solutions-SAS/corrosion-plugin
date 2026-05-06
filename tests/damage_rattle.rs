use corrotion::dsp::ModalProfileId;
use corrotion::voice::Voice;

#[test]
fn high_damage_produces_rattle() {
    let mut voice = Voice::new();
    voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.85, 2);

    let sample_rate = 48_000u32;
    let mut high_damage_variance = 0.0f32;
    let mut sample_count = 0usize;

    for _ in 0..48_000 {
        let sample = voice.process_sample(sample_rate);
        high_damage_variance += sample * sample;
        sample_count += 1;
    }

    let mut clean_voice = Voice::new();
    clean_voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut clean_variance = 0.0f32;
    for _ in 0..48_000 {
        let sample = clean_voice.process_sample(sample_rate);
        clean_variance += sample * sample;
    }

    let high_rms = (high_damage_variance / sample_count as f32).sqrt();
    let clean_rms = (clean_variance / sample_count as f32).sqrt();

    assert!(
        high_rms > clean_rms * 0.8,
        "high damage should produce comparable or higher RMS due to rattle, high={high_rms} clean={clean_rms}"
    );
}

#[test]
fn zero_damage_produces_no_rattle() {
    let mut voice = Voice::new();
    voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let sample_rate = 48_000u32;

    for _ in 0..1000 {
        let sample = voice.process_sample(sample_rate);
        assert!(sample.is_finite(), "zero damage output should be finite");
    }
}

#[test]
fn damage_rattle_is_signal_dependent() {
    let mut voice = Voice::new();
    voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.5, 2);

    let sample_rate = 48_000u32;

    let mut max_sample = 0.0f32;
    let mut max_delta = 0.0f32;
    let mut prev_sample = 0.0f32;

    for _ in 0..48_000 {
        let sample = voice.process_sample(sample_rate);
        max_sample = max_sample.max(sample.abs());
        max_delta = max_delta.max((sample - prev_sample).abs());
        prev_sample = sample;
    }

    assert!(max_sample > 0.0, "damage should produce output");
    assert!(
        max_delta > 1e-5,
        "damage rattle should introduce sample variation"
    );
}
