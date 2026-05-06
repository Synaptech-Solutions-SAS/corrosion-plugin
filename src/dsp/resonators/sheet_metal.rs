//! Sheet Metal resonator model using 2D Plate with Dynamic Buckling.
//!
//! Frequency formula: ω_n(t) = ω_{n,0} * (1 + β * (sum low-freq displacement)^2)

use crate::dsp::resonators::ResonatorAlgorithm;
use crate::dsp::ModalModeSpec;

/// Resonator model for sheet-metal plate spectra.
#[derive(Clone, Debug)]
pub struct SheetMetalResonator {
    sheet_size: f32,
    metal_thinness: f32,
    edge_damping: f32,
}

impl SheetMetalResonator {
    pub fn new() -> Self {
        Self {
            sheet_size: 0.5,
            metal_thinness: 0.4,
            edge_damping: 0.3,
        }
    }
}

impl Default for SheetMetalResonator {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonatorAlgorithm for SheetMetalResonator {
    fn generate_modes(
        &self,
        fundamental_hz: f32,
        mode_count: usize,
        size_scale: crate::dsp::SizeScale,
    ) -> Vec<ModalModeSpec> {
        let base = fundamental_hz / (size_scale.factor() * self.sheet_size);

        (1..=mode_count)
            .map(|n| {
                let nf = n as f32;
                let freq = base * (nf * nf);
                let warp_factor = 1.0 + self.metal_thinness * 0.2;
                let decay = (2.0 - self.edge_damping) * (1.0 + nf / mode_count as f32);
                let gain = 0.03 / nf;

                ModalModeSpec::new(freq * warp_factor, decay, gain)
            })
            .collect()
    }

    /// Dynamic buckling: frequencies wobble based on LF displacement
    /// Warp_factor(t) = 1 + β * (sum LF displacement)^2
    fn apply_warping(&mut self, modes: &mut [ModalModeSpec], lf_displacement: f32) {
        let warp = 1.0 + self.metal_thinness * lf_displacement * lf_displacement * 10.0;
        for mode in modes.iter_mut() {
            mode.frequency_hz *= warp;
        }
    }
}
