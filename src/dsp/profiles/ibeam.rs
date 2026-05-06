use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const IBEAM_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(55.0, 0.90, 0.0200),
    ModalModeSpec::new(110.0, 0.75, 0.0180),
    ModalModeSpec::new(165.0, 0.60, 0.0150),
    ModalModeSpec::new(220.0, 0.45, 0.0120),
    ModalModeSpec::new(275.0, 0.35, 0.0090),
    ModalModeSpec::new(330.0, 0.25, 0.0060),
    ModalModeSpec::new(385.0, 0.20, 0.0040),
    ModalModeSpec::new(440.0, 0.15, 0.0030),
];

pub const fn ibeam() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::IBeam,
        modes: &IBEAM_MODAL_PROFILE_MODES,
    }
}
