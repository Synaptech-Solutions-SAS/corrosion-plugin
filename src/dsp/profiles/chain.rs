use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const CHAIN_MODAL_PROFILE_MODES: [ModalModeSpec; 10] = [
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

pub const fn chain() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::Chain,
        modes: &CHAIN_MODAL_PROFILE_MODES,
    }
}
