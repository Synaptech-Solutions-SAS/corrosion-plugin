//! Real-time post-processing chain.
//!
//! The chain runs mono filtering/drive/body stages, widens to stereo, applies
//! the selected space mode, and finishes with an oversampled clipper.
use super::{
    FactoryEcho, FactoryReverb, FemBodyResonator, HrtfSpread, LorenzDrive, OversampledClipper,
    SpaceMode, SpringReverb, WdfLadderFilter,
};

/// Ordered post-processing pipeline for the plugin output.
pub struct PostProcessingChain {
    filter: WdfLadderFilter,
    drive: LorenzDrive,
    body: FemBodyResonator,
    spread: HrtfSpread,
    factory_reverb: FactoryReverb,
    spring_reverb: SpringReverb,
    factory_echo: FactoryEcho,
    clipper: OversampledClipper,

    space_mode: SpaceMode,
    space_amount: f32,

    sample_rate: f32,
}

impl PostProcessingChain {
    /// Creates the chain with default stage instances and sample rate.
    pub fn new() -> Self {
        Self {
            filter: WdfLadderFilter::new(),
            drive: LorenzDrive::new(),
            body: FemBodyResonator::new(),
            spread: HrtfSpread::new(),
            factory_reverb: FactoryReverb::new(),
            spring_reverb: SpringReverb::new(),
            factory_echo: FactoryEcho::new(),
            clipper: OversampledClipper::new(),
            space_mode: SpaceMode::Off,
            space_amount: 0.0,
            sample_rate: 48000.0,
        }
    }

    /// Updates the sample rate for every stage in the chain.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.filter.set_sample_rate(sample_rate);
        self.drive.set_sample_rate(sample_rate);
        self.body.set_sample_rate(sample_rate);
        self.spread.set_sample_rate(sample_rate);
        self.factory_reverb.set_sample_rate(sample_rate);
        self.spring_reverb.set_sample_rate(sample_rate);
        self.factory_echo.set_sample_rate(sample_rate);
        self.clipper.set_sample_rate(sample_rate);
    }

    /// Sets the WDF filter controls.
    pub fn set_filter_params(&mut self, cutoff: f32, resonance: f32, tolerance: f32) {
        self.filter.set_parameters(cutoff, resonance, tolerance);
    }

    /// Sets the chaotic drive controls.
    pub fn set_drive_params(&mut self, amount: f32, starvation: f32, chaos: f32) {
        self.drive.set_parameters(amount, starvation, chaos);
    }

    /// Sets the body resonance controls.
    pub fn set_body_params(&mut self, material: f32, volume: f32) {
        self.body.set_parameters(material, volume);
    }

    /// Sets stereo spread and listener proximity controls.
    pub fn set_spread_params(&mut self, width: f32, proximity: f32) {
        self.spread.set_parameters(width, proximity);
    }

    /// Chooses which space algorithm is active.
    pub fn set_space_mode(&mut self, mode: SpaceMode) {
        self.space_mode = mode;
    }

    /// Sets the wet mix for the active space algorithm.
    pub fn set_space_amount(&mut self, amount: f32) {
        self.space_amount = amount.clamp(0.0, 1.0);
    }

    /// Sets the factory room parameters.
    pub fn set_factory_params(&mut self, size: f32, clutter: f32, impedance: f32) {
        self.factory_reverb.set_parameters(size, clutter, impedance);
    }

    /// Sets the spring reverb parameters.
    pub fn set_spring_params(&mut self, tension: f32, stiffness: f32, tank_size: f32) {
        self.spring_reverb
            .set_parameters(tension, stiffness, tank_size);
    }

    /// Sets the factory echo parameters.
    pub fn set_echo_params(
        &mut self,
        delay_time: f32,
        machinery_movement: f32,
        high_frequency_damping: f32,
    ) {
        self.factory_echo
            .set_parameters(delay_time, machinery_movement, high_frequency_damping);
    }

    /// Sets the final clipper ceiling and knee softness.
    pub fn set_clipper_params(&mut self, ceiling: f32, softness: f32) {
        self.clipper.set_parameters(ceiling, softness);
    }

    /// Processes one stereo frame through the full stage order.
    pub fn process(&mut self, left: f32, right: f32) -> (f32, f32) {
        let mono = (left + right) * 0.5;

        // 1. WDF Ladder Filter
        let filtered = self.filter.process(mono);

        // 2. Lorenz Chaotic Drive
        let driven = self.drive.process(filtered);

        // 3. FEM Body Resonator
        let bodied = self.body.process(driven);

        // Convert back to stereo for spread
        let bodied_left = left * 0.3 + bodied * 0.7;
        let bodied_right = right * 0.3 + bodied * 0.7;

        // 4. HRTF Stereo Spread
        let (spread_left, spread_right) = self.spread.process(bodied_left, bodied_right);

        // 5. Space (Factory or Spring)
        let (space_left, space_right) = match self.space_mode {
            SpaceMode::Off => (spread_left, spread_right),
            SpaceMode::Factory => {
                let wet = self
                    .factory_reverb
                    .process((spread_left + spread_right) * 0.5);
                (
                    spread_left + wet * self.space_amount,
                    spread_right + wet * self.space_amount,
                )
            }
            SpaceMode::Spring => {
                let wet_l = self.spring_reverb.process(spread_left);
                let wet_r = self.spring_reverb.process(spread_right);
                (
                    spread_left + wet_l * self.space_amount,
                    spread_right + wet_r * self.space_amount,
                )
            }
            SpaceMode::Echo => {
                let (wet_l, wet_r) = self.factory_echo.process_stereo(spread_left, spread_right);
                (
                    spread_left + wet_l * self.space_amount,
                    spread_right + wet_r * self.space_amount,
                )
            }
        };

        // 6. 16x Oversampled Clipper
        self.clipper.process_stereo(space_left, space_right)
    }

    /// Clears internal state for all stage buffers and filters.
    pub fn reset(&mut self) {
        self.filter.reset();
        self.drive.reset();
        self.body.reset();
        self.spread.reset();
        self.factory_reverb.reset();
        self.spring_reverb.reset();
        self.factory_echo.reset();
        self.clipper.reset();
    }
}

impl Default for PostProcessingChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_processes_stereo() {
        let mut chain = PostProcessingChain::new();
        chain.set_sample_rate(48000.0);

        let (left, right) = chain.process(0.5, 0.3);
        assert!(left.is_finite());
        assert!(right.is_finite());
    }

    #[test]
    fn output_bounded() {
        let mut chain = PostProcessingChain::new();
        chain.set_sample_rate(48000.0);
        chain.set_drive_params(5.0, 1.0, 1.0);

        for i in 0..100 {
            let input = (i as f32 * 0.1).sin() * 2.0;
            let (left, right) = chain.process(input, input);
            assert!(left.abs() <= 1.0, "Left should be bounded by clipper");
            assert!(right.abs() <= 1.0, "Right should be bounded by clipper");
        }
    }
}
