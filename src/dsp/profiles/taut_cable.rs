use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const TAUTCABLE_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(110.0, 1.25, 0.0250),
    ModalModeSpec::new(220.0, 1.00, 0.0150),
    ModalModeSpec::new(330.0, 0.80, 0.0100),
    ModalModeSpec::new(440.0, 0.60, 0.0070),
    ModalModeSpec::new(550.0, 0.45, 0.0050),
    ModalModeSpec::new(660.0, 0.35, 0.0040),
    ModalModeSpec::new(770.0, 0.25, 0.0030),
    ModalModeSpec::new(880.0, 0.20, 0.0025),
];

pub const fn taut_cable() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::TautCable,
        modes: &TAUTCABLE_MODAL_PROFILE_MODES,
    }
}
