use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const PIPE_MODAL_PROFILE_MODES: [ModalModeSpec; 6] = [
    ModalModeSpec::new(220.0, 2.0, 0.0152),
    ModalModeSpec::new(439.5, 1.60, 0.0135),
    ModalModeSpec::new(660.0, 1.25, 0.0112),
    ModalModeSpec::new(881.0, 0.95, 0.0088),
    ModalModeSpec::new(1_103.0, 0.72, 0.0066),
    ModalModeSpec::new(1_327.0, 0.52, 0.0048),
];

pub const fn pipe() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::Pipe,
        modes: &PIPE_MODAL_PROFILE_MODES,
    }
}
