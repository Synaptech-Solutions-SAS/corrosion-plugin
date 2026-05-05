//! Exciter registry for the DSP layer.
//!
//! This module groups the exciter families used by the physical-modeling engine
//! and exposes a stable enum/lookup layer for the UI, presets, and voice logic.

pub mod corrugated_drag;
pub mod drumstick;
pub mod felt_mallet;
pub mod hand_strike;
pub mod hard_mallet;
pub mod heavy_grinding;
pub mod metal_chain;
pub mod metal_pipe;
pub mod other_specialty;
pub mod scrape;
pub mod stiff_point;
pub mod tension_rise;
pub mod wire_brush;

pub use corrugated_drag::CorrugatedDrag;
pub use drumstick::Drumstick;
pub use felt_mallet::FeltMallet;
pub use hand_strike::HandStrike;
pub use hard_mallet::HardMallet;
pub use heavy_grinding::HeavyGrinding;
pub use metal_chain::MetalChain;
pub use metal_pipe::MetalPipe;
pub use other_specialty::{ElectromagneticHum, ParticleRain, PneumaticJet, TensionSnap};
pub use scrape::ScrapeExciter;
pub use stiff_point::StiffPointScrape;
pub use tension_rise::TensionRise;
pub use wire_brush::WireBrush;

/// Named exciter families used by the instrument.
///
/// The enum is intentionally UI-friendly: it carries the coarse family name,
/// a canonical string label, and a compact integer representation for preset
/// storage and host-facing serialization.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Exciter {
    /// Legacy hit bucket used as the default fallback.
    Hit,
    /// Legacy scrape bucket used as the default fallback.
    Scrape,
    /// Fleshy contact with a highly damped spring response.
    HandStrike,
    /// Soft non-linear mallet contact.
    FeltMallet,
    /// Hard rigid mallet with Hertzian contact.
    HardMallet,
    /// Light rigid stick with micro-bounce behavior.
    Drumstick,
    /// Stochastic wire-brush impulse cluster.
    WireBrush,
    /// Metal-on-metal coupled resonant impact.
    MetalPipe,
    /// Multi-link impact train with rattle noise.
    MetalChain,
    /// Rigid point scrape with chatter.
    StiffPoint,
    /// Friction-heavy grinding with tearing noise.
    HeavyGrinding,
    /// Drag across a corrugated surface.
    CorrugatedDrag,
    /// Slow tension build-up with slip bursts.
    TensionRise,
    /// Pressurized air/steam turbulence exciter.
    PneumaticJet,
    /// Continuous electromagnetic drive.
    ElectromagneticHum,
    /// Stored tension that releases in a snap.
    TensionSnap,
    /// Asynchronous granular particle emission.
    ParticleRain,
}

impl Exciter {
    /// Converts a host integer into an exciter variant.
    ///
    /// Unknown values fall back to [`Exciter::Hit`] so preset loading remains
    /// resilient to stale or future enum values.
    pub fn from_int(v: i32) -> Self {
        match v {
            0 => Exciter::Hit,
            1 => Exciter::Scrape,
            2 => Exciter::HandStrike,
            3 => Exciter::FeltMallet,
            4 => Exciter::HardMallet,
            5 => Exciter::Drumstick,
            6 => Exciter::WireBrush,
            7 => Exciter::MetalPipe,
            8 => Exciter::MetalChain,
            9 => Exciter::StiffPoint,
            10 => Exciter::HeavyGrinding,
            11 => Exciter::CorrugatedDrag,
            12 => Exciter::TensionRise,
            13 => Exciter::PneumaticJet,
            14 => Exciter::ElectromagneticHum,
            15 => Exciter::TensionSnap,
            16 => Exciter::ParticleRain,
            _ => Exciter::Hit,
        }
    }

    /// Converts the exciter into its stable integer tag.
    pub fn to_int(&self) -> i32 {
        match self {
            Exciter::Hit => 0,
            Exciter::Scrape => 1,
            Exciter::HandStrike => 2,
            Exciter::FeltMallet => 3,
            Exciter::HardMallet => 4,
            Exciter::Drumstick => 5,
            Exciter::WireBrush => 6,
            Exciter::MetalPipe => 7,
            Exciter::MetalChain => 8,
            Exciter::StiffPoint => 9,
            Exciter::HeavyGrinding => 10,
            Exciter::CorrugatedDrag => 11,
            Exciter::TensionRise => 12,
            Exciter::PneumaticJet => 13,
            Exciter::ElectromagneticHum => 14,
            Exciter::TensionSnap => 15,
            Exciter::ParticleRain => 16,
        }
    }

    /// Returns the human-readable label used by the UI.
    pub fn name(&self) -> &'static str {
        match self {
            Exciter::Hit => "Hit",
            Exciter::Scrape => "Scrape",
            Exciter::HandStrike => "Hand Strike",
            Exciter::FeltMallet => "Felt Mallet",
            Exciter::HardMallet => "Hard Mallet",
            Exciter::Drumstick => "Drumstick",
            Exciter::WireBrush => "Wire Brush",
            Exciter::MetalPipe => "Metal Pipe",
            Exciter::MetalChain => "Metal Chain",
            Exciter::StiffPoint => "Stiff Point",
            Exciter::HeavyGrinding => "Heavy Grinding",
            Exciter::CorrugatedDrag => "Corrugated Drag",
            Exciter::TensionRise => "Tension Rise",
            Exciter::PneumaticJet => "Pneumatic Jet",
            Exciter::ElectromagneticHum => "Electromagnetic Hum",
            Exciter::TensionSnap => "Tension Snap",
            Exciter::ParticleRain => "Particle Rain",
        }
    }

    /// Returns the coarse family bucket used by the UI.
    pub fn category(&self) -> &'static str {
        match self {
            Exciter::Hit
            | Exciter::HandStrike
            | Exciter::FeltMallet
            | Exciter::HardMallet
            | Exciter::Drumstick
            | Exciter::WireBrush
            | Exciter::MetalPipe
            | Exciter::MetalChain => "Hit",
            Exciter::Scrape
            | Exciter::StiffPoint
            | Exciter::HeavyGrinding
            | Exciter::CorrugatedDrag
            | Exciter::TensionRise => "Scrape",
            Exciter::PneumaticJet
            | Exciter::ElectromagneticHum
            | Exciter::TensionSnap
            | Exciter::ParticleRain => "Other",
        }
    }

    /// Returns every exposed exciter in UI order.
    pub fn all_exciters() -> Vec<Exciter> {
        vec![
            Exciter::Hit,
            Exciter::HandStrike,
            Exciter::FeltMallet,
            Exciter::HardMallet,
            Exciter::Drumstick,
            Exciter::WireBrush,
            Exciter::MetalPipe,
            Exciter::MetalChain,
            Exciter::Scrape,
            Exciter::StiffPoint,
            Exciter::HeavyGrinding,
            Exciter::CorrugatedDrag,
            Exciter::TensionRise,
            Exciter::PneumaticJet,
            Exciter::ElectromagneticHum,
            Exciter::TensionSnap,
            Exciter::ParticleRain,
        ]
    }
}
