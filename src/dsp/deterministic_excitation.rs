//! Deterministic excitation buffers for tests and offline rendering.
//!
//! The voice module can use these fixed buffers to replay exact excitation
//! shapes without adding randomness or host-dependent state.

#[derive(Clone, Debug, PartialEq)]
/// Sample buffer containing a deterministic excitation pattern.
pub struct ExcitationInput {
    samples: Vec<f32>,
}

impl ExcitationInput {
    /// Create an impulse buffer of the requested length.
    pub fn impulse(frame_count: usize, excitation_frame: usize, amplitude: f32) -> Self {
        let mut samples = vec![0.0; frame_count];

        if let Some(sample) = samples.get_mut(excitation_frame) {
            *sample = amplitude;
        }

        Self { samples }
    }

    /// Return the number of frames in the buffer.
    pub fn frame_count(&self) -> usize {
        self.samples.len()
    }

    /// Return the excitation sample at `frame`.
    pub fn sample(&self, frame: usize) -> f32 {
        self.samples[frame]
    }

    #[cfg(test)]
    /// Expose the raw buffer for tests.
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}
