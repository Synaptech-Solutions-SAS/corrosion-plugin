//! # Parameter Definitions Module
//!
//! This module defines all user-facing parameters for the Corrosion synthesizer.
//! It provides the bridge between the host DAW (via nih_plug) and the internal
//! DSP algorithms, handling parameter serialization, automation, and UI integration.
//!
//! ## Parameter Categories
//!
//! The 70+ parameters are organized into logical groups:
//!
//! ### Sound Generation
//! - **Exciter** (exciter) - Type of excitation (Hit/Scrape/Specialty variants)
//! - **Object** (object) - Resonator object type (Pipe, Plate, Tank, etc.)
//!
//! ### Object Transformations
//! - **Size** (size) - Overall scale of the resonator (0.05x to 10x)
//! - **Rust** (rust) - Corrosion amount affecting high frequencies and decay
//! - **Damage** (damage) - Structural damage introducing rattle and chaos
//!
//! ### Exciter Controls
//! - **Pressure** (exciter_pressure) - Force of excitation
//! - **Speed** (exciter_speed) - Velocity for scrape-type exciters
//! - **Roughness** (exciter_roughness) - Surface texture and grit
//!
//! ### Envelope Generators
//! Standard ADSR for Hit/Specialty exciters:
//! - **Attack** (env_attack) - Time to reach peak (0.001s - 2s)
//! - **Decay** (env_decay) - Time to reach sustain (0.01s - 5s)
//! - **Sustain** (env_sustain) - Hold level (0.0 - 1.0)
//! - **Release** (env_release) - Time to silence (0.01s - 5s)
//!
//! MSEG (Multistage Envelope) for Scrape exciters:
//! - **Onset, Attack, Hold, Decay, Sustain, Release** - 6-stage envelope
//! - **Loop Mode** - Off, Forward, PingPong (partial)
//!
//! ### Post-Processing Chain
//! - **Filter** (filter_cutoff, filter_resonance) - Lowpass resonant filter
//! - **Drive** (drive, drive_amount) - Saturation and waveshaping
//! - **Body** (body) - Physical body resonance simulation
//! - **Spread** (width, spread_width) - Stereo width enhancement
//! - **Space** (space_mode, space_amount) - Reverb/echo effects
//!
//! ### Interaction Parameters
//! - **Strike Position** (strike_position) - Where exciter contacts object
//! - **Coupling** (coupling_stiffness) - Bidirectional interaction strength
//! - **Wander** (position_wander) - Simulated hand unsteadiness
//!
//! ## Relationships
//!
//! This module connects to:
//! - `lib.rs` - Parameter values read during audio processing
//! - `voice/mod.rs` - VoiceControls struct bundles parameters for voices
//! - `gui/editor.rs` - Parameters displayed and edited in UI
//! - `presets/mod.rs` - Parameters serialized/deserialized as presets
//!
//! ## Usage
//!
//! Parameters are accessed via the `CorrosionParams` struct which implements
//! nih_plug's `Params` trait for host integration:
//!
//! ```rust
//! let cutoff = plugin.params.filter_cutoff.value();
//! let exciter_type = plugin.params.exciter.value();
//! ```

use nih_plug::prelude::*;
#[cfg(feature = "gui")]
use nih_plug_egui::EguiState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Main parameter struct containing all user-facing controls.
///
/// This struct implements nih_plug's `Params` trait, allowing the host DAW
/// to automate and save parameter values. Each field corresponds to a
/// controllable parameter in the synthesizer.
///
/// The `#[derive(Params)]` macro generates the necessary trait implementations
/// for host integration, including parameter enumeration and value conversion.
#[derive(Params)]
pub struct CorrosionParams {
    /// GUI editor state persistence (window size, position)
    /// Only available when the "gui" feature is enabled
    #[cfg(feature = "gui")]
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    
    /// Exciter type selection (0-16 mapping to ExciterType enum)
    /// Controls the type of physical excitation (mallets, scrapes, specialty)
    #[id = "exciter"]
    pub exciter: IntParam,
    
    /// Resonator object type (0-8 mapping to Object enum)
    /// Determines the modal characteristics (pipe vs plate vs tank, etc.)
    #[id = "object"]
    pub object: IntParam,
    
    /// Object size scale (0.05 to 10.0)
    /// Affects pitch (inverse) and decay time (direct)
    /// Higher values = larger object = lower pitch, longer decay
    #[id = "size"]
    pub size: FloatParam,
    
