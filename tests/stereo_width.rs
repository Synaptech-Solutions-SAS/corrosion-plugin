use corrotion::dsp::{ModalProfileId, ModalResonator, ResonatorCore};

#[test]
fn width_zero_is_mono() {
    let mut resonator = ModalResonator::with_profile(ModalProfileId::Pipe);
    let sample_rate = 48_000u32;

    let mut mono_energy = 0.0f32;
    for _ in 0..100 {
        let (left, right) = resonator.process_sample_stereo(1.0, sample_rate, 0.0);
        approx_eq(left, right, 1.0e-6);
        mono_energy += left.abs();
    }

    assert!(mono_energy > 0.0, "Width=0 mono output should be audible");
}

#[test]
fn width_one_produces_stereo() {
    let mut resonator = ModalResonator::with_profile(ModalProfileId::Pipe);
    let sample_rate = 48_000u32;
    
    let mut left_sum = 0.0f32;
    let mut right_sum = 0.0f32;
    
    for _ in 0..100 {
        let (left, right) = resonator.process_sample_stereo(1.0, sample_rate, 1.0);
        left_sum += left.abs();
        right_sum += right.abs();
    }
    
    let correlation = (left_sum - right_sum).abs() / (left_sum + right_sum + 1e-6);
    
    assert!(
        correlation < 0.3,
        "Width=1 should produce stereo with correlated but different channels, correlation={correlation}"
    );
}

fn approx_eq(left: f32, right: f32, epsilon: f32) {
    assert!(
        (left - right).abs() <= epsilon,
        "left={left} right={right} epsilon={epsilon}"
    );
}
