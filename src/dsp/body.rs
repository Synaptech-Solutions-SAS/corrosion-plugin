//! Body resonator for low-frequency support coloration.
//!
//! The voice layer can mix this resonator with the main modal object to add a
//! perceptible body response without changing the primary modal profile.

use crate::dsp::ResonatorCoefficients;
use std::f32::consts::PI;

#[derive(Clone, Debug)]
/// Four-mode body resonator driven in the audio-rate voice path.
pub struct BodyResonator {
    modes: [BodyMode; 4],
}

#[derive(Clone, Debug)]
struct BodyMode {
    frequency_hz: f32,
    decay_seconds: f32,
    gain: f32,
    y1: f32,
    y2: f32,
    cached_sample_rate: Option<u32>,
    coefficients: Option<ResonatorCoefficients>,
}

impl BodyMode {
    fn new(frequency_hz: f32, decay_seconds: f32, gain: f32) -> Self {
        Self {
            frequency_hz,
            decay_seconds,
            gain,
            y1: 0.0,
            y2: 0.0,
            cached_sample_rate: None,
            coefficients: None,
        }
    }

    fn process(&mut self, input: f32, sample_rate: u32) -> f32 {
        if self.cached_sample_rate != Some(sample_rate) {
            let safe_sample_rate = sample_rate.max(1) as f32;
            let decay_seconds = self.decay_seconds.max(f32::EPSILON);
            let omega = 2.0 * PI * self.frequency_hz / safe_sample_rate;
            let r = (-1.0 / (decay_seconds * safe_sample_rate)).exp();

            self.coefficients = Some(ResonatorCoefficients {
                b0: self.gain,
                a1: -2.0 * r * omega.cos(),
                a2: r * r,
            });
            self.cached_sample_rate = Some(sample_rate);
        }

        let coeffs = self.coefficients.as_ref().unwrap();
        let sample = coeffs.b0 * input - coeffs.a1 * self.y1 - coeffs.a2 * self.y2;
        self.y2 = self.y1;
        self.y1 = sample;
        sample
    }
}

impl BodyResonator {
    /// Create the default four-mode body resonator.
    pub fn new() -> Self {
        Self {
            modes: [
                BodyMode::new(220.0, 0.8, 0.12),
                BodyMode::new(380.0, 0.6, 0.10),
                BodyMode::new(550.0, 0.5, 0.08),
                BodyMode::new(720.0, 0.4, 0.06),
            ],
        }
    }

    /// Process one sample through the body resonator.
    ///
    /// `amount` scales the wet contribution, and the input is returned dry when
    /// the amount is zero.
    pub fn process_sample(&mut self, input: f32, sample_rate: u32, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let body_sum = self
            .modes
            .iter_mut()
            .map(|mode| mode.process(input, sample_rate))
            .sum::<f32>();

        input + body_sum * amount
    }
}

impl Default for BodyResonator {
    /// Construct the default body resonator.
    fn default() -> Self {
        Self::new()
    }
}
