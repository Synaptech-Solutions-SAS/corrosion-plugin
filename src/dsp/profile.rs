//! Canonical modal profiles and control-driven profile expansion.
//!
//! The voice layer selects a modal profile per note and then expands it using
//! size, rust, damage, thickness, heat, and sludge controls before the
//! resonator turns the mode specs into per-sample IIR modes.

#[derive(Clone, Copy, Debug)]
/// Frequency, decay, and gain for a single modal peak.
pub struct ModalModeSpec {
    /// Peak frequency in hertz.
    pub frequency_hz: f32,
    /// Energy decay time in seconds.
    pub decay_seconds: f32,
    /// Relative amplitude for this mode.
    pub gain: f32,
}

impl ModalModeSpec {
    /// Create a modal mode specification.
    pub const fn new(frequency_hz: f32, decay_seconds: f32, gain: f32) -> Self {
        Self {
            frequency_hz,
            decay_seconds,
            gain,
        }
    }

    /// Scale the mode for object size.
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

    /// Apply corrosion loss to frequency-adjacent brightness and decay.
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

    /// Apply thickness-driven stiffness changes.
    pub fn thickened(
        self,
        thickness_amount: crate::dsp::ThicknessAmount,
        mode_index: usize,
    ) -> Self {
        let thickness = thickness_amount.amount();
        let n = (mode_index + 1) as f32;
        let stiffness = 1.0 + (thickness - 0.5) * 0.15 * n * n;

        Self {
            frequency_hz: (self.frequency_hz * stiffness.max(0.25).sqrt()).max(f32::EPSILON),
            decay_seconds: self.decay_seconds,
            gain: self.gain,
        }
    }

    /// Apply heat-driven softening and brightness loss.
    pub fn heated(self, heat_amount: crate::dsp::HeatAmount) -> Self {
        let heat = heat_amount.amount();
        let pitch_drop = 1.0 - heat * 0.05;
        let brightness_loss = 1.0 - heat * 0.2;

        Self {
            frequency_hz: (self.frequency_hz * pitch_drop.max(0.5)).max(f32::EPSILON),
            decay_seconds: self.decay_seconds,
            gain: (self.gain * brightness_loss.max(0.1)).max(f32::EPSILON),
        }
    }

    /// Apply sludge-driven mass and damping changes.
    pub fn sludge_loaded(self, sludge_amount: crate::dsp::SludgeAmount) -> Self {
        let sludge = sludge_amount.amount();
        let mass_ratio = (1.0 / (1.0 + sludge)).sqrt();
        let decay_loss = 1.0 / (1.0 + sludge * 1.5);
        let gain_loss = 1.0 - sludge * 0.35;

        Self {
            frequency_hz: (self.frequency_hz * mass_ratio).max(f32::EPSILON),
            decay_seconds: (self.decay_seconds * decay_loss.max(0.1)).max(f32::EPSILON),
            gain: (self.gain * gain_loss.max(0.1)).max(f32::EPSILON),
        }
    }

    /// Split a damaged mode into a primary and companion component.
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

    /// Retune the mode by a pitch scale factor.
    pub fn retuned(self, pitch_scale: f32) -> Self {
        Self {
            frequency_hz: (self.frequency_hz * pitch_scale.max(f32::EPSILON)).max(f32::EPSILON),
            ..self
        }
    }