    /// Rust amount (0.0 to 5.0)
    /// Simulates corrosion that damps high frequencies
    /// Higher values = duller, more muted sound
    #[id = "rust"]
    pub rust: FloatParam,
    
    /// Damage amount (0.0 to 10.0)
    /// Structural damage introducing non-linear rattle and buzzing
    /// Higher values = more industrial chaos
    #[id = "damage"]
    pub damage: FloatParam,
    
    /// Output drive/saturation (0.0 to 5.0)
    /// Asymmetric waveshaping for industrial character
    #[id = "drive"]
    pub drive: FloatParam,
    
    /// Master output gain in dB (mapped to linear 0.0 to +40dB)
    /// Final stage before the limiter
    #[id = "output"]
    pub output: FloatParam,
    
    /// Stereo width/spread (-2.0 to 3.0)
    /// Controls stereo field expansion in post-processing
    #[id = "width"]
    pub width: FloatParam,
    
    /// Body resonance amount (0.0 to 5.0)
    /// Physical cabinet/body resonance simulation
    #[id = "body"]
    pub body: FloatParam,
    
    /// UI scale factor (0-4 mapping to 50%, 75%, 100%, 125%, 150%)
    /// Only affects the editor GUI size
    #[id = "ui_scale"]
    pub ui_scale: IntParam,
    
    // ADSR Envelope Parameters (for Hit and Specialty exciters)
    
    /// Envelope attack time in seconds (0.001 to 2.0)
    /// Time from zero to peak level
    #[id = "env_attack"]
    pub env_attack: FloatParam,
    
    /// Envelope decay time in seconds (0.01 to 5.0)
    /// Time from peak to sustain level
    #[id = "env_decay"]
    pub env_decay: FloatParam,
    
    /// Envelope sustain level (0.0 to 1.0)
    /// Hold level while note is active
    #[id = "env_sustain"]
    pub env_sustain: FloatParam,
    
    /// Envelope release time in seconds (0.01 to 5.0)
    /// Time from sustain to silence after note-off
    #[id = "env_release"]
    pub env_release: FloatParam,
    
    // MSEG (Multistage Envelope) Parameters (for Scrape exciters)
    
    /// MSEG onset time - initial transient stage (0.001 to 1.0s)
    #[id = "mseg_onset"]
    pub mseg_onset: FloatParam,
    
    /// MSEG attack time - main rise stage (0.001 to 2.0s)
    #[id = "mseg_attack"]
    pub mseg_attack: FloatParam,
    
    /// MSEG hold time - peak hold stage (0.0 to 2.0s)
    #[id = "mseg_hold"]
    pub mseg_hold: FloatParam,
    
    /// MSEG decay time - fall to sustain (0.01 to 5.0s)
    #[id = "mseg_decay"]
    pub mseg_decay: FloatParam,
    
    /// MSEG sustain level (0.0 to 1.0)
    #[id = "mseg_sustain"]
    pub mseg_sustain: FloatParam,
    
    /// MSEG release time (0.01 to 5.0s)
    #[id = "mseg_release"]
    pub mseg_release: FloatParam,
    
    // Envelope Modulation
    
    /// Envelope amount - scales envelope influence on excitation (0.0 to 1.0)
    #[id = "env_amount"]
    pub env_amount: FloatParam,
    
    /// Velocity to peak amount - how much velocity affects envelope peak (0.0 to 1.0)
    #[id = "velocity_to_peak"]
    pub velocity_to_peak: FloatParam,
    
    /// Loop mode for MSEG (0=Off, 1=Forward, 2=PingPong)
    #[id = "loop_mode"]
    pub loop_mode: IntParam,
    
    /// Loop start stage (0-5, default 3=Decay)
    #[id = "loop_start_stage"]
    pub loop_start_stage: IntParam,
    
    /// Loop end stage (0-5, default 4=Sustain)
    #[id = "loop_end_stage"]
    pub loop_end_stage: IntParam,
    
    /// Sync rate for tempo-synced parameters (0.0 to 1.0)
    #[id = "sync_rate"]
    pub sync_rate: FloatParam,
    
    /// Global time scale multiplier (0.1 to 10.0)
    /// Scales all envelope times proportionally
    #[id = "global_time_scale"]
    pub global_time_scale: FloatParam,
    
