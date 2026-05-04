#[derive(Clone, Copy, Debug)]
pub struct ModalModeSpec {
    pub frequency_hz: f32,
    pub decay_seconds: f32,
    pub gain: f32,
}

impl ModalModeSpec {
    pub const fn new(frequency_hz: f32, decay_seconds: f32, gain: f32) -> Self {
        Self {
            frequency_hz,
            decay_seconds,
            gain,
        }
    }

    pub fn scaled_for_size(
        self,
        size_scale: crate::dsp::SizeScale,
        mode_index: usize,
        mode_count: usize,
    ) -> Self {
        let scale = size_scale.factor();
        let low_mode_weight = if mode_count <= 1 {
            1.0
        } else {
            1.0 - (mode_index as f32 / (mode_count - 1) as f32)
        };
        let resonance_tilt = 1.0 + (scale - 1.0) * 0.35 * low_mode_weight;

        Self {
            frequency_hz: self.frequency_hz / scale,
            decay_seconds: self.decay_seconds * scale,
            gain: (self.gain * resonance_tilt).max(f32::EPSILON),
        }
    }

    pub fn corroded(
        self,
        rust_amount: crate::dsp::RustAmount,
        mode_index: usize,
        mode_count: usize,
    ) -> Self {
        let rust = rust_amount.amount();
        let high_mode_weight = if mode_count <= 1 {
            0.0
        } else {
            mode_index as f32 / (mode_count - 1) as f32
        };
        let brightness_loss = 1.0 - rust * (0.20 + 0.65 * high_mode_weight);
        let decay_loss = 1.0 - rust * (0.30 + 0.45 * high_mode_weight);

        Self {
            frequency_hz: self.frequency_hz,
            decay_seconds: (self.decay_seconds * decay_loss).max(f32::EPSILON),
            gain: (self.gain * brightness_loss).max(f32::EPSILON),
        }
    }

    pub fn damaged(
        self,
        damage_amount: crate::dsp::DamageAmount,
        mode_index: usize,
        mode_count: usize,
    ) -> Vec<Self> {
        let damage = damage_amount.amount();
        if damage <= 0.0 {
            return vec![self];
        }

        let mode_position = if mode_count <= 1 {
            0.0
        } else {
            mode_index as f32 / (mode_count - 1) as f32
        };
        let low_mode_weight = 1.0 - mode_position;
        let detune_direction = if mode_index % 2 == 0 { -1.0 } else { 1.0 };
        let primary_detune = damage * (0.004 + 0.018 * mode_position);
        let companion_detune = damage * (0.010 + 0.030 * mode_position);
        let primary_decay_tilt = 1.0 - damage * (0.10 + 0.18 * low_mode_weight);
        let primary_gain_tilt = 1.0 + damage * (0.05 + 0.12 * mode_position);
        let companion_gain = self.gain * damage * (0.16 + 0.20 * mode_position);
        let companion_decay =
            self.decay_seconds * (0.28 + 0.18 * low_mode_weight + 0.14 * (1.0 - damage));

        let primary = Self {
            frequency_hz: (self.frequency_hz * (1.0 + primary_detune * detune_direction))
                .max(f32::EPSILON),
            decay_seconds: (self.decay_seconds * primary_decay_tilt).max(f32::EPSILON),
            gain: (self.gain * primary_gain_tilt).max(f32::EPSILON),
        };
        let companion = Self {
            frequency_hz: (self.frequency_hz * (1.0 - companion_detune * detune_direction))
                .max(f32::EPSILON),
            decay_seconds: companion_decay.max(f32::EPSILON),
            gain: companion_gain.max(f32::EPSILON),
        };

        vec![primary, companion]
    }
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalProfileId {
    Pipe,
    Plate,
    Tank,
    Chain,
}

#[derive(Clone, Copy, Debug)]
pub struct ModalProfile {
    pub id: ModalProfileId,
    pub modes: &'static [ModalModeSpec],
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const PIPE_MODAL_PROFILE_MODES: [ModalModeSpec; 6] = [
    ModalModeSpec::new(220.0, 2.05, 0.0152),
    ModalModeSpec::new(439.5, 1.72, 0.0135),
    ModalModeSpec::new(660.0, 1.36, 0.0112),
    ModalModeSpec::new(881.0, 1.05, 0.0088),
    ModalModeSpec::new(1_103.0, 0.81, 0.0066),
    ModalModeSpec::new(1_327.0, 0.62, 0.0048),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const PLATE_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(286.0, 0.94, 0.0092),
    ModalModeSpec::new(463.0, 0.82, 0.0089),
    ModalModeSpec::new(731.0, 0.72, 0.0084),
    ModalModeSpec::new(1_036.0, 0.61, 0.0076),
    ModalModeSpec::new(1_394.0, 0.52, 0.0068),
    ModalModeSpec::new(1_811.0, 0.44, 0.0059),
    ModalModeSpec::new(2_297.0, 0.37, 0.0050),
    ModalModeSpec::new(2_860.0, 0.31, 0.0042),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const TANK_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(96.0, 2.90, 0.0260),
    ModalModeSpec::new(151.0, 2.55, 0.0218),
    ModalModeSpec::new(226.0, 2.22, 0.0178),
    ModalModeSpec::new(318.0, 1.90, 0.0139),
    ModalModeSpec::new(439.0, 1.56, 0.0104),
    ModalModeSpec::new(588.0, 1.22, 0.0077),
    ModalModeSpec::new(774.0, 0.94, 0.0056),
    ModalModeSpec::new(1_002.0, 0.72, 0.0040),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const CHAIN_MODAL_PROFILE_MODES: [ModalModeSpec; 10] = [
    ModalModeSpec::new(74.0, 1.10, 0.0200),
    ModalModeSpec::new(78.0, 1.02, 0.0192),
    ModalModeSpec::new(91.0, 0.95, 0.0184),
    ModalModeSpec::new(95.0, 0.88, 0.0174),
    ModalModeSpec::new(124.0, 0.80, 0.0160),
    ModalModeSpec::new(130.0, 0.72, 0.0144),
    ModalModeSpec::new(167.0, 0.64, 0.0126),
    ModalModeSpec::new(174.0, 0.55, 0.0106),
    ModalModeSpec::new(271.0, 0.46, 0.0084),
    ModalModeSpec::new(283.0, 0.38, 0.0062),
];

impl ModalProfile {
    pub fn from_id(id: ModalProfileId) -> Self {
        match id {
            ModalProfileId::Pipe => Self::pipe(),
            ModalProfileId::Plate => Self::plate(),
            ModalProfileId::Tank => Self::tank(),
            ModalProfileId::Chain => Self::chain(),
        }
    }

    pub const fn pipe() -> Self {
        Self {
            id: ModalProfileId::Pipe,
            modes: &PIPE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub const fn plate() -> Self {
        Self {
            id: ModalProfileId::Plate,
            modes: &PLATE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub const fn tank() -> Self {
        Self {
            id: ModalProfileId::Tank,
            modes: &TANK_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub const fn chain() -> Self {
        Self {
            id: ModalProfileId::Chain,
            modes: &CHAIN_MODAL_PROFILE_MODES,
        }
    }

    pub fn scaled_mode_specs(
        self,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
    ) -> Vec<ModalModeSpec> {
        self.modes
            .iter()
            .enumerate()
            .flat_map(|(index, mode)| {
                mode.scaled_for_size(size_scale, index, self.modes.len())
                    .corroded(rust_amount, index, self.modes.len())
                    .damaged(damage_amount, index, self.modes.len())
            })
            .collect()
    }
}
