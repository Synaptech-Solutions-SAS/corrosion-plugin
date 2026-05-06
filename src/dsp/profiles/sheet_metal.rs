use crate::dsp::{ModalModeSpec, ModalProfile, ModalProfileId};

pub const SHEETMETAL_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(60.0, 1.50, 0.0300),
    ModalModeSpec::new(120.0, 1.25, 0.0200),
    ModalModeSpec::new(180.0, 1.00, 0.0150),
    ModalModeSpec::new(240.0, 0.80, 0.0110),
    ModalModeSpec::new(300.0, 0.65, 0.0080),
    ModalModeSpec::new(360.0, 0.50, 0.0060),
    ModalModeSpec::new(420.0, 0.40, 0.0045),
    ModalModeSpec::new(480.0, 0.30, 0.0035),
];

pub const fn sheet_metal() -> ModalProfile {
    ModalProfile {
        id: ModalProfileId::SheetMetal,
        modes: &SHEETMETAL_MODAL_PROFILE_MODES,
    }
}
