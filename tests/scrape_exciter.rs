use corrotion::dsp::ModalProfileId;
use corrotion::voice::Voice;

#[test]
fn scrape_exciter_produces_sustained_output() {
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
        "scrape exciter should sustain: early={early_energy}, late={late_energy}"
    );
}

#[test]
fn scrape_vs_hit_produce_different_output() {
    let sample_rate = 48_000u32;
    let frames = 10_000;

    let mut hit_voice = Voice::new();
    hit_voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);

    let mut scrape_voice = Voice::new();
    scrape_voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 1);

    let mut hit_samples = Vec::new();
    let mut scrape_samples = Vec::new();

    for _ in 0..frames {
        hit_samples.push(hit_voice.process_sample(sample_rate));
        scrape_samples.push(scrape_voice.process_sample(sample_rate));
    }

    let hit_energy: f32 = hit_samples.iter().map(|s| s * s).sum();
    let scrape_energy: f32 = scrape_samples.iter().map(|s| s * s).sum();

    let ratio = if hit_energy > scrape_energy {
        hit_energy / scrape_energy.max(1e-10)
    } else {
        scrape_energy / hit_energy.max(1e-10)
    };

    assert!(
        ratio > 1.5,
        "scrape and hit should produce significantly different energy: ratio={ratio}"
    );
}
