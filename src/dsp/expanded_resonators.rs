//! Expanded resonator families for non-canonical modal objects.
//!
//! These helpers generate frequency/decay/gain triplets for profile expansion
//! so the voice layer can emulate beams, cables, springs, plates, and cogs
//! without allocating during audio processing.

/// I-Beam - Timoshenko Thick Beam Theory
/// f_n = f_1 * n^2 / sqrt(1 + C * n^2)
#[derive(Clone, Debug)]
/// Resonator model for I-beam-like modal spectra.
pub struct IBeamResonator {
    beam_mass: f32,
    shear_density: f32,
    rigidity_damping: f32,
    base_frequency: f32,
}

impl IBeamResonator {
    /// Create the default I-beam resonator model.
    pub fn new() -> Self {
        Self {
            beam_mass: 1.0,
            shear_density: 0.5,
            rigidity_damping: 0.7,
            base_frequency: 55.0,
        }
    }

    /// Generate modal triplets for an I-beam spectrum.
    pub fn generate_modes(&self, count: usize, size_scale: f32) -> Vec<(f32, f32, f32)> {
        let mut modes = Vec::with_capacity(count);
        let f1 = self.base_frequency / size_scale;
        let c = self.shear_density * 2.0;

        for n in 1..=count {
            let nf = n as f32;
            let freq = f1 * nf * nf / (1.0 + c * nf * nf).sqrt();
            let decay = 1.5 - self.rigidity_damping * 0.8 * (nf / count as f32);
            let gain = 1.0 / nf;
            modes.push((freq, decay, gain * self.beam_mass));
        }
        modes
    }
}

impl Default for IBeamResonator {
    /// Construct the default I-beam resonator.
    fn default() -> Self {
        Self::new()
    }
}

/// Taut Cable - Stiff String with Tension-Modulated Pitch
/// f_n = n * f_0 * sqrt(1 + B * n^2), with f_0(t) rising with amplitude
#[derive(Clone, Debug)]
/// Resonator model for stiff cable-like spectra.
pub struct TautCableResonator {
    cable_tension: f32,
    braid_stiffness: f32,
    tension_drop: f32,
    base_frequency: f32,
}

impl TautCableResonator {
    /// Create the default taut-cable resonator model.
    pub fn new() -> Self {
        Self {
            cable_tension: 0.5,
            braid_stiffness: 0.3,
            tension_drop: 0.4,
            base_frequency: 110.0,
        }
    }

    /// Generate modal triplets for a taut-cable spectrum.
    pub fn generate_modes(&self, count: usize, size_scale: f32) -> Vec<(f32, f32, f32)> {
        let mut modes = Vec::with_capacity(count);
        let tension_factor = 0.8 + self.cable_tension * 0.4;
        let f0 = (self.base_frequency * tension_factor) / size_scale;
        let b = self.braid_stiffness * 0.01;

        for n in 1..=count {
            let nf = n as f32;
            let freq = nf * f0 * (1.0 + b * nf * nf).sqrt();
            let decay = 2.0 - 0.5 * (nf / count as f32);
            let gain = 1.0 / (nf * nf * 0.5 + 0.5);
            modes.push((freq, decay, gain));
        }
        modes
    }

    /// Modulate the fundamental frequency by total amplitude.
    pub fn modulate_frequency(&self, f0: f32, total_amplitude: f32) -> f32 {
        f0 + self.tension_drop * total_amplitude * 50.0
    }
}

impl Default for TautCableResonator {
    /// Construct the default taut-cable resonator.
    fn default() -> Self {
        Self::new()
    }
}

/// Heavy Coil Spring - Highly Dispersive Helical Waveguide
/// f_n = f_1 * (n^alpha + Jitter_n), with comb-filter pickup structure
#[derive(Clone, Debug)]
/// Resonator model for dispersive coil-spring spectra.
pub struct CoilSpringResonator {
    coil_length: f32,
    dispersion_chirp: f32,
    spring_slosh: f32,
    base_frequency: f32,
}

impl CoilSpringResonator {
    /// Create the default coil-spring resonator model.
    pub fn new() -> Self {
        Self {
            coil_length: 0.5,
            dispersion_chirp: 0.5,
            spring_slosh: 0.3,
            base_frequency: 80.0,
        }
    }

