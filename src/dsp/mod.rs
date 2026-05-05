//! DSP core module.
//!
//! This module exports the modal object models, excitation primitives,
//! resonator math, envelope generation, and realtime budget helpers used by
//! the voice layer. The voice module owns note lifecycle and sample-rate
//! processing; these types provide the per-note DSP building blocks.

pub mod body;
pub mod budget;
pub mod deterministic_excitation;
pub mod exciters;
pub mod expanded_resonators;
pub mod interaction;
pub mod mseg;
pub mod post_processing;
pub mod profile;
pub mod resonator;
pub mod transforms;

pub use body::BodyResonator;
pub use budget::{
    offline_peak_shared_mode_limit, realtime_mode_count_estimate, realtime_mode_count_estimates,
    safe_realtime_shared_mode_limit, RealtimeModeCountEstimate,
};
pub use deterministic_excitation::ExcitationInput;
pub use exciters::{
    CorrugatedDrag, Drumstick, ElectromagneticHum, Exciter, FeltMallet, HandStrike, HardMallet,
    HeavyGrinding, MetalChain, MetalPipe, ParticleRain, PneumaticJet, ScrapeExciter,
    StiffPointScrape, TensionRise, TensionSnap, WireBrush,
};
pub use expanded_resonators::{
    CoilSpringResonator, IBeamResonator, IndustrialCogResonator, SheetMetalResonator,
    TautCableResonator,
};
pub use interaction::{
    mode_coefficient_1d, mode_coefficient_2d, BidirectionalInteractionBus, InteractionState,
};
pub use mseg::{LoopMode, Stage, MSEG};
pub use post_processing::{
    FactoryReverb, FemBodyResonator, HrtfSpread, LorenzDrive, OversampledClipper,
    PostProcessingChain, SpaceMode, SpringReverb, WdfLadderFilter,
};
pub use profile::{ModalModeSpec, ModalProfile, ModalProfileId};
pub use resonator::{ModalResonator, ResonatorCoefficients, ResonatorCore, SecondOrderMode};
pub use transforms::{
    DamageAmount, HeatAmount, RustAmount, SizeScale, SludgeAmount, ThicknessAmount,
};

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