    /// Velocity to envelope level amount (0.0 to 1.0)
    #[id = "velocity_to_level"]
    pub velocity_to_level: FloatParam,
    
    /// Velocity to envelope time amount (0.0 to 1.0)
    /// Higher velocities = shorter times
    #[id = "velocity_to_time"]
    pub velocity_to_time: FloatParam,
    
    /// Curve tension for envelope stages (-1.0 to 1.0)
    /// Negative = exponential, 0 = linear, positive = logarithmic
    #[id = "curve_tension"]
    pub curve_tension: FloatParam,
    
    // Exciter Controls
    
    /// Exciter pressure/force (0.0 to 1.0)
    /// Overall intensity of excitation
    #[id = "exciter_pressure"]
    pub exciter_pressure: FloatParam,
    
    /// Exciter speed (0.0 to 1.0)
    /// Velocity for scrape-type exciters (bow, scrape, drag)
    #[id = "exciter_speed"]
    pub exciter_speed: FloatParam,
    
    /// Exciter roughness (0.0 to 1.0)
    /// Surface texture - affects grit, chatter, and friction
    #[id = "exciter_roughness"]
    pub exciter_roughness: FloatParam,
    
    // Interaction Parameters
    
    /// Strike position on resonator (0.0 to 1.0)
    /// 0.0 = edge, 0.5 = center, 1.0 = other edge
    #[id = "strike_position"]
    pub strike_position: FloatParam,
    
    /// Coupling stiffness (0.0 to 1.0)
    /// Bidirectional interaction strength between exciter and resonator
    #[id = "coupling_stiffness"]
    pub coupling_stiffness: FloatParam,
    
    /// Position wander amount (0.0 to 1.0)
    /// Simulates hand unsteadiness during excitation
    #[id = "position_wander"]
    pub position_wander: FloatParam,
    
    /// Position envelope amount (0.0 to 1.0)
    /// How much envelope affects strike position
    #[id = "position_envelope"]
    pub position_envelope: FloatParam,
    
    /// Fundamental frequency anchor (0.0 to 1.0)
    /// Minimum coefficient for fundamental mode to prevent nulls
    #[id = "fundamental_anchor"]
    pub fundamental_anchor: FloatParam,
    
    // Resonator Controls
    
    /// Resonator damping (0.0 to 1.0)
    /// Overall decay time modifier
    #[id = "res_damping"]
    pub res_damping: FloatParam,
    
    /// Resonator brightness (0.0 to 1.0)
    /// High frequency emphasis
    #[id = "res_brightness"]
    pub res_brightness: FloatParam,
    
    // Advanced Transformations
    
    /// Material thickness (0.0 to 1.0)
    /// Affects stiffness and frequency distribution
    #[id = "thickness"]
    pub thickness: FloatParam,
    
    /// Heat amount (0.0 to 1.0)
    /// Thermal effects - detuning and damping
    #[id = "heat"]
    pub heat: FloatParam,
    
    /// Sludge amount (0.0 to 1.0)
    /// Viscous damping effect
    #[id = "sludge"]
    pub sludge: FloatParam,
    
    // Post-Processing Filter
    
    /// Filter cutoff frequency in Hz (20 to 20000, skewed)
    /// Lowpass filter before drive stage
    #[id = "filter_cutoff"]
    pub filter_cutoff: FloatParam,
    
    /// Filter resonance (0.0 to 1.0)
    /// Peaking response at cutoff frequency
    #[id = "filter_resonance"]
    pub filter_resonance: FloatParam,
    
    /// Component tolerance (0.0 to 1.0)
    /// Analog component variation for vintage character
    #[id = "component_tolerance"]
    pub component_tolerance: FloatParam,
    
    // Drive/Post-Processing
    
    /// Drive amount in post-processing (0.0 to 5.0)
    /// Separate from exciter drive - affects final output
    #[id = "drive_amount"]
    pub drive_amount: FloatParam,
    
    /// Bias starvation (0.0 to 1.0)
    /// Circuit bias reduction for asymmetric distortion
    #[id = "bias_starvation"]
    pub bias_starvation: FloatParam,
    
    /// Chaos depth (0.0 to 1.0)
    /// Lorenz attractor modulation for chaotic drive
    #[id = "chaos_depth"]
    pub chaos_depth: FloatParam,
    
    // Stereo/Spatial
    
    /// Spread width (0.0 to 1.0)
    /// Stereo width from HRTF-based processing
    #[id = "spread_width"]
    pub spread_width: FloatParam,
    
