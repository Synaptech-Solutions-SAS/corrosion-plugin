use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const COG_MODAL_PROFILE_MODES: [ModalModeSpec; 12] = [
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

pub const fn industrial_cog() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::IndustrialCog,
        modes: &COG_MODAL_PROFILE_MODES,
    }
}
