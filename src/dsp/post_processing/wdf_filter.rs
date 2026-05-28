//! Four-pole ladder-style filter used in the post chain.
//!
//! Despite the historical `WdfLadderFilter` type name (kept for ABI/preset
//! stability), this stage is **not** a Wave Digital Filter circuit solver
//! and does not perform iterative Newton-Raphson on a non-linear circuit.
//! It is a much cheaper approximation: four cascaded one-pole trapezoidal
//! integrators with transistor-style soft-saturation between stages, plus a
//! standard feedback term for resonance — i.e. a Moog-flavored topology
//! that is computationally similar to a TPT (topology-preserving transform)
//! ladder, not an electrical-domain WDF.
//!
//! See `docs/detailed-specs/post-processing.md` for the contractual
//! intended response; this file describes the shipped approximation
//! (P1.7 decision banner in `docs/ARCHITECTURE.md` §9).
#[derive(Clone, Debug)]
/// Four-pole ladder-style filter approximation with resonance and saturation.
/// Not a circuit-solver WDF — see the module docstring above.
pub struct WdfLadderFilter {
    /// Smoothed (audible) cutoff used to compute `g`. Lags `target_cutoff_hz`
    /// by a few ms so rapid host automation never zippers.
    cutoff_hz: f32,
    target_cutoff_hz: f32,
    /// Smoothed (audible) resonance, paired with `cutoff_hz`.
    resonance: f32,
    target_resonance: f32,
    component_tolerance: f32,

    // Filter state (4 stages)
    y: [f32; 4],

    // Cached coefficients
    g: f32,
    g_comp: f32,
    sample_rate: f32,
    /// Per-sample one-pole smoother coefficient (≈1 - exp(-1/(tau·sr))).
    smoothing_coeff: f32,
    /// Whether at least one `set_parameters` call has happened since
    /// new/reset. The first call snaps; later calls smooth.
    smoothing_primed: bool,
}

impl WdfLadderFilter {
    /// One-pole smoother time constant in seconds. Slow enough to kill zipper
    /// noise without making knob movement feel laggy.
    const SMOOTHING_TAU_SECONDS: f32 = 0.02;

    /// Creates the filter with a wide-open cutoff and zero resonance.
    pub fn new() -> Self {
        let mut filter = Self {
            cutoff_hz: 20000.0,
            target_cutoff_hz: 20000.0,
            resonance: 0.0,
            target_resonance: 0.0,
            component_tolerance: 0.0,
            y: [0.0; 4],
            g: 0.0,
            g_comp: 0.0,
            sample_rate: 48000.0,
            smoothing_coeff: 0.0,
            smoothing_primed: false,
        };
        filter.recompute_smoothing_coeff();
        filter.update_coefficients();
        filter
    }

    /// Sets cutoff, resonance, and component tolerance.
    ///
    /// Cutoff and resonance update *targets*; the smoothers track them at
    /// audio rate inside `process()`. Component tolerance jumps immediately
    /// because it is a noise scale, not a coefficient. The first call after
    /// `new`/`reset` snaps so initial setup reaches the requested response
    /// without an audible ramp.
    pub fn set_parameters(&mut self, cutoff_hz: f32, resonance: f32, tolerance: f32) {
        self.target_cutoff_hz = cutoff_hz.clamp(20.0, 20000.0);
        self.target_resonance = resonance.clamp(0.0, 1.0);
        self.component_tolerance = tolerance.clamp(0.0, 1.0);
        if !self.smoothing_primed {
            self.smoothing_primed = true;
            self.settle();
        }
    }

    /// Snap the cutoff/resonance smoothers immediately to their targets.
    ///
    /// Used after a `reset()` or in static-config contexts (tests, offline
    /// renders) where the audio-rate ramp would obscure parameter response.
    pub fn settle(&mut self) {
        self.cutoff_hz = self.target_cutoff_hz;
        self.resonance = self.target_resonance;
        self.update_coefficients();
    }

    /// Updates the coefficient cache for a new sample rate.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.recompute_smoothing_coeff();
        self.update_coefficients();
    }

    fn recompute_smoothing_coeff(&mut self) {
        let sr = self.sample_rate.max(1.0);
        self.smoothing_coeff = 1.0 - (-1.0 / (Self::SMOOTHING_TAU_SECONDS * sr)).exp();
    }

    fn update_coefficients(&mut self) {
        let wc = 2.0 * std::f32::consts::PI * self.cutoff_hz / self.sample_rate;
        self.g = wc.tan();

        // Compensation gain for resonance
        self.g_comp = 1.0 / (1.0 + self.resonance * 3.5);
    }

    /// Processes one mono sample through the ladder stages.
    pub fn process(&mut self, input: f32) -> f32 {
        // Advance the cutoff/resonance smoothers and rebuild coefficients if
        // either drifted enough to matter. The `0.005`/`5e-4` thresholds skip
        // tan()/divide work on every steady-state sample without making the
        // ramp audibly stepwise.
        let cutoff_changed = (self.target_cutoff_hz - self.cutoff_hz).abs() > 0.005;
        let resonance_changed = (self.target_resonance - self.resonance).abs() > 5e-4;
        if cutoff_changed || resonance_changed {
            self.cutoff_hz +=
                (self.target_cutoff_hz - self.cutoff_hz) * self.smoothing_coeff;
            self.resonance +=
                (self.target_resonance - self.resonance) * self.smoothing_coeff;
            self.update_coefficients();
        }

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
        // Re-prime so the next `set_parameters` snaps to the requested values.
        self.smoothing_primed = false;
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
    fn mid_stream_cutoff_change_is_smoothed() {
        // After priming, an abrupt cutoff change must not appear as a single-
        // sample step in the audible response. We compare two filters: one
        // that gets the new cutoff applied with smoothing, and one that
        // settles immediately. The smoothed filter must trail the snapped one
        // for at least one millisecond after the change.
        let sr = 48_000.0;
        let mut smoothed = WdfLadderFilter::new();
        smoothed.set_sample_rate(sr);
        smoothed.set_parameters(20_000.0, 0.0, 0.0);
        // Prime past the first-set snap.
        for _ in 0..512 {
            smoothed.process(0.5);
        }

        let mut snapped = smoothed.clone();
        snapped.set_parameters(200.0, 0.0, 0.0);
        snapped.settle();
        smoothed.set_parameters(200.0, 0.0, 0.0);

        let mut delta = 0.0_f32;
        for i in 0..48 {
            let x = ((i as f32) * 0.3).sin();
            delta += (smoothed.process(x) - snapped.process(x)).abs();
        }
        assert!(
            delta > 0.0,
            "smoothed filter should lag the snapped one immediately after a cutoff change"
        );
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
