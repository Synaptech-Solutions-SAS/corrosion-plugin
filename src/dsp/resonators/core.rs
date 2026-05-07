//! Second-order modal resonator core used by the voice layer.
//!
//! The voice module feeds excitation into these modes per sample, while the
//! interaction bus updates coupling and strike-position coefficients without
//! allocating on the audio thread.

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
/// Coefficients for a damped second-order modal resonator.
pub struct ResonatorCoefficients {
    /// Input gain.
    pub b0: f32,
    /// Feedback coefficient for the first delay.
    pub a1: f32,
    /// Feedback coefficient for the second delay.
    pub a2: f32,
}

impl ResonatorCoefficients {
    /// Compute resonator coefficients for a modal specification.
    pub fn for_mode(mode: crate::dsp::ModalModeSpec, sample_rate: u32) -> Self {
        let safe_sample_rate = sample_rate.max(1) as f32;
        let decay_seconds = mode.decay_seconds.max(f32::EPSILON);
        let omega = 2.0 * PI * mode.frequency_hz / safe_sample_rate;
        let r = (-1.0 / (decay_seconds * safe_sample_rate)).exp();

        Self {
            b0: mode.gain,
            a1: -2.0 * r * omega.cos(),
            a2: r * r,
        }
    }
}

#[derive(Clone, Debug)]
/// Cached second-order state for one modal peak.
pub struct SecondOrderMode {
    pub(crate) spec: crate::dsp::ModalModeSpec,
    coefficients: ResonatorCoefficients,
    cached_sample_rate: Option<u32>,
    y1: f32,
    y2: f32,
}

impl SecondOrderMode {
    /// Create a modal state from a modal specification.
    pub fn new(spec: crate::dsp::ModalModeSpec) -> Self {
        Self {
            spec,
            coefficients: ResonatorCoefficients::for_mode(spec, 48_000),
            cached_sample_rate: None,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Process one excitation sample through the resonator.
    pub fn process(&mut self, excitation: f32, sample_rate: u32) -> f32 {
        if self.cached_sample_rate != Some(sample_rate) {
            self.coefficients = ResonatorCoefficients::for_mode(self.spec, sample_rate);
            self.cached_sample_rate = Some(sample_rate);
        }

        let sample = self.coefficients.b0 * excitation
            - self.coefficients.a1 * self.y1
            - self.coefficients.a2 * self.y2;

        self.y2 = self.y1;
        self.y1 = sample;
        sample
    }
}

#[derive(Clone, Debug)]
/// Modal resonator assembled from a profile and control transforms.
pub struct ModalResonator {
    pub(crate) profile: crate::dsp::ModalProfileId,
    #[allow(dead_code)]
    pub(crate) size_scale: crate::dsp::SizeScale,
    #[allow(dead_code)]
    pub(crate) rust_amount: crate::dsp::RustAmount,
    #[allow(dead_code)]
    pub(crate) damage_amount: crate::dsp::DamageAmount,
    #[allow(dead_code)]
    pub(crate) thickness_amount: crate::dsp::ThicknessAmount,
    #[allow(dead_code)]
    pub(crate) heat_amount: crate::dsp::HeatAmount,
    #[allow(dead_code)]
    pub(crate) sludge_amount: crate::dsp::SludgeAmount,
    pub(crate) modes: Vec<SecondOrderMode>,
    interaction_bus: crate::dsp::BidirectionalInteractionBus,
    last_output: f32,
    current_output: f32,
}

impl ModalResonator {
    const REFERENCE_PITCH_HZ: f32 = 220.0;

    // Internal constructor intentionally mirrors all modal control transforms.
    #[allow(clippy::too_many_arguments)]
    fn from_profile(
        profile: crate::dsp::ModalProfile,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
        thickness_amount: crate::dsp::ThicknessAmount,
        heat_amount: crate::dsp::HeatAmount,
        sludge_amount: crate::dsp::SludgeAmount,
        target_frequency_hz: Option<f32>,
        damping: f32,
        brightness: f32,
    ) -> Self {
        let pitch_scale = target_frequency_hz
            .map(|target| (target / Self::REFERENCE_PITCH_HZ.max(f32::EPSILON)).max(f32::EPSILON));
        let mode_count = profile.mode_count().max(1);
        let modes: Vec<SecondOrderMode> = profile
            .scaled_mode_specs_extended(
                size_scale,
                rust_amount,
                damage_amount,
                thickness_amount,
                heat_amount,
                sludge_amount,
            )
            .into_iter()
            .enumerate()
            .map(|(index, mode)| {
                let mode_weight = if mode_count <= 1 {
                    0.0
                } else {
                    index as f32 / (mode_count - 1) as f32
                };
                let mode = if let Some(scale) = pitch_scale {
                    mode.retuned(scale)
                } else {
                    mode
                };
                mode.adjusted_for_controls(damping, brightness, mode_weight)
            })
            .map(SecondOrderMode::new)
            .collect();

        let mut interaction_bus = crate::dsp::BidirectionalInteractionBus::new();
        interaction_bus.initialize(modes.len());

        Self {
            profile: profile.id,
            size_scale,
            rust_amount,
            damage_amount,
            thickness_amount,
            heat_amount,
            sludge_amount,
            modes,
            interaction_bus,
            last_output: 0.0,
            current_output: 0.0,
        }
    }

    /// Create a resonator from the default profile.
    pub fn with_profile(profile_id: crate::dsp::ModalProfileId) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            crate::dsp::SizeScale::default(),
            crate::dsp::RustAmount::default(),
            crate::dsp::DamageAmount::default(),
            crate::dsp::ThicknessAmount::default(),
            crate::dsp::HeatAmount::default(),
            crate::dsp::SludgeAmount::default(),
            None,
            0.5,
            0.5,
        )
    }

