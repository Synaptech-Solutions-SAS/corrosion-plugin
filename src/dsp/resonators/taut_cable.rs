//! Taut Cable resonator model using Stiff String with Tension-Modulated Pitch.
//!
//! Frequency formula: f_n = n * f_0 * sqrt(1 + B * n^2), with f_0(t) rising with amplitude

use crate::dsp::resonators::ResonatorAlgorithm;
use crate::dsp::ModalModeSpec;

/// Resonator model for stiff cable-like spectra.
#[derive(Clone, Debug)]
pub struct TautCableResonator {
    cable_tension: f32,
    braid_stiffness: f32,
    tension_drop: f32,
}

impl TautCableResonator {
    pub fn new() -> Self {
        Self {
            cable_tension: 0.5,
            braid_stiffness: 0.3,
            tension_drop: 0.4,
        }
    }
}

impl Default for TautCableResonator {
    fn default() -> Self {
        Self::new()
    }
}

impl ResonatorAlgorithm for TautCableResonator {
    fn generate_modes(
        &self,
        fundamental_hz: f32,
        mode_count: usize,
        size_scale: crate::dsp::SizeScale,
    ) -> Vec<ModalModeSpec> {
        let tension_factor = 0.8 + self.cable_tension * 0.4;
        let f0 = (fundamental_hz * tension_factor) / size_scale.factor();
        let b = self.braid_stiffness * 0.01;

        (1..=mode_count)
            .map(|n| {
                let nf = n as f32;
                let freq = nf * f0 * (1.0 + b * nf * nf).sqrt();
                let decay = 2.0 - 0.5 * (nf / mode_count as f32);
                let gain = 0.025 / (nf * nf * 0.5 + 0.5);

                ModalModeSpec::new(freq, decay, gain)
            })
            .collect()
    }

    fn update_dynamic_frequencies(&mut self, modes: &mut [ModalModeSpec], total_amplitude: f32) {
        let pitch_shift = 1.0 + self.tension_drop * total_amplitude * 0.1;
        for mode in modes.iter_mut() {
            mode.frequency_hz *= pitch_shift;
        }
    }
}
