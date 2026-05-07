use corrosion::dsp::ModalProfileId;
use corrosion::voice::Voice;

#[test]
fn bow_exciter_produces_sustained_output() {
    let sample_rate = 48_000u32;
    let mut voice = Voice::new();
    voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 1);

    let mut samples = Vec::new();
    for _ in 0..(sample_rate * 2) {
        samples.push(voice.process_sample(sample_rate));
    }

    assert!(samples.iter().all(|s| s.is_finite()));

    let early_energy: f32 = samples.iter().take(1000).map(|s| s.abs()).sum();
    let late_energy: f32 = samples.iter().skip(1000).take(1000).map(|s| s.abs()).sum();

    assert!(
        late_energy > early_energy * 0.1,
        "bow exciter should sustain: early={early_energy}, late={late_energy}"
    );
}

#[test]
fn bow_and_hand_strike_produce_different_output() {
    let sample_rate = 48_000u32;
    let frames = 10_000;

    let mut hand_strike_voice = Voice::new();
    hand_strike_voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 2);

    let mut bow_voice = Voice::new();
    bow_voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 1);

    let mut hand_strike_samples = Vec::new();
    let mut bow_samples = Vec::new();

    for _ in 0..frames {
        hand_strike_samples.push(hand_strike_voice.process_sample(sample_rate));
        bow_samples.push(bow_voice.process_sample(sample_rate));
    }

    let hand_strike_energy: f32 = hand_strike_samples.iter().map(|s| s * s).sum();
    let bow_energy: f32 = bow_samples.iter().map(|s| s * s).sum();

    let ratio = if hand_strike_energy > bow_energy {
        hand_strike_energy / bow_energy.max(1e-10)
    } else {
        bow_energy / hand_strike_energy.max(1e-10)
    };

    assert!(
        ratio > 1.5,
        "bow and hand strike should produce significantly different energy: ratio={ratio}"
    );
}
