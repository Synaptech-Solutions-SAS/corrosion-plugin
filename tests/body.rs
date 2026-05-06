use corrotion::dsp::BodyResonator;

#[test]
fn body_adds_low_mid_energy() {
    let mut body = BodyResonator::new();
    let sample_rate = 48_000u32;

    let input = 0.5f32;
    let dry = input;
    let wet = body.process_sample(input, sample_rate, 0.7);

    assert!(
        wet.abs() > dry.abs(),
        "body should add energy, dry={dry} wet={wet}"
    );
}

#[test]
fn body_zero_amount_passes_through() {
    let mut body = BodyResonator::new();
    let sample_rate = 48_000u32;

    let input = 0.5f32;
    let output = body.process_sample(input, sample_rate, 0.0);

    assert_eq!(output, 0.0, "body=0 should return 0 (no body signal)");
}

#[test]
fn body_full_amount_affects_signal() {
    let mut body = BodyResonator::new();
    let sample_rate = 48_000u32;

    let input = 0.5f32;
    let output = body.process_sample(input, sample_rate, 1.0);

    assert!(
        output.abs() > input.abs(),
        "body=1.0 should significantly affect signal"
    );
}
