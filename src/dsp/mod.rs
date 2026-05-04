pub mod body;
pub mod budget;
pub mod deterministic_excitation;
pub mod exciters;
pub mod profile;
pub mod resonator;
pub mod transforms;

pub use body::BodyResonator;
pub use budget::{
    offline_peak_shared_mode_limit, realtime_mode_count_estimate, realtime_mode_count_estimates,
    safe_realtime_shared_mode_limit, RealtimeModeCountEstimate,
};
pub use deterministic_excitation::ExcitationInput;
pub use exciters::{Exciter, ScrapeExciter};
pub use profile::{ModalModeSpec, ModalProfile, ModalProfileId};
pub use resonator::{ModalResonator, ResonatorCore, ResonatorCoefficients, SecondOrderMode};
pub use transforms::{DamageAmount, RustAmount, SizeScale};

pub use crate::offline::{
    DamageVariationRenderSpec, RustVariationRenderSpec, DAMAGE_VARIATION_SPECS,
    FAMILY_COMPARISON_SPECS, RUST_VARIATION_SPECS,
};

#[cfg(test)]
pub(crate) use profile::{
    PIPE_MODAL_PROFILE_MODES, PLATE_MODAL_PROFILE_MODES, TANK_MODAL_PROFILE_MODES,
};

#[cfg(test)]
mod tests;