    /// Listener proximity (0.0 to 1.0)
    /// Near-field effect strength
    #[id = "listener_proximity"]
    pub listener_proximity: FloatParam,
    
    // Body Resonance
    
    /// Chassis material (0.0 to 1.0)
    /// Body resonance material selection
    #[id = "chassis_material"]
    pub chassis_material: FloatParam,
    
    /// Chassis volume (0.0 to 1.0)
    /// Body resonance intensity
    #[id = "chassis_volume"]
    pub chassis_volume: FloatParam,
    
    // Space/Reverb
    
    /// Space mode (0=Off, 1=Factory, 2=Spring, 3=Echo)
    #[id = "space_mode"]
    pub space_mode: IntParam,
    
    /// Space/reverb amount (0.0 to 1.0)
    /// Wet/dry mix for spatial effects
    #[id = "space_amount"]
    pub space_amount: FloatParam,
    
    /// Factory reverb size (0.0 to 1.0)
    /// Room size for factory reverb algorithm
    #[id = "factory_size"]
    pub factory_size: FloatParam,
    
    /// Machinery clutter (0.0 to 1.0)
    /// Industrial equipment density in reverb
    #[id = "machinery_clutter"]
    pub machinery_clutter: FloatParam,
    
    /// Wall impedance (0.0 to 1.0)
    /// Surface reflectivity for reverb
    #[id = "wall_impedance"]
    pub wall_impedance: FloatParam,
    
    /// Spring reverb tension (0.0 to 1.0)
    /// Spring stiffness in spring reverb
    #[id = "spring_tension"]
    pub spring_tension: FloatParam,
    
    /// Wire stiffness (0.0 to 1.0)
    /// Spring wire thickness in spring reverb
    #[id = "wire_stiffness"]
    pub wire_stiffness: FloatParam,
    
    /// Spring tank size (0.0 to 1.0)
    /// Physical spring tank dimensions
    #[id = "spring_tank_size"]
    pub spring_tank_size: FloatParam,
    
    /// Echo delay time (0.0 to 1.0)
    /// Normalized delay time for echo mode
    #[id = "delay_time"]
    pub delay_time: FloatParam,
    
    /// Machinery movement (0.0 to 1.0)
    /// Doppler/modulation for echo delays
    #[id = "machinery_movement"]
    pub machinery_movement: FloatParam,
    
    /// High frequency damping (0.0 to 1.0)
    /// Air absorption in spatial effects
    #[id = "high_frequency_damping"]
    pub high_frequency_damping: FloatParam,
    
    /// Analog ceiling (0.5 to 1.0)
    /// Clipper threshold for final output stage
    #[id = "analog_ceiling"]
    pub analog_ceiling: FloatParam,
    
    /// Diode softness (0.0 to 1.0)
    /// Transition hardness in output clipper
    #[id = "diode_softness"]
    pub diode_softness: FloatParam,
}

/// Exciter type enumeration for serialization and UI display.
///
/// Maps integer parameter values (0-16) to named exciter types.
/// Organized into three families:
/// - **Hit** (0-8): Impact-based excitation
/// - **Scrape** (1, 9-12): Continuous friction excitation
/// - **Specialty** (13-16): Non-traditional industrial excitation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ExciterType {
    /// Generic hit exciter
    Hit,
    /// Bow/scrape friction model
    Scrape,
    /// Hand strike - flesh-on-object impact
    HandStrike,
    /// Felt mallet - soft impact with polynomial compression
    FeltMallet,
    /// Hard mallet - rigid impact with Hertzian contact
    HardMallet,
    /// Drumstick - wood tip with micro-bouncing
    Drumstick,
    /// Wire brush - stochastic impulse cluster
    WireBrush,
    /// Metal pipe - bi-directional metal-on-metal
    MetalPipe,
    /// Metal chain - cascading multi-mass impacts
    MetalChain,
    /// Stiff point - nail/awl chatter scrape
    StiffPoint,
    /// Heavy grinding - concrete/sandpaper friction
    HeavyGrinding,
    /// Corrugated drag - stick on ribbed surface
    CorrugatedDrag,
    /// Tension rise - creak/groan mechanics
    TensionRise,
    /// Pneumatic jet - steam/air valve
    PneumaticJet,
    /// Electromagnetic hum - transformer/motor
    ElectromagneticHum,
    /// Tension snap - wire break/gear catch
    TensionSnap,
    /// Particle rain - debris/sand/gravel stream
    ParticleRain,
}