    /// Create a resonator with a profile and size control.
    pub fn with_profile_and_size(
        profile_id: crate::dsp::ModalProfileId,
        size_scale: crate::dsp::SizeScale,
    ) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            size_scale,
            crate::dsp::RustAmount::default(),
            crate::dsp::DamageAmount::default(),
            crate::dsp::ThicknessAmount::default(),
            crate::dsp::HeatAmount::default(),
            crate::dsp::SludgeAmount::default(),
            None,
            0.5,
            0.5,
        )
    }

    /// Create a resonator with profile, size, and rust controls.
    pub fn with_profile_size_and_rust(
        profile_id: crate::dsp::ModalProfileId,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
    ) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            size_scale,
            rust_amount,
            crate::dsp::DamageAmount::default(),
            crate::dsp::ThicknessAmount::default(),
            crate::dsp::HeatAmount::default(),
            crate::dsp::SludgeAmount::default(),
            None,
            0.5,
            0.5,
        )
    }

    /// Create a resonator with size, rust, and damage controls.
    pub fn with_profile_size_rust_and_damage(
        profile_id: crate::dsp::ModalProfileId,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
    ) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            size_scale,
            rust_amount,
            damage_amount,
            crate::dsp::ThicknessAmount::default(),
            crate::dsp::HeatAmount::default(),
            crate::dsp::SludgeAmount::default(),
            None,
            0.5,
            0.5,
        )
    }

    /// Create a resonator with the full control surface and pitch target.
    // Public full-control constructor preserves the existing voice call shape.
    #[allow(clippy::too_many_arguments)]
    pub fn with_profile_controls_and_note(
        profile_id: crate::dsp::ModalProfileId,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
        thickness_amount: crate::dsp::ThicknessAmount,
        heat_amount: crate::dsp::HeatAmount,
        sludge_amount: crate::dsp::SludgeAmount,
        target_frequency_hz: f32,
        damping: f32,
        brightness: f32,
    ) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            size_scale,
            rust_amount,
            damage_amount,
            thickness_amount,
            heat_amount,
            sludge_amount,
            Some(target_frequency_hz),
            damping,
            brightness,
        )
    }
}

impl Default for ModalResonator {
    /// Construct the default pipe resonator.
    fn default() -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::pipe(),
            crate::dsp::SizeScale::default(),
            crate::dsp::RustAmount::default(),
            crate::dsp::DamageAmount::default(),
            crate::dsp::ThicknessAmount::default(),
            crate::dsp::HeatAmount::default(),
            crate::dsp::SludgeAmount::default(),
            None,
            0.5,
            0.5,
        )
    }
}

/// Audio-rate resonator API used by the voice module.
pub trait ResonatorCore {
    /// Process one mono excitation sample.
    fn process_sample(&mut self, excitation: f32, sample_rate: u32) -> f32;
    /// Process one sample and return a stereo pair.
    fn process_sample_stereo(
        &mut self,
        excitation: f32,
        sample_rate: u32,
        width: f32,
    ) -> (f32, f32);
}

