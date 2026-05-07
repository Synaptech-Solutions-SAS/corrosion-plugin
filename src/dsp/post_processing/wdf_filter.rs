//! WDF-style ladder filter used in the post chain.
//!
//! This stage approximates a Moog-like four-pole ladder with a soft nonlinear
//! saturation curve and lightweight resonance compensation.
#[derive(Clone, Debug)]
/// Four-stage ladder filter approximation with resonance and saturation.
pub struct WdfLadderFilter {
    cutoff_hz: f32,
    resonance: f32,
    component_tolerance: f32,

    // Filter state (4 stages)
    y: [f32; 4],

    // Cached coefficients
    g: f32,
    g_comp: f32,
    sample_rate: f32,
}

impl WdfLadderFilter {
    /// Creates the filter with a wide-open cutoff and zero resonance.
    pub fn new() -> Self {
        Self {
            cutoff_hz: 20000.0,
            resonance: 0.0,
            component_tolerance: 0.0,
            y: [0.0; 4],
            g: 0.0,
            g_comp: 0.0,
            sample_rate: 48000.0,
        }
    }

    /// Sets cutoff, resonance, and component tolerance.
    pub fn set_parameters(&mut self, cutoff_hz: f32, resonance: f32, tolerance: f32) {
        self.cutoff_hz = cutoff_hz.clamp(20.0, 20000.0);
        self.resonance = resonance.clamp(0.0, 1.0);
        self.component_tolerance = tolerance.clamp(0.0, 1.0);
        self.update_coefficients();
    }

    /// Updates the coefficient cache for a new sample rate.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.update_coefficients();
    }

    fn update_coefficients(&mut self) {
        let wc = 2.0 * std::f32::consts::PI * self.cutoff_hz / self.sample_rate;
        self.g = wc.tan();

        // Compensation gain for resonance
        self.g_comp = 1.0 / (1.0 + self.resonance * 3.5);
    }

    /// Processes one mono sample through the ladder stages.
    pub fn process(&mut self, input: f32) -> f32 {
        // Apply component tolerance as subtle noise/jitter
        let tolerance_jitter = if self.component_tolerance > 0.0 {
            let noise = (self.y[0] * 43_758.547).fract() * 2.0 - 1.0;
            noise * self.component_tolerance * 0.001
        } else {
            0.0
        };

        let x = input + tolerance_jitter;

        // Feedback path with resonance
        let feedback = self.y[3] * self.resonance * 4.0;
        let x_in = x - feedback;

        // 4-pole ladder stages with nonlinear saturation
        let mut stage_in = x_in;
        for i in 0..4 {
            let g = self.g;
            // Trapezoidal integration: y[n] = (g * x + s) / (1 + g)
            // where s is the state
            let s = self.y[i] - g * self.y[i]; // equivalent to (1-g)*y[i-1]
            let y_new = (g * stage_in + s) / (1.0 + g);

            // Nonlinear saturation (transistor behavior)
            let saturated = Self::transistor_saturate(y_new);

            self.y[i] = saturated;
            stage_in = saturated;
        }

        // Output with compensation
        self.y[3] * self.g_comp
    }

    fn transistor_saturate(x: f32) -> f32 {
        // Soft clipping approximation of transistor junction
        let threshold = 0.7;
        if x.abs() < threshold {
            x
        } else {
            let sign = x.signum();
            let excess = x.abs() - threshold;
            let compressed = threshold + (1.0 - threshold) * (1.0 - (-excess * 2.0).exp());
            sign * compressed.min(1.0)
        }
    }

    /// Clears the filter state history.
    pub fn reset(&mut self) {
        self.y = [0.0; 4];
    }
}

impl Default for WdfLadderFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_processes_without_nan() {
        let mut filter = WdfLadderFilter::new();
        filter.set_sample_rate(48000.0);
        filter.set_parameters(1000.0, 0.5, 0.0);

        for i in 0..100 {
            let input = (i as f32 * 0.01).sin();
            let output = filter.process(input);
            assert!(output.is_finite(), "Output should be finite");
        }
    }

    #[test]
    fn zero_input_produces_zero() {
        let mut filter = WdfLadderFilter::new();
        filter.set_sample_rate(48000.0);

        for _ in 0..50 {
            let output = filter.process(0.0);
            assert!(
                output.abs() < 1e-6,
                "Zero input should produce near-zero output"
            );
        }
    }

    #[test]
    fn low_cutoff_attenuates_high_freqs() {
        let mut filter = WdfLadderFilter::new();
        filter.set_sample_rate(48000.0);
        filter.set_parameters(100.0, 0.0, 0.0);

        // High frequency input
        let mut high_energy = 0.0f32;
        for i in 0..480 {
            let input = (i as f32 * 0.5).sin(); // High freq
            high_energy += filter.process(input).abs();
        }

        filter.reset();
        filter.set_parameters(5000.0, 0.0, 0.0);

        let mut low_energy = 0.0f32;
        for i in 0..480 {
            let input = (i as f32 * 0.5).sin();
            low_energy += filter.process(input).abs();
        }

        assert!(high_energy < low_energy, "Low cutoff should attenuate more");
    }
}