/// Exciter family categorization for envelope selection.
///
/// Each family uses a different envelope type:
/// - **Hit**: One-shot AR envelope
/// - **Scrape**: MSEG with looping sustain
/// - **Specialty**: ADSR envelope
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExciterFamily {
    /// Impact family - uses one-shot AR envelope
    Hit,
    /// Friction family - uses MSEG with looping
    Scrape,
    /// Non-traditional family - uses ADSR envelope
    Specialty,
}

impl ExciterType {
    /// Converts integer parameter value to ExciterType enum.
    ///
    /// # Arguments
    /// * `v` - Integer value (0-16)
    ///
    /// # Returns
    /// Corresponding ExciterType variant, defaulting to Hit for invalid values
    pub fn from_int(v: i32) -> Self {
        match v {
            0 => ExciterType::Hit,
            1 => ExciterType::Scrape,
            2 => ExciterType::HandStrike,
            3 => ExciterType::FeltMallet,
            4 => ExciterType::HardMallet,
            5 => ExciterType::Drumstick,
            6 => ExciterType::WireBrush,
            7 => ExciterType::MetalPipe,
            8 => ExciterType::MetalChain,
            9 => ExciterType::StiffPoint,
            10 => ExciterType::HeavyGrinding,
            11 => ExciterType::CorrugatedDrag,
            12 => ExciterType::TensionRise,
            13 => ExciterType::PneumaticJet,
            14 => ExciterType::ElectromagneticHum,
            15 => ExciterType::TensionSnap,
            16 => ExciterType::ParticleRain,
            _ => ExciterType::Hit,
        }
    }

    /// Returns human-readable name for UI display.
    ///
    /// # Returns
    /// Static string slice with display name
    pub fn name(self) -> &'static str {
        match self {
            ExciterType::Hit => "Hit",
            ExciterType::Scrape => "Scrape",
            ExciterType::HandStrike => "Hand Strike",
            ExciterType::FeltMallet => "Felt Mallet",
            ExciterType::HardMallet => "Hard Mallet",
            ExciterType::Drumstick => "Drumstick",
            ExciterType::WireBrush => "Wire Brush",
            ExciterType::MetalPipe => "Metal Pipe",
            ExciterType::MetalChain => "Metal Chain",
            ExciterType::StiffPoint => "Stiff Point",
            ExciterType::HeavyGrinding => "Heavy Grinding",
            ExciterType::CorrugatedDrag => "Corrugated Drag",
            ExciterType::TensionRise => "Tension Rise",
            ExciterType::PneumaticJet => "Pneumatic Jet",
            ExciterType::ElectromagneticHum => "Electromagnetic Hum",
            ExciterType::TensionSnap => "Tension Snap",
            ExciterType::ParticleRain => "Particle Rain",
        }
    }

    /// Returns the family category for envelope selection.
    ///
    /// # Returns
    /// ExciterFamily enum determining which envelope type to use
    pub fn family(self) -> ExciterFamily {
        match self {
            ExciterType::Hit
            | ExciterType::HandStrike
            | ExciterType::FeltMallet
            | ExciterType::HardMallet
            | ExciterType::Drumstick
            | ExciterType::WireBrush
            | ExciterType::MetalPipe
            | ExciterType::MetalChain => ExciterFamily::Hit,
            ExciterType::Scrape
            | ExciterType::StiffPoint
            | ExciterType::HeavyGrinding
            | ExciterType::CorrugatedDrag
            | ExciterType::TensionRise => ExciterFamily::Scrape,
            ExciterType::PneumaticJet
            | ExciterType::ElectromagneticHum
            | ExciterType::TensionSnap
            | ExciterType::ParticleRain => ExciterFamily::Specialty,
        }
    }

    /// Converts ExciterType to integer for parameter storage.
    ///
    /// # Returns
    /// Integer value (0-16) for serialization
    pub fn to_int(self) -> i32 {
        match self {
            ExciterType::Hit => 0,
            ExciterType::Scrape => 1,
            ExciterType::HandStrike => 2,
            ExciterType::FeltMallet => 3,
            ExciterType::HardMallet => 4,
            ExciterType::Drumstick => 5,
            ExciterType::WireBrush => 6,
            ExciterType::MetalPipe => 7,
            ExciterType::MetalChain => 8,
            ExciterType::StiffPoint => 9,
            ExciterType::HeavyGrinding => 10,
            ExciterType::CorrugatedDrag => 11,
            ExciterType::TensionRise => 12,
            ExciterType::PneumaticJet => 13,
            ExciterType::ElectromagneticHum => 14,
            ExciterType::TensionSnap => 15,
            ExciterType::ParticleRain => 16,
        }
    }
}