    /// Apply the top-level damping and brightness controls.
    pub fn adjusted_for_controls(self, damping: f32, brightness: f32, mode_weight: f32) -> Self {
        let damping_scale = (1.0 + (0.5 - damping.clamp(0.0, 1.0)) * 1.2).max(0.1);
        let brightness_tilt = 1.0 + (brightness.clamp(0.0, 1.0) - 0.5) * (0.5 + 1.0 * mode_weight);

        Self {
            frequency_hz: self.frequency_hz,
            decay_seconds: (self.decay_seconds * damping_scale).max(f32::EPSILON),
            gain: (self.gain * brightness_tilt.max(0.1)).max(f32::EPSILON),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Identifiers for the canonical modal object families.
pub enum ModalProfileId {
    Pipe,
    Plate,
    Tank,
    Chain,
    IBeam,
    TautCable,
    CoilSpring,
    SheetMetal,
    IndustrialCog,
}

#[derive(Clone, Copy, Debug)]
/// A named canonical modal profile.
pub struct ModalProfile {
    /// Stable profile identifier.
    pub id: ModalProfileId,
    /// Canonical mode list.
    pub modes: &'static [ModalModeSpec],
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const PIPE_MODAL_PROFILE_MODES: [ModalModeSpec; 6] = [
    ModalModeSpec::new(220.0, 2.0, 0.0152),
    ModalModeSpec::new(439.5, 1.60, 0.0135),
    ModalModeSpec::new(660.0, 1.25, 0.0112),
    ModalModeSpec::new(881.0, 0.95, 0.0088),
    ModalModeSpec::new(1_103.0, 0.72, 0.0066),
    ModalModeSpec::new(1_327.0, 0.52, 0.0048),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const PLATE_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(286.0, 0.47, 0.0092),
    ModalModeSpec::new(463.0, 0.41, 0.0089),
    ModalModeSpec::new(731.0, 0.36, 0.0084),
    ModalModeSpec::new(1_036.0, 0.30, 0.0076),
    ModalModeSpec::new(1_394.0, 0.26, 0.0068),
    ModalModeSpec::new(1_811.0, 0.22, 0.0059),
    ModalModeSpec::new(2_297.0, 0.18, 0.0050),
    ModalModeSpec::new(2_860.0, 0.15, 0.0042),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const TANK_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(96.0, 2.5, 0.0260),
    ModalModeSpec::new(151.0, 2.1, 0.0218),
    ModalModeSpec::new(226.0, 1.75, 0.0178),
    ModalModeSpec::new(318.0, 1.40, 0.0139),
    ModalModeSpec::new(439.0, 1.05, 0.0104),
    ModalModeSpec::new(588.0, 0.80, 0.0077),
    ModalModeSpec::new(774.0, 0.58, 0.0056),
    ModalModeSpec::new(1_002.0, 0.45, 0.0040),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const CHAIN_MODAL_PROFILE_MODES: [ModalModeSpec; 10] = [
    ModalModeSpec::new(74.0, 0.55, 0.0200),
    ModalModeSpec::new(78.0, 0.51, 0.0192),
    ModalModeSpec::new(91.0, 0.47, 0.0184),
    ModalModeSpec::new(95.0, 0.44, 0.0174),
    ModalModeSpec::new(124.0, 0.40, 0.0160),
    ModalModeSpec::new(130.0, 0.36, 0.0144),
    ModalModeSpec::new(167.0, 0.32, 0.0126),
    ModalModeSpec::new(174.0, 0.27, 0.0106),
    ModalModeSpec::new(271.0, 0.23, 0.0084),
    ModalModeSpec::new(283.0, 0.19, 0.0062),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const IBEAM_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(55.0, 0.90, 0.0200),
    ModalModeSpec::new(110.0, 0.75, 0.0180),
    ModalModeSpec::new(165.0, 0.60, 0.0150),
    ModalModeSpec::new(220.0, 0.45, 0.0120),
    ModalModeSpec::new(275.0, 0.35, 0.0090),
    ModalModeSpec::new(330.0, 0.25, 0.0060),
    ModalModeSpec::new(385.0, 0.20, 0.0040),
    ModalModeSpec::new(440.0, 0.15, 0.0030),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const TAUTCABLE_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(110.0, 1.25, 0.0250),
    ModalModeSpec::new(220.0, 1.00, 0.0150),
    ModalModeSpec::new(330.0, 0.80, 0.0100),
    ModalModeSpec::new(440.0, 0.60, 0.0070),
    ModalModeSpec::new(550.0, 0.45, 0.0050),
    ModalModeSpec::new(660.0, 0.35, 0.0040),
    ModalModeSpec::new(770.0, 0.25, 0.0030),
    ModalModeSpec::new(880.0, 0.20, 0.0025),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const COILSPRING_MODAL_PROFILE_MODES: [ModalModeSpec; 10] = [
    ModalModeSpec::new(80.0, 0.75, 0.0200),
    ModalModeSpec::new(85.0, 0.70, 0.0180),
    ModalModeSpec::new(95.0, 0.65, 0.0160),
    ModalModeSpec::new(110.0, 0.60, 0.0140),
    ModalModeSpec::new(130.0, 0.55, 0.0120),
    ModalModeSpec::new(160.0, 0.50, 0.0100),
    ModalModeSpec::new(200.0, 0.45, 0.0080),
    ModalModeSpec::new(260.0, 0.40, 0.0060),
    ModalModeSpec::new(350.0, 0.35, 0.0045),
    ModalModeSpec::new(480.0, 0.30, 0.0035),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const SHEETMETAL_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(60.0, 1.50, 0.0300),
    ModalModeSpec::new(120.0, 1.25, 0.0200),
    ModalModeSpec::new(180.0, 1.00, 0.0150),
    ModalModeSpec::new(240.0, 0.80, 0.0110),
    ModalModeSpec::new(300.0, 0.65, 0.0080),
    ModalModeSpec::new(360.0, 0.50, 0.0060),
    ModalModeSpec::new(420.0, 0.40, 0.0045),
    ModalModeSpec::new(480.0, 0.30, 0.0035),
];

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const COG_MODAL_PROFILE_MODES: [ModalModeSpec; 12] = [
    ModalModeSpec::new(200.0, 0.60, 0.0150),
    ModalModeSpec::new(204.0, 0.57, 0.0120),
    ModalModeSpec::new(350.0, 0.45, 0.0100),
    ModalModeSpec::new(357.0, 0.42, 0.0080),
    ModalModeSpec::new(580.0, 0.35, 0.0070),
    ModalModeSpec::new(591.0, 0.32, 0.0060),
    ModalModeSpec::new(850.0, 0.25, 0.0050),
    ModalModeSpec::new(867.0, 0.24, 0.0040),
    ModalModeSpec::new(1150.0, 0.20, 0.0035),
    ModalModeSpec::new(1173.0, 0.19, 0.0030),
    ModalModeSpec::new(1500.0, 0.15, 0.0025),
    ModalModeSpec::new(1530.0, 0.14, 0.0020),
];

impl ModalProfile {
    /// Resolve a profile from its identifier.
    pub fn from_id(id: ModalProfileId) -> Self {
        match id {
            ModalProfileId::Pipe => Self::pipe(),
            ModalProfileId::Plate => Self::plate(),
            ModalProfileId::Tank => Self::tank(),
            ModalProfileId::Chain => Self::chain(),
            ModalProfileId::IBeam => Self::ibeam(),
            ModalProfileId::TautCable => Self::taut_cable(),
            ModalProfileId::CoilSpring => Self::coil_spring(),
            ModalProfileId::SheetMetal => Self::sheet_metal(),
            ModalProfileId::IndustrialCog => Self::industrial_cog(),
        }
    }

    /// Return the canonical pipe profile.
    pub const fn pipe() -> Self {
        Self {
            id: ModalProfileId::Pipe,
            modes: &PIPE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical plate profile.
    pub const fn plate() -> Self {
        Self {
            id: ModalProfileId::Plate,
            modes: &PLATE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical tank profile.
    pub const fn tank() -> Self {
        Self {
            id: ModalProfileId::Tank,
            modes: &TANK_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical chain profile.
    pub const fn chain() -> Self {
        Self {
            id: ModalProfileId::Chain,
            modes: &CHAIN_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical I-beam profile.
    pub const fn ibeam() -> Self {
        Self {
            id: ModalProfileId::IBeam,
            modes: &IBEAM_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical taut-cable profile.
    pub const fn taut_cable() -> Self {
        Self {
            id: ModalProfileId::TautCable,
            modes: &TAUTCABLE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical coil-spring profile.
    pub const fn coil_spring() -> Self {
        Self {
            id: ModalProfileId::CoilSpring,
            modes: &COILSPRING_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical sheet-metal profile.
    pub const fn sheet_metal() -> Self {
        Self {
            id: ModalProfileId::SheetMetal,
            modes: &SHEETMETAL_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the canonical industrial-cog profile.
    pub const fn industrial_cog() -> Self {
        Self {
            id: ModalProfileId::IndustrialCog,
            modes: &COG_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    /// Return the number of canonical modes in the profile.
    pub fn mode_count(&self) -> usize {
        self.modes.len()
    }

    /// Return the canonical mode slice.
    pub fn modes(&self) -> &[ModalModeSpec] {
        self.modes
    }

    /// Expand the profile with size, rust, and damage controls.
    pub fn scaled_mode_specs(
        &self,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
    ) -> Vec<ModalModeSpec> {
        self.scaled_mode_specs_extended(
            size_scale,
            rust_amount,
            damage_amount,
            crate::dsp::ThicknessAmount::default(),
            crate::dsp::HeatAmount::default(),
            crate::dsp::SludgeAmount::default(),
        )
    }

    /// Expand the profile with all available control transforms.
    pub fn scaled_mode_specs_extended(
        &self,
        size_scale: crate::dsp::SizeScale,
        rust_amount: crate::dsp::RustAmount,
        damage_amount: crate::dsp::DamageAmount,
        thickness_amount: crate::dsp::ThicknessAmount,
        heat_amount: crate::dsp::HeatAmount,
        sludge_amount: crate::dsp::SludgeAmount,
    ) -> Vec<ModalModeSpec> {
        let mode_count = self.modes.len();
        let mut result = Vec::new();

        for (i, mode) in self.modes.iter().enumerate() {
            let sized = mode.scaled_for_size(size_scale, i, mode_count);
            let rusted = sized.corroded(rust_amount, i, mode_count);
            let thickened = rusted.thickened(thickness_amount, i);
            let heated = thickened.heated(heat_amount);
            let sludge_loaded = heated.sludge_loaded(sludge_amount);
            let damaged = sludge_loaded.damaged(damage_amount, i, mode_count);
            result.extend(damaged);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modal_profile_pipe_has_modes() {
        let profile = ModalProfile::pipe();
        assert!(!profile.modes().is_empty());
    }

    #[test]
    fn modal_profile_pipe_modes_are_finite() {
        let profile = ModalProfile::pipe();
        for mode in profile.modes() {
            assert!(mode.frequency_hz.is_finite());
            assert!(mode.decay_seconds.is_finite());
            assert!(mode.gain.is_finite());
            assert!(mode.frequency_hz > 0.0);
            assert!(mode.decay_seconds > 0.0);
            assert!(mode.gain >= 0.0);
        }
    }

    #[test]
    fn modal_profile_tank_modes_are_finite() {
        let profile = ModalProfile::tank();
        for mode in profile.modes() {
            assert!(mode.frequency_hz.is_finite());
            assert!(mode.decay_seconds.is_finite());
            assert!(mode.gain.is_finite());
            assert!(mode.frequency_hz > 0.0);
            assert!(mode.decay_seconds > 0.0);
            assert!(mode.gain >= 0.0);
        }
    }

    #[test]
    fn modal_profile_modes_are_finite_for_all_profiles() {
        let profiles = [
            ModalProfile::pipe(),
            ModalProfile::plate(),
            ModalProfile::tank(),
            ModalProfile::chain(),
            ModalProfile::ibeam(),
            ModalProfile::taut_cable(),
            ModalProfile::coil_spring(),
            ModalProfile::sheet_metal(),
            ModalProfile::industrial_cog(),
        ];
        for profile in profiles {
            for mode in profile.modes() {
                assert!(mode.frequency_hz.is_finite(), "frequency_hz not finite for {:?}", profile.id);
                assert!(mode.decay_seconds.is_finite(), "decay_seconds not finite for {:?}", profile.id);
                assert!(mode.gain.is_finite(), "gain not finite for {:?}", profile.id);
                assert!(mode.frequency_hz > 0.0, "frequency_hz <= 0 for {:?}", profile.id);
                assert!(mode.decay_seconds > 0.0, "decay_seconds <= 0 for {:?}", profile.id);
                assert!(mode.gain >= 0.0, "gain < 0 for {:?}", profile.id);
            }
        }
    }

    #[test]
    fn scaled_for_size_preserves_finite() {
        let mode = ModalModeSpec::new(440.0, 1.0, 0.01);
        let scaled = mode.scaled_for_size(crate::dsp::SizeScale::new(1.5), 0, 3);
        assert!(scaled.frequency_hz.is_finite());
        assert!(scaled.decay_seconds.is_finite());
        assert!(scaled.gain.is_finite());
        assert!(scaled.frequency_hz > 0.0);
        assert!(scaled.decay_seconds > 0.0);
        assert!(scaled.gain >= 0.0);
    }

    #[test]
    fn corroded_preserves_finite() {
        let mode = ModalModeSpec::new(440.0, 1.0, 0.01);
        let corroded = mode.corroded(crate::dsp::RustAmount::new(0.5), 1, 3);
        assert!(corroded.frequency_hz.is_finite());
        assert!(corroded.decay_seconds.is_finite());
        assert!(corroded.gain.is_finite());
        assert!(corroded.frequency_hz > 0.0);
        assert!(corroded.decay_seconds > 0.0);
        assert!(corroded.gain >= 0.0);
    }

    #[test]
    fn damaged_expands_and_preserves_finite() {
        let mode = ModalModeSpec::new(440.0, 1.0, 0.01);
        let damaged = mode.damaged(crate::dsp::DamageAmount::new(0.5), 1, 3);
        assert!(damaged.len() >= 1, "damaged() should return at least the primary mode");
        for m in &damaged {
            assert!(m.frequency_hz.is_finite(), "frequency_hz not finite");
            assert!(m.decay_seconds.is_finite(), "decay_seconds not finite");
            assert!(m.gain.is_finite(), "gain not finite");
            assert!(m.frequency_hz > 0.0, "frequency_hz <= 0");
            assert!(m.decay_seconds > 0.0, "decay_seconds <= 0");
            assert!(m.gain >= 0.0, "gain < 0");
        }
    }

    #[test]
    fn damaged_with_zero_returns_single_mode() {
        let mode = ModalModeSpec::new(440.0, 1.0, 0.01);
        let damaged = mode.damaged(crate::dsp::DamageAmount::new(0.0), 1, 3);
        assert_eq!(damaged.len(), 1);
        assert_eq!(damaged[0].frequency_hz, mode.frequency_hz);
        assert_eq!(damaged[0].decay_seconds, mode.decay_seconds);
        assert_eq!(damaged[0].gain, mode.gain);
    }
}
