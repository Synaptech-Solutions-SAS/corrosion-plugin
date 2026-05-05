//! Wire brush exciter.
//!
//! Generates a clustered impulse cloud using a bounded Poisson-like timing
//! process, then shapes the result with a high-pass response for bristly attack.
//! It is the `dsp/exciters` stochastic impulse family member rather than a
//! mass/spring contact model.
//! Use it for brushed metal, bristle sweeps, and dense granular swishes.

/// Stochastic impulse-cluster exciter for wire-brush textures.
#[derive(Clone, Debug)]
pub struct WireBrush {
    wire_density: u32,
    spread_duration_ms: f32,
    wire_stiffness: f32, // Normalized 0-1, maps to filter cutoff
    amplitude_randomization: f32,

    // State
    impulses: Vec<Impulse>,
    active: bool,
    time_ms: f32,
    sample_rate: f32,
    total_impulses: u32,
    generated_count: u32,
    rng_phase: f32,
    filter_input: f32,
    filter_output: f32,
}

#[derive(Clone, Debug)]
struct Impulse {
    time_ms: f32,
    amplitude: f32,
    triggered: bool,
}

impl WireBrush {
    /// Creates a default wire-brush exciter.
    pub fn new() -> Self {
        Self {
            wire_density: 50,
            spread_duration_ms: 100.0,
            wire_stiffness: 0.5,
            amplitude_randomization: 0.3,
            impulses: Vec::with_capacity(100),
            active: false,
            time_ms: 0.0,
            sample_rate: 48000.0,
            total_impulses: 50,
            generated_count: 0,
            rng_phase: 0.0,
            filter_input: 0.0,
            filter_output: 0.0,
        }
    }

    /// Sets impulse density, spread, stiffness, and amplitude randomization.
    pub fn set_parameters(
        &mut self,
        density: u32,
        spread_ms: f32,
        stiffness: f32,
        randomization: f32,
    ) {
        self.wire_density = density.clamp(1, 5000);
        self.spread_duration_ms = spread_ms.clamp(1.0, 10000.0);
        self.wire_stiffness = stiffness.clamp(0.0, 5.0);
        self.amplitude_randomization = randomization.clamp(0.0, 5.0);
        self.total_impulses = self.wire_density;
    }

    /// Updates the sample rate used for timing and filtering.
    pub fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }

    /// Starts the impulse cluster and seeds the timing RNG.
    pub fn trigger(&mut self, velocity: f32) {
        self.active = true;
        self.time_ms = 0.0;
        self.generated_count = 0;
        self.impulses.clear();
        self.rng_phase = velocity * 100.0;

        // Pre-generate Poisson-distributed impulses
        let base_amplitude = 0.5 + velocity * 0.5;
        let mut current_time = 0.0;

        // Use simple pseudo-random for deterministic behavior
        for i in 0..self.total_impulses {
            // Poisson process: inter-arrival times are exponential
            let lambda = self.total_impulses as f32 / self.spread_duration_ms;
            let u = self.pseudo_random(i);
            let interval = -u.ln() / lambda;

            current_time += interval;
            if current_time > self.spread_duration_ms {
                break;
            }

            // Random amplitude with controlled variance
            let amp_variation =
                1.0 + (self.pseudo_random(i + 1000) - 0.5) * 2.0 * self.amplitude_randomization;
            let amplitude = base_amplitude * amp_variation.max(0.1);

            self.impulses.push(Impulse {
                time_ms: current_time,
                amplitude,
                triggered: false,
            });
        }
    }

    /// Processes one sample of impulse generation and high-pass shaping.
    pub fn process_sample(
        &mut self,
        _resonator_displacement: f32,
        _resonator_velocity: f32,
    ) -> f32 {
        if !self.active {
            return 0.0;
        }

        // Advance time
        let dt_ms = 1000.0 / self.sample_rate;
        self.time_ms += dt_ms;

        // Find impulses that should trigger this sample
        let mut output = 0.0;
        for impulse in &mut self.impulses {
            if !impulse.triggered && impulse.time_ms <= self.time_ms {
                impulse.triggered = true;
                output += impulse.amplitude;
                self.generated_count += 1;
            }
        }

        // Apply high-pass filter based on wire_stiffness
        // Higher stiffness = higher cutoff = brighter, sharper sound
        let cutoff = 100.0 + self.wire_stiffness * 4000.0;
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let alpha = dt_ms / 1000.0 / (rc + dt_ms / 1000.0);

        // Simple high-pass approximation using struct state
        let filtered = alpha * (self.filter_output + output - self.filter_input);
        self.filter_input = output;
        self.filter_output = filtered;
        output = filtered.max(0.0);

        // Deactivate when all impulses triggered and duration elapsed
        if self.time_ms > self.spread_duration_ms
            && self.generated_count >= self.impulses.len() as u32
        {
            self.active = false;
        }

        output
    }

    /// Returns whether the brush is still emitting impulses.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Simple deterministic pseudo-random for reproducibility
    fn pseudo_random(&mut self, seed: u32) -> f32 {
        self.rng_phase += 1.61803398875 + seed as f32 * 0.1;
        let hash = (self.rng_phase.sin() * 43758.5453).fract();
        hash.abs()
    }
}

impl Default for WireBrush {
    fn default() -> Self {
        Self::new()
    }
}
