use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct ResonatorCoefficients {
    pub b0: f32,
    pub a1: f32,
    pub a2: f32,
}

impl ResonatorCoefficients {
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
pub struct SecondOrderMode {
    pub(crate) spec: crate::dsp::ModalModeSpec,
    coefficients: ResonatorCoefficients,
    cached_sample_rate: Option<u32>,
    y1: f32,
    y2: f32,
}

impl SecondOrderMode {
    pub fn new(spec: crate::dsp::ModalModeSpec) -> Self {
        Self {
            spec,
            coefficients: ResonatorCoefficients::for_mode(spec, 48_000),
            cached_sample_rate: None,
            y1: 0.0,
            y2: 0.0,
        }
    }

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
pub struct ModalResonator {
    pub(crate) profile: crate::dsp::ModalProfileId,
    #[allow(dead_code)]
    pub(crate) size_scale: crate::dsp::SizeScale,
    #[allow(dead_code)]
    pub(crate) rust_amount: crate::dsp::RustAmount,
    #[allow(dead_code)]
    pub(crate) damage_amount: crate::dsp::DamageAmount,
    pub(crate) modes: Vec<SecondOrderMode>,
}

impl ModalResonator {
    fn from_profile(
        profile: crate::dsp::ModalProfile,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
    ) -> Self {
        Self {
            profile: profile.id,
            size_scale,
            rust_amount,
            damage_amount,
            modes: profile
                .scaled_mode_specs(size_scale, rust_amount, damage_amount)
                .iter()
                .copied()
                .map(SecondOrderMode::new)
                .collect(),
        }
    }

    pub fn with_profile(profile_id: crate::dsp::ModalProfileId) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            crate::dsp::SizeScale::default(),
            crate::dsp::RustAmount::default(),
            crate::dsp::DamageAmount::default(),
        )
    }

    pub fn with_profile_and_size(
        profile_id: crate::dsp::ModalProfileId,
        size_scale: crate::dsp::SizeScale,
    ) -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::from_id(profile_id),
            size_scale,
            crate::dsp::RustAmount::default(),
            crate::dsp::DamageAmount::default(),
        )
    }

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
        )
    }

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
        )
    }
}

impl Default for ModalResonator {
    fn default() -> Self {
        Self::from_profile(
            crate::dsp::ModalProfile::pipe(),
            crate::dsp::SizeScale::default(),
            crate::dsp::RustAmount::default(),
            crate::dsp::DamageAmount::default(),
        )
    }
}

pub trait ResonatorCore {
    fn process_sample(&mut self, excitation: f32, sample_rate: u32) -> f32;
    fn process_sample_stereo(&mut self, excitation: f32, sample_rate: u32, width: f32) -> (f32, f32);
}

impl ResonatorCore for ModalResonator {
    fn process_sample(&mut self, excitation: f32, sample_rate: u32) -> f32 {
        let mode_sum = self
            .modes
            .iter_mut()
            .map(|mode| mode.process(excitation, sample_rate))
            .sum::<f32>();

        match self.profile {
            crate::dsp::ModalProfileId::Pipe => mode_sum,
            crate::dsp::ModalProfileId::Plate => mode_sum,
            crate::dsp::ModalProfileId::Tank => mode_sum,
            crate::dsp::ModalProfileId::Chain => mode_sum,
        }
    }

    fn process_sample_stereo(&mut self, excitation: f32, sample_rate: u32, width: f32) -> (f32, f32) {
        let mode_count = self.modes.len();
        if mode_count == 0 {
            return (0.0, 0.0);
        }

        let mut left_sum = 0.0f32;
        let mut right_sum = 0.0f32;

        for (index, mode) in self.modes.iter_mut().enumerate() {
            let sample = mode.process(excitation, sample_rate);
            let mode_position = index as f32 / mode_count.max(1) as f32;
            let pan_spread = width * mode_position;
            let pan_direction = if index % 2 == 0 { 1.0 } else { -1.0 };
            let pan = pan_spread * pan_direction;
            let left_gain = (1.0 - pan).max(0.0).min(1.0);
            let right_gain = (1.0 + pan).max(0.0).min(1.0);
            
            left_sum += sample * left_gain;
            right_sum += sample * right_gain;
        }

        (left_sum, right_sum)
    }
}