/// Creates an IntParam for exciter selection with name display and parsing.
///
/// # Arguments
/// * `default` - Default exciter type index (0-16)
///
/// # Returns
/// Configured IntParam with value-to-string and string-to-value conversions
pub fn exciter_param(default: i32) -> IntParam {
    IntParam::new("Exciter", default, IntRange::Linear { min: 0, max: 16 })
        .with_value_to_string(Arc::new(|value| {
            ExciterType::from_int(value).name().to_string()
        }))
        .with_string_to_value(Arc::new(|string| {
            let normalized = string.trim();
            [
                ExciterType::Hit,
                ExciterType::Scrape,
                ExciterType::HandStrike,
                ExciterType::FeltMallet,
                ExciterType::HardMallet,
                ExciterType::Drumstick,
                ExciterType::WireBrush,
                ExciterType::MetalPipe,
                ExciterType::MetalChain,
                ExciterType::StiffPoint,
                ExciterType::HeavyGrinding,
                ExciterType::CorrugatedDrag,
                ExciterType::TensionRise,
                ExciterType::PneumaticJet,
                ExciterType::ElectromagneticHum,
                ExciterType::TensionSnap,
                ExciterType::ParticleRain,
            ]
            .into_iter()
            .find(|exciter| exciter.name().eq_ignore_ascii_case(normalized))
            .map(ExciterType::to_int)
        }))
}

/// Resonator object type enumeration.
///
/// Each object has distinct modal characteristics based on physical geometry:
/// - **Pipe**: Cylindrical resonator with longitudinal modes
/// - **Plate**: 2D surface with complex modal density
/// - **Tank**: Volumetric resonator with helmholtz modes
/// - **Chain**: 1D linked segments
/// - **IBeam**: Structural beam with bending/torsional modes
/// - **TautCable**: String-like with harmonic overtones
/// - **CoilSpring**: Helical spring with non-harmonic modes
/// - **SheetMetal**: Thin plate with high modal density
/// - **IndustrialCog**: Rotating object with cyclic modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Object {
    /// Cylindrical pipe resonator
    Pipe,
    /// Rectangular plate resonator
    Plate,
    /// Enclosed tank/volume
    Tank,
    /// Hanging chain segments
    Chain,
    /// Structural I-beam
    IBeam,
    /// Tensioned cable/string
    TautCable,
    /// Helical coil spring
    CoilSpring,
    /// Thin sheet metal panel
    SheetMetal,
    /// Industrial gear/cog
    IndustrialCog,
}

impl Object {
    /// Converts integer parameter value to Object enum.
    ///
    /// # Arguments
    /// * `v` - Integer value (0-8)
    ///
    /// # Returns
    /// Corresponding Object variant, defaulting to Pipe for invalid values
    pub fn from_int(v: i32) -> Self {
        match v {
            0 => Object::Pipe,
            1 => Object::Plate,
            2 => Object::Tank,
            3 => Object::Chain,
            4 => Object::IBeam,
            5 => Object::TautCable,
            6 => Object::CoilSpring,
            7 => Object::SheetMetal,
            8 => Object::IndustrialCog,
            _ => Object::Pipe,
        }
    }

    /// Returns human-readable name for UI display.
    ///
    /// # Returns
    /// Static string slice with display name
    pub fn name(self) -> &'static str {
        match self {
            Object::Pipe => "Pipe",
            Object::Plate => "Plate",
            Object::Tank => "Tank",
            Object::Chain => "Chain",
            Object::IBeam => "I-Beam",
            Object::TautCable => "Taut Cable",
            Object::CoilSpring => "Coil Spring",
            Object::SheetMetal => "Sheet Metal",
            Object::IndustrialCog => "Industrial Cog",
        }
    }

    /// Converts Object to integer for parameter storage.
    ///
    /// # Returns
    /// Integer value (0-8) for serialization
    pub fn to_int(self) -> i32 {
        match self {
            Object::Pipe => 0,
            Object::Plate => 1,
            Object::Tank => 2,
            Object::Chain => 3,
            Object::IBeam => 4,
            Object::TautCable => 5,
            Object::CoilSpring => 6,
            Object::SheetMetal => 7,
            Object::IndustrialCog => 8,
        }
    }
}

