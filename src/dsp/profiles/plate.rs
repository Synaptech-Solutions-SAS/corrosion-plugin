use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const PLATE_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(286.0, 0.47, 0.0092),
    ModalModeSpec::new(463.0, 0.41, 0.0089),
    ModalModeSpec::new(731.0, 0.36, 0.0084),
    ModalModeSpec::new(1_036.0, 0.30, 0.0076),
    ModalModeSpec::new(1_394.0, 0.26, 0.0068),
    ModalModeSpec::new(1_811.0, 0.22, 0.0059),
    ModalModeSpec::new(2_297.0, 0.18, 0.0050),
    ModalModeSpec::new(2_860.0, 0.15, 0.0042),
];

pub const fn plate() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::Plate,
        modes: &PLATE_MODAL_PROFILE_MODES,
    }
}
