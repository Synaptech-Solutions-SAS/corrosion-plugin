use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const COILSPRING_MODAL_PROFILE_MODES: [ModalModeSpec; 10] = [
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

pub const fn coil_spring() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::CoilSpring,
        modes: &COILSPRING_MODAL_PROFILE_MODES,
    }
}