/// Creates an IntParam for object selection with name display and parsing.
///
/// # Arguments
/// * `default` - Default object index (0-8)
///
/// # Returns
/// Configured IntParam with value-to-string and string-to-value conversions
pub fn object_param(default: i32) -> IntParam {
    IntParam::new("Object", default, IntRange::Linear { min: 0, max: 8 })
        .with_value_to_string(Arc::new(|value| Object::from_int(value).name().to_string()))
        .with_string_to_value(Arc::new(|string| {
            let normalized = string.trim();
            [
                Object::Pipe,
                Object::Plate,
                Object::Tank,
                Object::Chain,
                Object::IBeam,
                Object::TautCable,
                Object::CoilSpring,
                Object::SheetMetal,
                Object::IndustrialCog,
            ]
            .into_iter()
            .find(|object| object.name().eq_ignore_ascii_case(normalized))
            .map(Object::to_int)
        }))
}

impl Default for CorrosionParams {
    /// Creates default parameter values for plugin initialization.
    ///
    /// Sets sensible defaults for all 70+ parameters to ensure
    /// the plugin produces audible output immediately upon loading.
    fn default() -> Self {
        Self {
            #[cfg(feature = "gui")]
            editor_state: EguiState::from_size(1440, 1024),
            
            // Sound generation defaults
            exciter: exciter_param(0), // Hit
            object: object_param(0),   // Pipe
            
            // Object transformation defaults
            size: FloatParam::new(
                "Size",
                1.0,
                FloatRange::Linear {
                    min: 0.05,
                    max: 10.0,
                },
            ),
            rust: FloatParam::new("Rust", 0.0, FloatRange::Linear { min: 0.0, max: 5.0 }),
            damage: FloatParam::new(
                "Damage",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10.0,
                },
            ),
            
            // Master controls
            drive: FloatParam::new("Drive", 0.0, FloatRange::Linear { min: 0.0, max: 5.0 }),
            output: FloatParam::new(
                "Output",
                util::db_to_gain(0.0),
                FloatRange::Linear {
                    min: 0.0,
                    max: util::db_to_gain(40.0),
                },
            ),
            width: FloatParam::new(
                "Width",
                0.5,
                FloatRange::Linear {
                    min: -2.0,
                    max: 3.0,
                },
            ),
            body: FloatParam::new("Body", 0.0, FloatRange::Linear { min: 0.0, max: 5.0 }),
            
            // UI scale (percentage display)
            ui_scale: IntParam::new("UI Scale", 2, IntRange::Linear { min: 0, max: 4 })
                .with_value_to_string(Arc::new(|value| match value {
                    0 => "50%".to_string(),
                    1 => "75%".to_string(),
                    2 => "100%".to_string(),
                    3 => "125%".to_string(),
                    4 => "150%".to_string(),
                    _ => "100%".to_string(),
                })),
            
            // ADSR envelope defaults
            env_attack: FloatParam::new(
                "Attack",
                0.05,
                FloatRange::Linear {
                    min: 0.001,
                    max: 2.0,
                },
            ),
            env_decay: FloatParam::new(
                "Decay",
                0.3,
                FloatRange::Linear {
                    min: 0.01,
                    max: 5.0,
                },
            ),
            env_sustain: FloatParam::new("Sustain", 0.7, FloatRange::Linear { min: 0.0, max: 1.0 }),
            env_release: FloatParam::new(
                "Release",
                0.5,
                FloatRange::Linear {
                    min: 0.01,
                    max: 5.0,
                },
            ),
            
            // MSEG envelope defaults
            mseg_onset: FloatParam::new(
                "Onset",
                0.01,
                FloatRange::Linear {
                    min: 0.001,
                    max: 1.0,
                },
            ),
            mseg_attack: FloatParam::new(
                "MSEG Attack",
                0.05,
                FloatRange::Linear {
                    min: 0.001,
                    max: 2.0,
                },
            ),
            mseg_hold: FloatParam::new("Hold", 0.02, FloatRange::Linear { min: 0.0, max: 2.0 }),
            mseg_decay: FloatParam::new(
                "MSEG Decay",
                0.3,
                FloatRange::Linear {
                    min: 0.01,
                    max: 5.0,
                },
            ),
            mseg_sustain: FloatParam::new(
                "MSEG Sustain",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            mseg_release: FloatParam::new(
                "MSEG Release",
                0.1,
                FloatRange::Linear {
                    min: 0.01,
                    max: 5.0,
                },
            ),
            
            // Envelope modulation defaults
            env_amount: FloatParam::new(
                "Env Amount",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            velocity_to_peak: FloatParam::new(
                "Velocity To Peak",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // MSEG loop defaults
            loop_mode: IntParam::new("Loop Mode", 0, IntRange::Linear { min: 0, max: 2 }),
            loop_start_stage: IntParam::new(
                "Loop Start Stage",
                3,
                IntRange::Linear { min: 0, max: 5 },
            ),
            loop_end_stage: IntParam::new("Loop End Stage", 4, IntRange::Linear { min: 0, max: 5 }),
            
            // Timing and sync
            sync_rate: FloatParam::new(
                "Sync Rate",
                0.25,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            global_time_scale: FloatParam::new(
                "Global Time Scale",
                1.0,
                FloatRange::Linear {
                    min: 0.1,
                    max: 10.0,
                },
            ),
            velocity_to_level: FloatParam::new(
                "Velocity To Level",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            velocity_to_time: FloatParam::new(
                "Velocity To Time",
                0.15,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            curve_tension: FloatParam::new(
                "Curve Tension",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            ),
            
            // Exciter control defaults
            exciter_pressure: FloatParam::new(
                "Pressure",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            exciter_speed: FloatParam::new("Speed", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            exciter_roughness: FloatParam::new(
                "Roughness",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Interaction defaults
            strike_position: FloatParam::new(
                "Strike Position",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            coupling_stiffness: FloatParam::new(
                "Coupling Stiffness",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            position_wander: FloatParam::new(
                "Position Wander",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            position_envelope: FloatParam::new(
                "Position Envelope",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            fundamental_anchor: FloatParam::new(
                "Fundamental Anchor",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Resonator control defaults
            res_damping: FloatParam::new("Damping", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            res_brightness: FloatParam::new(
                "Brightness",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            thickness: FloatParam::new("Thickness", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            heat: FloatParam::new("Heat", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            sludge: FloatParam::new("Sludge", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            
            // Post-processing filter defaults
            filter_cutoff: FloatParam::new(
                "Filter Cutoff",
                20000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20000.0,
                    factor: 0.5,
                },
            ),
            filter_resonance: FloatParam::new(
                "Filter Resonance",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            component_tolerance: FloatParam::new(
                "Component Tolerance",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Drive defaults
            drive_amount: FloatParam::new(
                "Drive Amount",
                0.0,
                FloatRange::Linear { min: 0.0, max: 5.0 },
            ),
            bias_starvation: FloatParam::new(
                "Bias Starvation",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            chaos_depth: FloatParam::new(
                "Chaos Depth",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Stereo/spatial defaults
            spread_width: FloatParam::new(
                "Spread Width",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            listener_proximity: FloatParam::new(
                "Listener Proximity",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Body resonance defaults
            chassis_material: FloatParam::new(
                "Chassis Material",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            chassis_volume: FloatParam::new(
                "Chassis Volume",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Space/reverb defaults
            space_mode: IntParam::new("Space Mode", 0, IntRange::Linear { min: 0, max: 3 }),
            space_amount: FloatParam::new(
                "Space Amount",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            factory_size: FloatParam::new(
                "Factory Size",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            machinery_clutter: FloatParam::new(
                "Machinery Clutter",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            wall_impedance: FloatParam::new(
                "Wall Impedance",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            spring_tension: FloatParam::new(
                "Spring Tension",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            wire_stiffness: FloatParam::new(
                "Wire Stiffness",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            spring_tank_size: FloatParam::new(
                "Spring Tank Size",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            delay_time: FloatParam::new(
                "Delay Time",
                0.25,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            machinery_movement: FloatParam::new(
                "Machinery Movement",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            high_frequency_damping: FloatParam::new(
                "High Frequency Damping",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
            
            // Output stage defaults
            analog_ceiling: FloatParam::new(
                "Analog Ceiling",
                0.9661,
                FloatRange::Linear { min: 0.5, max: 1.0 },
            ),
            diode_softness: FloatParam::new(
                "Diode Softness",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
        }
    }
}
