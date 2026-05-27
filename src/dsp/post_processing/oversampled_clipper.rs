//! Oversampled final clipper for the post chain.
//!
//! This stage approximates a true-peak limiter using 16x oversampling and a
//! soft diode-style transfer curve.
/// Final soft clipper with configurable oversampled processing.
pub struct OversampledClipper {
    ceiling: f32,
    softness: f32,
    oversample_factor: usize,

    // Previous input per channel, used to interpolate the upsampled grid.
    prev_input: [f32; 2],

    sample_rate: f32,
}

impl OversampledClipper {
    /// Creates the clipper with a conservative ceiling and 16x oversampling.
    pub fn new() -> Self {
        Self {
            ceiling: 0.9661,
            softness: 0.5,
            oversample_factor: 16,
            prev_input: [0.0; 2],
            sample_rate: 48000.0,
        }
    }

    /// Sets the clipping ceiling and knee softness.
    pub fn set_parameters(&mut self, ceiling: f32, softness: f32) {
        self.ceiling = ceiling.clamp(0.5, 1.0);
        self.softness = softness.clamp(0.0, 1.0);
    }

    /// Sets the oversampling factor (1, 4, 8, or 16).
    pub fn set_oversample_factor(&mut self, factor: usize) {
        self.oversample_factor = factor.clamp(1, 16);
    }

    /// Updates the internal rate used for reporting and future expansion.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    /// Processes one mono sample through the oversampled limiter.
    pub fn process(&mut self, input: f32) -> f32 {
        self.process_channel(input, 0)
    }

    /// Oversamples one channel: linear-interpolate from the previous input up to
    /// the new input, clip each high-rate sample, then box-filter decimate.
    ///
    /// Interpolating across the previous/current input (rather than holding a
    /// constant) is what makes the factor matter: clipping a rising ramp at the
    /// high rate and averaging the result band-limits the harmonics the clip
    /// generates, so higher factors fold less energy back as aliasing.
    fn process_channel(&mut self, input: f32, channel: usize) -> f32 {
        let os_factor = self.oversample_factor;
        let prev = self.prev_input[channel];
        self.prev_input[channel] = input;

        if os_factor <= 1 {
            return self.diode_clip(input);
        }

        let mut sum = 0.0f32;
        for k in 1..=os_factor {
            let frac = k as f32 / os_factor as f32;
            let interpolated = prev + (input - prev) * frac;
            sum += self.diode_clip(interpolated);
        }

        sum / os_factor as f32
    }

    fn diode_clip(&self, x: f32) -> f32 {
        let threshold = self.ceiling;
        let abs_x = x.abs();

        if abs_x <= threshold {
            x
        } else {
            let sign = x.signum();
            let excess = abs_x - threshold;
            let knee = 2.0 + self.softness * 3.0;
            let compressed = excess / (1.0 + excess * knee);
            sign * (threshold + compressed * (1.0 - threshold) * 0.1)
        }
    }

    /// Processes a stereo frame by clipping each channel independently.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process_channel(left, 0), self.process_channel(right, 1))
    }

    /// Clears oversampling state buffers.
    pub fn reset(&mut self) {
        self.prev_input = [0.0; 2];
    }
}

impl Default for OversampledClipper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_exceeds_ceiling() {
        let mut clipper = OversampledClipper::new();
        clipper.set_sample_rate(48000.0);
        clipper.set_parameters(0.8, 0.0);

        for _ in 0..100 {
            let input = 2.0;
            let output = clipper.process(input);
            assert!(
                output.abs() <= 0.81,
                "Output should not exceed ceiling significantly"
            );
        }
    }

    #[test]
    fn preserves_low_levels() {
        let mut clipper = OversampledClipper::new();
        clipper.set_sample_rate(48000.0);
        clipper.set_parameters(0.9661, 0.0);

        for i in 0..100 {
            let input = (i as f32 * 0.01).sin() * 0.3;
            let output = clipper.process(input);
            assert!(
                (output - input).abs() < 0.01,
                "Low levels should be preserved"
            );
        }
    }

    #[test]
    fn oversampling_factor_changes_output_on_transients() {
        // Drive both clippers with the same signal that swings past the ceiling.
        // With real interpolation across inputs, a higher factor band-limits the
        // clip differently, so the outputs must diverge on the transients.
        let inputs = [0.0, 2.0, -2.0, 1.5, -1.8, 0.4, -0.4, 2.5];

        let mut c1 = OversampledClipper::new();
        c1.set_parameters(0.8, 0.5);
        c1.set_oversample_factor(1);

        let mut c16 = OversampledClipper::new();
        c16.set_parameters(0.8, 0.5);
        c16.set_oversample_factor(16);

        let mut total_diff = 0.0f32;
        for &x in inputs.iter() {
            total_diff += (c1.process(x) - c16.process(x)).abs();
        }

        assert!(
            total_diff > 0.001,
            "1x and 16x oversampling must differ on transients, diff={total_diff}"
        );
    }

    #[test]
    fn stereo_channels_have_independent_state() {
        // The two channels must not share interpolation state.
        let mut clipper = OversampledClipper::new();
        clipper.set_parameters(0.8, 0.5);
        clipper.set_oversample_factor(8);

        let (l1, r1) = clipper.process_stereo(2.0, -2.0);
        assert!(l1.is_finite() && r1.is_finite());
        // Left rising from 0, right falling from 0 — opposite-signed inputs
        // should yield opposite-signed clipped outputs.
        assert!(l1 > 0.0 && r1 < 0.0, "channels must track their own input sign");
    }

    #[test]
    fn no_nan_produced() {
        let mut clipper = OversampledClipper::new();
        clipper.set_sample_rate(48000.0);

        for i in 0..1000 {
            let input = (i as f32 * 0.1).sin() * 2.0;
            let output = clipper.process(input);
            assert!(output.is_finite(), "Output should be finite");
        }
    }
}
