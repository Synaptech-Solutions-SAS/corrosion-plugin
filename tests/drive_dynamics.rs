use corrotion::apply_drive;

#[test]
fn drive_preserves_dynamics() {
    let sample_rate = 48_000u32;
    let mut crest_factors = Vec::new();

    for &drive in &[0.0f32, 0.5, 0.85] {
        let mut peak = 0.0f32;
        let mut sum_squares = 0.0f32;
        let mut count = 0usize;

        for i in 0..sample_rate {
            let t = i as f32 / sample_rate as f32;
            let input = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.8;
            let output = apply_drive(input, drive);

            peak = peak.max(output.abs());
            sum_squares += output * output;
            count += 1;
        }

        let rms = (sum_squares / count as f32).sqrt();
        let crest_factor = if rms > 1e-6 { peak / rms } else { 0.0 };
        crest_factors.push(crest_factor);
    }

    let high_drive_crest = crest_factors[2];
    let no_drive_crest = crest_factors[0];

    assert!(
        high_drive_crest >= no_drive_crest * 0.5,
        "high drive crest factor ({high_drive_crest}) should preserve at least 50% of no-drive crest ({no_drive_crest})"
    );
}

#[test]
fn drive_output_bounded() {
    for &drive in &[0.0f32, 0.5, 1.0] {
        for i in 0..1000 {
            let input = (i as f32 / 100.0).sin() * 0.9;
            let output = apply_drive(input, drive);
            assert!(
                output.abs() <= 1.5,
                "drive output should be bounded, got {output} with drive={drive}"
            );
        }
    }
}

#[test]
fn zero_drive_is_unity() {
    for i in 0..100 {
        let input = (i as f32 / 10.0).sin() * 0.9;
        let output = apply_drive(input, 0.0);
        assert_eq!(
            output, input,
            "zero drive should pass signal through unchanged"
        );
    }
}