impl ModalResonator {
    /// Update interaction parameters from the voice layer.
    pub fn set_interaction_params(
        &mut self,
        strike_position: f32,
        coupling_stiffness: f32,
        position_wander: f32,
        position_envelope: f32,
        fundamental_anchor: f32,
        sample_rate: f32,
    ) {
        self.interaction_bus
            .state
            .set_strike_position(strike_position);
        self.interaction_bus.state.coupling_stiffness = coupling_stiffness.clamp(0.0, 1.0);
        self.interaction_bus.state.fundamental_lock = fundamental_anchor > 0.0;
        self.interaction_bus.state.fundamental_minimum = fundamental_anchor.clamp(0.0, 1.0);
        let wander_rate = 0.1 + position_envelope.clamp(0.0, 1.0) * 4.9;
        self.interaction_bus.state.set_position_wander(
            position_wander,
            wander_rate,
            sample_rate.max(1.0),
        );
        self.interaction_bus.update();
    }

    /// Return the current summed displacement.
    pub fn get_displacement(&self) -> f32 {
        self.interaction_bus.state.resonator_displacement
    }

    /// Return the current summed velocity estimate.
    pub fn get_velocity(&self) -> f32 {
        self.interaction_bus.state.resonator_velocity
    }
}

impl ResonatorCore for ModalResonator {
    fn process_sample(&mut self, excitation: f32, sample_rate: u32) -> f32 {
        self.last_output = self.current_output;

        self.interaction_bus.update();
        let coupling = self
            .interaction_bus
            .state
            .coupling_stiffness
            .clamp(0.0, 1.0);
        self.interaction_bus.state.set_exciter_force(excitation);

        let mut mode_sum = 0.0f32;
        let mut interaction_displacement = 0.0f32;

        for (index, mode) in self.modes.iter_mut().enumerate() {
            let coeff = self
                .interaction_bus
                .mode_coefficients
                .get(index)
                .copied()
                .unwrap_or(1.0);
            let weighted_force = excitation * coeff;
            let blended_force = excitation * (1.0 - coupling) + weighted_force * coupling;
            let mode_output = mode.process(blended_force, sample_rate);
            mode_sum += mode_output;
            interaction_displacement += mode_output * coeff;
        }

        self.current_output = mode_sum;
        self.interaction_bus.state.resonator_velocity =
            interaction_displacement - self.interaction_bus.state.resonator_displacement;
        self.interaction_bus.state.resonator_displacement = interaction_displacement;

        match self.profile {
            crate::dsp::ModalProfileId::Pipe => mode_sum,
            crate::dsp::ModalProfileId::Plate => mode_sum,
            crate::dsp::ModalProfileId::Tank => mode_sum,
            crate::dsp::ModalProfileId::Chain => mode_sum,
            crate::dsp::ModalProfileId::IBeam => mode_sum,
            crate::dsp::ModalProfileId::TautCable => mode_sum,
            crate::dsp::ModalProfileId::CoilSpring => mode_sum,
            crate::dsp::ModalProfileId::SheetMetal => mode_sum,
            crate::dsp::ModalProfileId::IndustrialCog => mode_sum,
        }
    }

    fn process_sample_stereo(
        &mut self,
        excitation: f32,
        sample_rate: u32,
        width: f32,
    ) -> (f32, f32) {
        let mode_count = self.modes.len();
        if mode_count == 0 {
            return (0.0, 0.0);
        }

        self.last_output = self.current_output;
        self.interaction_bus.update();
        let coupling = self
            .interaction_bus
            .state
            .coupling_stiffness
            .clamp(0.0, 1.0);
        self.interaction_bus.state.set_exciter_force(excitation);

        let mut left_sum = 0.0f32;
        let mut right_sum = 0.0f32;
        let mut interaction_displacement = 0.0f32;

        for (index, mode) in self.modes.iter_mut().enumerate() {
            let coeff = self
                .interaction_bus
                .mode_coefficients
                .get(index)
                .copied()
                .unwrap_or(1.0);
            let weighted_force = excitation * coeff;
            let blended_force = excitation * (1.0 - coupling) + weighted_force * coupling;
            let sample = mode.process(blended_force, sample_rate);
            let mode_position = index as f32 / mode_count.max(1) as f32;
            let pan_spread = width * mode_position;
            let pan_direction = if index.is_multiple_of(2) { 1.0 } else { -1.0 };
            let pan = (pan_spread * pan_direction).clamp(-1.0, 1.0);
            let left_gain = 0.5 * (1.0 - pan);
            let right_gain = 0.5 * (1.0 + pan);

            left_sum += sample * left_gain;
            right_sum += sample * right_gain;
            interaction_displacement += sample * coeff;
        }

        self.current_output = (left_sum + right_sum) * 0.5;
        self.interaction_bus.state.resonator_velocity =
            interaction_displacement - self.interaction_bus.state.resonator_displacement;
        self.interaction_bus.state.resonator_displacement = interaction_displacement;
        (left_sum, right_sum)
    }
}
