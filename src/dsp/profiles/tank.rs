use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const TANK_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(96.0, 2.5, 0.0260),
    ModalModeSpec::new(151.0, 2.1, 0.0218),
    ModalModeSpec::new(226.0, 1.75, 0.0178),
    ModalModeSpec::new(318.0, 1.40, 0.0139),
    ModalModeSpec::new(439.0, 1.05, 0.0104),
    ModalModeSpec::new(588.0, 0.80, 0.0077),
    ModalModeSpec::new(774.0, 0.58, 0.0056),
    ModalModeSpec::new(1_002.0, 0.45, 0.0040),
];

pub const fn tank() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::Tank,
        modes: &TANK_MODAL_PROFILE_MODES,
    }
}