    /// Generate modal triplets for a coil-spring spectrum.
    pub fn generate_modes(&self, count: usize, size_scale: f32) -> Vec<(f32, f32, f32)> {
        let mut modes = Vec::with_capacity(count);
        let f1 = self.base_frequency / size_scale;
        let alpha = 0.5 + self.coil_length * 0.5;

        for n in 1..=count {
            let nf = n as f32;
            let jitter = (nf * 1.618).fract() * self.spring_slosh * 0.5;
            let freq = f1 * (nf.powf(alpha) + jitter);
            let decay = 1.0 + 0.5 * (nf / count as f32);
            let dispersion = (nf * self.dispersion_chirp * std::f32::consts::PI).cos();
            let gain = (1.0 / nf) * dispersion.abs();
            modes.push((freq, decay, gain.max(0.01)));
        }
        modes
    }
}

impl Default for CoilSpringResonator {
    /// Construct the default coil-spring resonator.
    fn default() -> Self {
        Self::new()
    }
}

/// Sheet Metal - 2D Plate with Dynamic Buckling
/// ω_n(t) = ω_{n,0} * (1 + β * (sum low-freq displacement)^2)
#[derive(Clone, Debug)]
/// Resonator model for sheet-metal plate spectra.
pub struct SheetMetalResonator {
    sheet_size: f32,
    metal_thinness: f32,
    edge_damping: f32,
}

impl SheetMetalResonator {
    /// Create the default sheet-metal resonator model.
    pub fn new() -> Self {
        Self {
            sheet_size: 0.5,
            metal_thinness: 0.4,
            edge_damping: 0.3,
        }
    }

    /// Generate modal triplets for a sheet-metal spectrum.
    pub fn generate_modes(&self, count: usize, size_scale: f32) -> Vec<(f32, f32, f32)> {
        let mut modes = Vec::with_capacity(count);
        let base = 60.0 / (size_scale * self.sheet_size);

        for n in 1..=count {
            let nf = n as f32;
            let freq = base * (nf * nf);
            let warp_factor = 1.0 + self.metal_thinness * 0.2;
            let decay = (2.0 - self.edge_damping) * (1.0 + nf / count as f32);
            let gain = 1.0 / nf;
            modes.push((freq * warp_factor, decay, gain));
        }
        modes
    }

    /// Warp a base frequency by low-frequency displacement.
    pub fn warp_frequency(&self, f0: f32, lf_displacement: f32) -> f32 {
        let warp = 1.0 + self.metal_thinness * lf_displacement * lf_displacement * 10.0;
        f0 * warp
    }
}

impl Default for SheetMetalResonator {
    /// Construct the default sheet-metal resonator.
    fn default() -> Self {
        Self::new()
    }
}

/// Industrial Cog - Circular Free-Boundary Plate with Mode Splitting
/// f_n split into f_n(1 ± ε) pairs for metallic beating
#[derive(Clone, Debug)]
/// Resonator model for split-pair cog and blade spectra.
pub struct IndustrialCogResonator {
    blade_radius: f32,
    tooth_dissonance: f32,
    blade_thickness: f32,
    base_frequency: f32,
}

impl IndustrialCogResonator {
    const BESSEL_ROOTS: [f32; 6] = [3.83, 7.02, 10.17, 13.32, 16.47, 19.64];

    /// Create the default industrial-cog resonator model.
    pub fn new() -> Self {
        Self {
            blade_radius: 0.5,
            tooth_dissonance: 0.1,
            blade_thickness: 0.5,
            base_frequency: 200.0,
        }
    }

    /// Generate modal triplets for an industrial-cog spectrum.
    pub fn generate_modes(&self, count: usize, size_scale: f32) -> Vec<(f32, f32, f32)> {
        let mut modes = Vec::with_capacity(count * 2);
        let scale = self.base_frequency / (size_scale * (0.5 + self.blade_radius));

        for i in 0..count.min(6) {
            let root = Self::BESSEL_ROOTS[i];
            let base_freq = scale * root * root * 0.01;
            let epsilon = self.tooth_dissonance * 0.03 * ((i + 1) as f32);

            let decay = 0.5 + self.blade_thickness * 1.5;
            let gain = 1.0 / ((i + 1) as f32);

            modes.push((base_freq * (1.0 - epsilon), decay, gain));
            modes.push((base_freq * (1.0 + epsilon), decay * 0.9, gain * 0.8));
        }
        modes
    }
}

impl Default for IndustrialCogResonator {
    /// Construct the default industrial-cog resonator.
    fn default() -> Self {
        Self::new()
    }
}
