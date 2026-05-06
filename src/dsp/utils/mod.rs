//! Utility DSP modules for testing and offline rendering.
//!
//! These modules are not used in the real-time plugin path but provide
//! utilities for deterministic testing, offline rendering, and budget
//! estimation.

pub mod body;
pub mod budget;
pub mod deterministic_excitation;

pub use body::BodyResonator;
pub use budget::{
    offline_peak_shared_mode_limit, realtime_mode_count_estimate, realtime_mode_count_estimates,
    safe_realtime_shared_mode_limit, RealtimeModeCountEstimate,
};
pub use deterministic_excitation::ExcitationInput;
