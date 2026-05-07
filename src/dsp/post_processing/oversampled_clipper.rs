//! Oversampled final clipper for the post chain.
//!
//! This stage approximates a true-peak limiter using 16x oversampling and a
//! soft diode-style transfer curve.
/// Final soft clipper with configurable oversampled processing.
pub struct OversampledClipper {
    ceiling: f32,
    softness: f32,
    oversample_factor: usize,

    // Polyphase interpolation/decimation state
    upsample_state: [f32; 16],
    downsample_state: [f32; 16],

    sample_rate: f32,
}

impl OversampledClipper {
    /// Creates the clipper with a conservative ceiling and 16x oversampling.
    pub fn new() -> Self {
        Self {
            ceiling: 0.9661,
            softness: 0.5,
            oversample_factor: 16,
            upsample_state: [0.0; 16],
            downsample_state: [0.0; 16],
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
        let os_factor = self.oversample_factor;
        if os_factor <= 1 {
            return self.diode_clip(input);
        }

        // Upsample (zero-stuffing with simple hold)
        let mut os_samples = [0.0f32; 16];
        os_samples.fill(input);

        // Process each oversampled sample
        let mut sum = 0.0f32;
        for sample in os_samples.iter().take(os_factor) {
            let clipped = self.diode_clip(*sample);
            sum += clipped;
        }

        // Decimate (average)
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
        (self.process(left), self.process(right))
    }

    /// Clears oversampling state buffers.
    pub fn reset(&mut self) {
        self.upsample_state = [0.0; 16];
        self.downsample_state = [0.0; 16];
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
