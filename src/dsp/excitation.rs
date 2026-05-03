#[derive(Clone, Debug, PartialEq)]
pub struct ExcitationInput {
    samples: Vec<f32>,
}

impl ExcitationInput {
    pub fn impulse(frame_count: usize, excitation_frame: usize, amplitude: f32) -> Self {
        let mut samples = vec![0.0; frame_count];

        if let Some(sample) = samples.get_mut(excitation_frame) {
            *sample = amplitude;
        }

        Self { samples }
    }

    pub fn frame_count(&self) -> usize {
        self.samples.len()
    }

    pub fn sample(&self, frame: usize) -> f32 {
        self.samples[frame]
    }

    #[cfg(test)]
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}
