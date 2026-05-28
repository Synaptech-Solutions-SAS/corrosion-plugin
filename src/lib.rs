//! # Corrosion - Industrial Physical Modeling Synthesizer
//!
//! This is the root module of the Corrosion VST plugin, an industrial physical modeling
//! synthesizer built using the nih_plug framework. The plugin implements a modal synthesis
//! engine with 17 different exciter types, 9 resonator objects, and comprehensive
//! post-processing effects.
//!
//! ## Architecture Overview
//!
//! The plugin follows a modular architecture with clear separation of concerns:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                        lib.rs (This File)                   │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │   params    │  │    voice    │  │         dsp         │  │
//! │  │  (Params)   │  │  (Voices)   │  │ (Exciters/Resonator)│  │
//! │  └─────────────┘  └─────────────┘  └─────────────────────┘  │
//! │         │                │                    │              │
//! │         ▼                ▼                    ▼              │
//! │  ┌────────────────────────────────────────────────────────┐ │
//! │  │              PostProcessingChain                       │ │
//! │  │  (Filter → Drive → Body → Spread → Space → Clipper)  │ │
//! │  └────────────────────────────────────────────────────────┘ │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Relationships
//!
//! - **`dsp`** - Digital Signal Processing core: exciters, resonators, transforms, effects
//! - **`voice`** - Voice management: polyphony, voice stealing, per-note state
//! - **`params`** - Parameter definitions and host communication
//! - **`gui`** - Optional egui-based editor (feature-gated)
//! - **`presets`** - Preset serialization/deserialization
//! - **`offline`** - Offline rendering and batch processing utilities
//! - **`randomizer`** - Parameter randomization utilities
//!
//! ## Audio Signal Flow
//!
//! 1. **MIDI Input** → Note events trigger voice allocation in `VoiceManager`
//! 2. **Excitation** → Selected exciter model generates force
//! 3. **Resonator** → Modal resonator filters excitation into pitched sound
//! 4. **Voice Mixing** → Active voices are summed with polyphony limit
//! 5. **Post-Processing** → Global effects chain processes mixed output
//! 6. **Output** → Final stereo pair with limiting
//!
//! ## Real-Time Constraints
//!
//! This plugin is designed for real-time audio processing:
//! - No heap allocations in the audio callback (`process()`)
//! - Lock-free voice management
//! - Sample-accurate MIDI event handling
//! - Denormal number protection

/// Digital Signal Processing module - contains all audio algorithms
pub mod dsp;

/// GUI module - optional egui-based editor interface
pub mod gui;

/// Offline rendering module - batch processing and export utilities
pub mod offline;

/// Parameter definitions - host-facing parameter interface
mod params;

/// Preset management - serialization and preset bank handling
pub mod presets;

/// Parameter randomization - creative variation generation
pub mod randomizer;

/// Voice management - polyphony, voice allocation, per-note processing
pub mod voice;

// External dependencies
use std::num::NonZeroU32;
use std::sync::Arc;

// nih_plug framework for VST/CLAP plugin hosting
use nih_plug::prelude::*;

// Internal imports
use voice::{VoiceControls, VoiceManager};

// Public re-exports for convenience
pub use params::{CorrosionParams, Object};
pub use presets::Preset;

/// Output limiter threshold to prevent clipping and ensure headroom.
///
/// Set slightly below 1.0 (0.9661) to allow for inter-sample peaks
/// and provide safety margin for downstream processing.
pub const LIMITER_THRESHOLD: f32 = 0.9661;

/// Applies a hard limiter to a single sample to prevent output clipping.
///
/// This function clamps the sample to the range [-LIMITER_THRESHOLD, LIMITER_THRESHOLD].
/// It's applied as the final stage of the signal chain before output.
///
/// # Arguments
/// * `sample` - Input sample value
///
/// # Returns
/// Limited sample value within safe output range
///
/// # Example
/// ```rust
/// let limited = corrosion::apply_output_limiter(1.5);
/// assert_eq!(limited, corrosion::LIMITER_THRESHOLD);
/// ```
#[inline]
pub fn apply_output_limiter(sample: f32) -> f32 {
    sample.clamp(-LIMITER_THRESHOLD, LIMITER_THRESHOLD)
}

/// Applies asymmetric drive/saturation to a sample.
///
/// This implements a multi-stage waveshaper with:
/// - Linear pass-through for low amplitudes
/// - Smooth knee transition using cubic Hermite interpolation
/// - Asymmetric saturation (different curves for positive/negative)
/// - Dry/wet mixing
///
/// The asymmetry creates harmonic complexity typical of industrial textures.
///
/// # Arguments
/// * `sample` - Input sample value
/// * `drive` - Drive amount (0.0 = clean, higher = more saturation)
///
/// # Returns
/// Processed sample with applied drive and saturation
#[inline]
pub fn apply_drive(sample: f32, drive: f32) -> f32 {
    // Early exit if drive is disabled for efficiency
    if drive <= 0.0 {
        return sample;
    }

    // Calculate drive gain - exponential scaling for perceptual uniformity
    let drive_gain = 1.0 + drive * 4.0;
    let amplified = sample * drive_gain;

    // Extract magnitude and sign for asymmetric processing
    let abs_sample = amplified.abs();
    let sign = amplified.signum();

    // Apply asymmetric waveshaping based on signal polarity
    let shaped = if sign > 0.0 {
        // Positive half: gentler curve, more "tube-like"
        let soft_threshold = 0.3;
        let hard_threshold = 0.8;

        if abs_sample < soft_threshold {
            // Linear region - preserve transients
            amplified
        } else if abs_sample < hard_threshold {
            // Soft knee using smoothstep (3t² - 2t³) for musical saturation
            let t = (abs_sample - soft_threshold) / (hard_threshold - soft_threshold);
            let eased =
                soft_threshold + (hard_threshold - soft_threshold) * (t * t * (3.0 - 2.0 * t));
            sign * eased
        } else {
            // Hard limiting with exponential decay tail
            let t = (abs_sample - hard_threshold) / (1.0 + drive_gain);
            let compressed = hard_threshold + (1.0 - hard_threshold) * (1.0 - (-t).exp());
            sign * compressed.min(1.2)
        }
    } else {
        // Negative half: sharper curve, more aggressive limiting
        let soft_threshold = 0.25;
        let hard_threshold = 0.7;

        if abs_sample < soft_threshold {
            // Linear region
            amplified
        } else if abs_sample < hard_threshold {
            // Soft knee
            let t = (abs_sample - soft_threshold) / (hard_threshold - soft_threshold);
            let eased =
                soft_threshold + (hard_threshold - soft_threshold) * (t * t * (3.0 - 2.0 * t));
            sign * eased
        } else {
            // Hard limiting with tighter ceiling for negative excursions
            let t = (abs_sample - hard_threshold) / (1.0 + drive_gain);
            let compressed = hard_threshold + (1.0 - hard_threshold) * (1.0 - (-t).exp());
            sign * compressed.min(1.1)
        }
    };

    // Dry/wet mix: more drive = more wet signal
    let dry_wet = shaped * drive + sample * (1.0 - drive * 0.5);
    dry_wet.clamp(-1.5, 1.5)
}

/// Handles incoming MIDI note events and routes them to voice management.
///
/// This function is called from the audio thread when MIDI events are received.
/// It:
/// 1. Maps Object parameter to ModalProfileId for resonator selection
/// 2. Collects all current parameter values into VoiceControls
/// 3. Triggers note_on or note_off in the voice manager
///
/// # Arguments
/// * `plugin` - Mutable reference to the plugin instance
/// * `event` - The MIDI note event to process
///
/// # Real-Time Safety
/// This function is called from the audio callback and must not allocate
/// or perform blocking operations.
fn handle_note_event(plugin: &mut Corrosion, event: NoteEvent<()>) {
    match event {
        NoteEvent::NoteOn { note, velocity, .. } => {
            // Map the Object parameter to a modal profile for the resonator
            // Each object has distinct modal characteristics (pipe vs plate vs tank, etc.)
            let profile = match Object::from_int(plugin.params.object.value()) {
                Object::Pipe => dsp::ModalProfileId::Pipe,
                Object::Plate => dsp::ModalProfileId::Plate,
                Object::Tank => dsp::ModalProfileId::Tank,
                Object::Chain => dsp::ModalProfileId::Chain,
                Object::IBeam => dsp::ModalProfileId::IBeam,
                Object::TautCable => dsp::ModalProfileId::TautCable,
                Object::CoilSpring => dsp::ModalProfileId::CoilSpring,
                Object::SheetMetal => dsp::ModalProfileId::SheetMetal,
                Object::IndustrialCog => dsp::ModalProfileId::IndustrialCog,
            };

            // Collect all current parameter values into VoiceControls struct
            // This bundles parameters that need to be passed to the voice
            let controls = VoiceControls {
                env_attack: plugin.params.env_attack.value(),
                env_decay: plugin.params.env_decay.value(),
                env_sustain: plugin.params.env_sustain.value(),
                env_release: plugin.params.env_release.value(),
                mseg_onset: plugin.params.mseg_onset.value(),
                mseg_attack: plugin.params.mseg_attack.value(),
                mseg_hold: plugin.params.mseg_hold.value(),
                mseg_decay: plugin.params.mseg_decay.value(),
                mseg_sustain: plugin.params.mseg_sustain.value(),
                mseg_release: plugin.params.mseg_release.value(),
                env_amount: plugin.params.env_amount.value(),
                velocity_to_peak: plugin.params.velocity_to_peak.value(),
                loop_mode: plugin.params.loop_mode.value(),
                loop_start_stage: plugin.params.loop_start_stage.value(),
                loop_end_stage: plugin.params.loop_end_stage.value(),
                global_time_scale: plugin.params.global_time_scale.value(),
                velocity_to_level: plugin.params.velocity_to_level.value(),
                velocity_to_time: plugin.params.velocity_to_time.value(),
                curve_tension: plugin.params.curve_tension.value(),
                exciter_pressure: plugin.params.exciter_pressure.value(),
                exciter_speed: plugin.params.exciter_speed.value(),
                exciter_roughness: plugin.params.exciter_roughness.value(),
                hand_mass: plugin.params.hand_mass.value(),
                flesh_stiffness: plugin.params.flesh_stiffness.value(),
                flesh_damping: plugin.params.flesh_damping.value(),
                mute_decay: plugin.params.mute_decay.value(),
                mallet_mass: plugin.params.mallet_mass.value(),
                felt_softness: plugin.params.felt_softness.value(),
                core_hardness: plugin.params.core_hardness.value(),
                compression_curve: plugin.params.compression_curve.value(),
                material_stiffness: plugin.params.material_stiffness.value(),
                impact_damping: plugin.params.impact_damping.value(),
                stick_mass: plugin.params.stick_mass.value(),
                tip_stiffness: plugin.params.tip_stiffness.value(),
                restitution_bounciness: plugin.params.restitution_bounciness.value(),
                micro_bounce_limit: plugin.params.micro_bounce_limit.value(),
                wire_density: plugin.params.wire_density.value(),
                spread_duration: plugin.params.spread_duration.value(),
                brush_wire_stiffness: plugin.params.brush_wire_stiffness.value(),
                amplitude_randomization: plugin.params.amplitude_randomization.value(),
                pipe_mass: plugin.params.pipe_mass.value(),
                metal_stiffness: plugin.params.metal_stiffness.value(),
                pipe_pitch: plugin.params.pipe_pitch.value(),
                pipe_ring_decay: plugin.params.pipe_ring_decay.value(),
                link_count: plugin.params.link_count.value(),
                chain_mass: plugin.params.chain_mass.value(),
                drop_envelope_spread: plugin.params.drop_envelope_spread.value(),
                internal_rattle: plugin.params.internal_rattle.value(),
                rattle_color: plugin.params.rattle_color.value(),
                bow_pressure: plugin.params.bow_pressure.value(),
                bow_speed: plugin.params.bow_speed.value(),
                rosin_grip: plugin.params.rosin_grip.value(),
                slip_curve: plugin.params.slip_curve.value(),
                scrape_speed: plugin.params.scrape_speed.value(),
                point_pressure: plugin.params.point_pressure.value(),
                chatter_pitch: plugin.params.chatter_pitch.value(),
                chatter_damping: plugin.params.chatter_damping.value(),
                grind_speed: plugin.params.grind_speed.value(),
                grind_pressure: plugin.params.grind_pressure.value(),
                surface_grit: plugin.params.surface_grit.value(),
                grit_color: plugin.params.grit_color.value(),
                drag_speed: plugin.params.drag_speed.value(),
                ridge_spacing: plugin.params.ridge_spacing.value(),
                ridge_depth: plugin.params.ridge_depth.value(),
                drag_exciter_mass: plugin.params.drag_exciter_mass.value(),
                pull_speed: plugin.params.pull_speed.value(),
                break_threshold: plugin.params.break_threshold.value(),
                slip_stochasticity: plugin.params.slip_stochasticity.value(),
                creak_sharpness: plugin.params.creak_sharpness.value(),
                air_pressure: plugin.params.air_pressure.value(),
                nozzle_width: plugin.params.nozzle_width.value(),
                turbulence_chaos: plugin.params.turbulence_chaos.value(),
                mains_frequency: plugin.params.mains_frequency.value(),
                coil_proximity: plugin.params.coil_proximity.value(),
                voltage_sag: plugin.params.voltage_sag.value(),
                pull_distance: plugin.params.pull_distance.value(),
                hook_stiffness: plugin.params.hook_stiffness.value(),
                snap_force: plugin.params.snap_force.value(),
                flow_rate: plugin.params.flow_rate.value(),
                particle_mass: plugin.params.particle_mass.value(),
                mass_variance: plugin.params.mass_variance.value(),
                strike_position: plugin.params.strike_position.value(),
                coupling_stiffness: plugin.params.coupling_stiffness.value(),
                position_wander: plugin.params.position_wander.value(),
                position_envelope: plugin.params.position_envelope.value(),
                fundamental_anchor: plugin.params.fundamental_anchor.value(),
                res_damping: plugin.params.res_damping.value(),
                res_brightness: plugin.params.res_brightness.value(),
                thickness: plugin.params.thickness.value(),
                heat: plugin.params.heat.value(),
                sludge: plugin.params.sludge.value(),
                character: dsp::CharacterParams {
                    pipe_diameter: plugin.params.pipe_diameter.value(),
                    plate_aspect: plugin.params.plate_aspect.value(),
                    plate_stiffness: plugin.params.plate_stiffness.value(),
                    tank_volume: plugin.params.tank_volume.value(),
                    tank_cavity_mix: plugin.params.tank_cavity_mix.value(),
                    chain_link_mass: plugin.params.chain_link_mass.value(),
                    chain_instability: plugin.params.chain_instability.value(),
                    beam_shear: plugin.params.beam_shear.value(),
                    cable_braid: plugin.params.cable_braid.value(),
                    cable_tension_drop: plugin.params.cable_tension_drop.value(),
                    spring_dispersion: plugin.params.spring_dispersion.value(),
                    spring_slosh: plugin.params.spring_slosh.value(),
                    sheet_thinness: plugin.params.sheet_thinness.value(),
                    cog_dissonance: plugin.params.cog_dissonance.value(),
                },
            };

            // Get the selected exciter type (0-16 for different exciters)
            let exciter_type = plugin.params.exciter.value();

            // Trigger note-on in voice manager with all parameters
            // The voice manager handles polyphony and voice allocation
            plugin.voice_manager.note_on_with_controls(
                note,
                note_event_velocity_to_voice_velocity(velocity),
                profile,
                plugin.params.size.value(),
                plugin.params.rust.value(),
                plugin.params.damage.value(),
                exciter_type,
                controls,
            );
        }
        NoteEvent::NoteOff { note, .. } => {
            // Release the voice playing this note
            plugin.voice_manager.note_off(note);
        }
        _ => {}
    }
}

/// Convert NIH-plug's normalized note velocity into the MIDI-scale velocity
/// expected by the current voice layer.
///
/// `NoteEvent::NoteOn::velocity` is documented by NIH-plug as `[0, 1]`, while
/// `Voice::note_on()` and the existing voice/renderer tests use the traditional
/// MIDI `0..127` scale. Keeping the conversion at the host boundary prevents
/// hosted notes from being attenuated twice before they reach the exciters.
#[inline]
fn note_event_velocity_to_voice_velocity(velocity: f32) -> f32 {
    velocity.clamp(0.0, 1.0) * 127.0
}

/// Processes pending MIDI events that are scheduled for the current sample.
///
/// This function implements sample-accurate MIDI timing by checking if events
/// should be processed at the current sample position. Events with timing
/// less than or equal to the current sample_id are processed immediately.
///
/// # Arguments
/// * `plugin` - Mutable reference to the plugin instance
/// * `sample_id` - Current sample index within the buffer
/// * `next_event` - Pointer to the next pending event (updated as events are consumed)
/// * `fetch_next` - Closure that retrieves the next event from the host
///
/// # Type Parameters
/// * `F` - Closure type for fetching next event
fn process_pending_events<F>(
    plugin: &mut Corrosion,
    sample_id: u32,
    next_event: &mut Option<NoteEvent<()>>,
    mut fetch_next: F,
) where
    F: FnMut() -> Option<NoteEvent<()>>,
{
    // Process all events scheduled for or before the current sample
    while let Some(event) = *next_event {
        if event.timing() > sample_id {
            // Event is in the future, stop processing
            break;
        }

        // Handle the current event
        handle_note_event(plugin, event);

        // Fetch the next event from the host
        *next_event = fetch_next();
    }
}

/// Returns the plugin version from Cargo.toml at compile time.
///
/// # Returns
/// Plugin version string (e.g., "1.0.0")
pub fn corrosion_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Main plugin struct containing all state.
///
/// This struct implements the nih_plug Plugin trait and contains:
/// - Parameter storage (Arc<CorrosionParams>)
/// - Voice manager for polyphony
/// - Post-processing effects chain
///
/// # Thread Safety
/// The plugin is designed for single-threaded audio callback usage.
/// Parameters are shared via Arc but are read-only during processing.
pub struct Corrosion {
    /// Shared parameter storage - Arc allows sharing with GUI thread
    params: Arc<CorrosionParams>,

    /// Voice manager handles polyphony, voice allocation, and voice stealing
    voice_manager: VoiceManager,

    /// Post-processing chain applies global effects to mixed output
    post_chain: dsp::PostProcessingChain,
}

impl Default for Corrosion {
    /// Creates a default plugin instance with initialized subsystems.
    fn default() -> Self {
        let params = Arc::new(CorrosionParams::default());

        Self {
            params,
            voice_manager: VoiceManager::new(),
            post_chain: dsp::PostProcessingChain::new(),
        }
    }
}

impl Corrosion {
    /// Serializes current parameter state to bytes for host state save.
    ///
    /// # Returns
    /// JSON-encoded preset bytes, or empty vector on serialization failure
    pub fn get_state(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(&Preset::from_params(Self::NAME, &self.params))
            .unwrap_or_default()
    }

    /// Restores parameter state from serialized bytes.
    ///
    /// # Arguments
    /// * `state` - Serialized preset bytes from previous get_state() call
    pub fn load_state(&mut self, state: &[u8]) {
        if let Ok(preset) = serde_json::from_slice::<Preset>(state) {
            self.params = Arc::new(preset.into_params());
        }
    }
}

impl Plugin for Corrosion {
    // Plugin metadata constants
    const NAME: &'static str = "Corrosion";
    const VENDOR: &'static str = "Corrosion Audio";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Audio I/O configuration - stereo output, no input
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,                // No audio input (instrument)
        main_output_channels: NonZeroU32::new(2), // Stereo output
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    /// MIDI input configuration - basic MIDI support for notes
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    /// No SysEx message support
    type SysExMessage = ();

    /// No background tasks
    type BackgroundTask = ();

    /// Returns parameter interface for host automation
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    /// Called once when plugin is instantiated.
    ///
    /// # Returns
    /// true if initialization succeeded
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    /// Resets all plugin state (called when playback starts/stops).
    ///
    /// Reinitializes voice manager and post-processing chain to clean state.
    fn reset(&mut self) {
        self.voice_manager = VoiceManager::new();
        self.post_chain = dsp::PostProcessingChain::new();
    }

    /// Main audio processing callback - called for each buffer.
    ///
    /// This is the core real-time processing loop. It:
    /// 1. Processes MIDI events with sample-accurate timing
    /// 2. Generates audio from all active voices
    /// 3. Applies post-processing effects
    /// 4. Outputs final stereo signal
    ///
    /// # Real-Time Constraints
    /// - No heap allocations
    /// - No blocking operations
    /// - Must complete within buffer duration
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Get first MIDI event and sample rate from host
        let mut next_event = context.next_event();
        let sample_rate = context.transport().sample_rate as u32;

        // Read parameter values and configure the post-processing chain once per
        // buffer. These setters recompute coefficients (filter tan(), FEM freqs,
        // reverb delays) that must not run at audio rate, and the per-sample
        // version compounded the FactoryReverb comb delays. Host automation is
        // applied at buffer granularity, so per-buffer updates are sufficient.
        let drive = self.params.drive.value();
        let output_gain = self.params.output.value();
        let width = self.params.width.value();
        let body_amount = (self.params.body.value() / 5.0).clamp(0.0, 1.0);

        self.post_chain.set_sample_rate(sample_rate as f32);
        self.post_chain.set_filter_params(
            self.params.filter_cutoff.value(),
            self.params.filter_resonance.value(),
            self.params.component_tolerance.value(),
        );
        self.post_chain.set_drive_params(
            self.params.drive_amount.value(),
            self.params.bias_starvation.value(),
            self.params.chaos_depth.value(),
        );
        self.post_chain.set_body_params(
            self.params.chassis_material.value(),
            self.params.chassis_volume.value().max(body_amount),
        );
        self.post_chain.set_spread_params(
            self.params.spread_width.value(),
            self.params.listener_proximity.value(),
        );

        // Map space mode integer to enum
        let space_mode = match self.params.space_mode.value() {
            0 => dsp::SpaceMode::Off,
            1 => dsp::SpaceMode::Factory,
            2 => dsp::SpaceMode::Spring,
            3 => dsp::SpaceMode::Echo,
            _ => dsp::SpaceMode::Off,
        };
        self.post_chain.set_space_mode(space_mode);

        let post_quality = match self.params.quality_mode.value() {
            0 => dsp::PostQualityMode::Eco,
            1 => dsp::PostQualityMode::Normal,
            2 => dsp::PostQualityMode::High,
            3 => dsp::PostQualityMode::Render,
            _ => dsp::PostQualityMode::Normal,
        };
        self.post_chain.set_quality_mode(post_quality);
        self.post_chain
            .set_space_amount(self.params.space_amount.value());
        self.post_chain.set_factory_params(
            self.params.factory_size.value(),
            self.params.machinery_clutter.value(),
            self.params.wall_impedance.value(),
        );
        self.post_chain.set_spring_params(
            self.params.spring_tension.value(),
            self.params.wire_stiffness.value(),
            self.params.spring_tank_size.value(),
        );
        self.post_chain.set_echo_params(
            self.params.delay_time.value(),
            self.params.machinery_movement.value(),
            self.params.high_frequency_damping.value(),
        );
        self.post_chain.set_clipper_params(
            self.params.analog_ceiling.value(),
            self.params.diode_softness.value(),
        );

        // Process each sample in the buffer
        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Handle any MIDI events scheduled for this sample
            process_pending_events(self, sample_id as u32, &mut next_event, || {
                context.next_event()
            });

            // Generate stereo audio from all active voices
            let (left_sample, right_sample) =
                self.voice_manager.process_sample_stereo(sample_rate, width);

            let mut left = left_sample;
            let mut right = right_sample;

            // Apply drive/saturation
            left = apply_drive(left, drive);
            right = apply_drive(right, drive);

            // Apply post-processing chain
            let (post_left, post_right) = self.post_chain.process(left, right);
            left = post_left;
            right = post_right;

            // Apply output gain and limiting
            left = apply_output_limiter(left * output_gain);
            right = apply_output_limiter(right * output_gain);

            // Write output to buffer channels
            for (idx, channel_sample) in channel_samples.into_iter().enumerate() {
                if idx == 0 {
                    *channel_sample = left;
                } else if idx == 1 {
                    *channel_sample = right;
                }
            }
        }

        ProcessStatus::Normal
    }

    /// Creates the plugin editor GUI (if gui feature is enabled).
    ///
    /// # Returns
    /// Some(Editor) if GUI is available, None otherwise
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        #[cfg(feature = "gui")]
        {
            gui::create_editor(self.params.clone(), self.params.editor_state.clone())
        }
        #[cfg(not(feature = "gui"))]
        {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        apply_drive, apply_output_limiter, handle_note_event,
        note_event_velocity_to_voice_velocity, process_pending_events, Corrosion, Object,
    };
    use crate::params::object_param;
    use nih_plug::prelude::NoteEvent;
    use std::sync::Arc;

    /// Tests that multiple note events are processed correctly across a buffer.
    ///
    /// Verifies sample-accurate event timing and energy accumulation.
    #[test]
    fn processes_multiple_note_events_across_buffer() {
        let mut plugin = Corrosion::default();
        let mut single_note_plugin = Corrosion::default();
        let mut events = vec![
            NoteEvent::NoteOn {
                timing: 0,
                voice_id: None,
                channel: 0,
                note: 48,
                velocity: 1.0,
            },
            NoteEvent::NoteOn {
                timing: 3,
                voice_id: None,
                channel: 0,
                note: 52,
                velocity: 1.0,
            },
            NoteEvent::NoteOn {
                timing: 7,
                voice_id: None,
                channel: 0,
                note: 55,
                velocity: 1.0,
            },
        ]
        .into_iter();

        let mut next_event = events.next();

        for sample_id in 0..8 {
            process_pending_events(&mut plugin, sample_id, &mut next_event, || events.next());
        }

        handle_note_event(
            &mut single_note_plugin,
            NoteEvent::NoteOn {
                timing: 0,
                voice_id: None,
                channel: 0,
                note: 48,
                velocity: 1.0,
            },
        );

        // Compare energy of stacked notes vs single note
        let mut stacked_energy = 0.0f32;
        let mut single_note_energy = 0.0f32;
        for _ in 0..48_000 {
            stacked_energy += plugin.voice_manager.process_sample(48_000).abs();
            single_note_energy += single_note_plugin
                .voice_manager
                .process_sample(48_000)
                .abs();
        }

        assert!(
            stacked_energy > single_note_energy * 1.15,
            "stacked notes should accumulate energy, stacked={stacked_energy} single={single_note_energy}"
        );
        assert!(next_event.is_none(), "all queued events should be consumed");
    }

    #[test]
    fn note_event_velocity_is_converted_for_voice_layer() {
        assert_eq!(note_event_velocity_to_voice_velocity(0.0), 0.0);
        assert_eq!(note_event_velocity_to_voice_velocity(0.5), 63.5);
        assert_eq!(note_event_velocity_to_voice_velocity(1.0), 127.0);
    }

    #[test]
    fn hosted_normalized_note_velocity_reaches_output_chain() {
        let mut plugin = Corrosion::default();
        handle_note_event(
            &mut plugin,
            NoteEvent::NoteOn {
                timing: 0,
                voice_id: None,
                channel: 0,
                note: 60,
                velocity: 1.0,
            },
        );

        let sample_rate = 48_000u32;
        let mut peak = 0.0f32;
        for _ in 0..4096 {
            let (left_sample, right_sample) = plugin
                .voice_manager
                .process_sample_stereo(sample_rate, plugin.params.width.value());

            let mut left = apply_drive(left_sample, plugin.params.drive.value());
            let mut right = apply_drive(right_sample, plugin.params.drive.value());

            plugin.post_chain.set_sample_rate(sample_rate as f32);
            plugin.post_chain.set_filter_params(
                plugin.params.filter_cutoff.value(),
                plugin.params.filter_resonance.value(),
                plugin.params.component_tolerance.value(),
            );
            plugin.post_chain.set_drive_params(
                plugin.params.drive_amount.value(),
                plugin.params.bias_starvation.value(),
                plugin.params.chaos_depth.value(),
            );
            plugin.post_chain.set_body_params(
                plugin.params.chassis_material.value(),
                plugin.params.chassis_volume.value(),
            );
            plugin.post_chain.set_spread_params(
                plugin.params.spread_width.value(),
                plugin.params.listener_proximity.value(),
            );
            plugin.post_chain.set_space_mode(crate::dsp::SpaceMode::Off);
            plugin
                .post_chain
                .set_space_amount(plugin.params.space_amount.value());
            plugin.post_chain.set_clipper_params(
                plugin.params.analog_ceiling.value(),
                plugin.params.diode_softness.value(),
            );

            (left, right) = plugin.post_chain.process(left, right);
            left = apply_output_limiter(left * plugin.params.output.value());
            right = apply_output_limiter(right * plugin.params.output.value());
            peak = peak.max(left.abs()).max(right.abs());
        }

        eprintln!("hosted normalized note peak={peak:.6}");
        assert!(
            peak > 0.01,
            "hosted note should produce audible output, peak={peak}"
        );
    }

    /// Tests that Object parameter displays correct names.
    #[test]
    fn object_param_displays_names() {
        let params = crate::CorrosionParams::default();

        assert_eq!(params.object.to_string(), Object::Pipe.name());
    }

    /// Tests that changing object parameter affects runtime output.
    #[test]
    fn direct_object_param_changes_runtime_profile() {
        let mut pipe_plugin = Corrosion::default();
        let mut tank_plugin = Corrosion::default();

        Arc::get_mut(&mut pipe_plugin.params).unwrap().object = object_param(Object::Pipe.to_int());
        Arc::get_mut(&mut tank_plugin.params).unwrap().object = object_param(Object::Tank.to_int());

        let event = NoteEvent::NoteOn {
            timing: 0,
            voice_id: None,
            channel: 0,
            note: 60,
            velocity: 1.0,
        };
        handle_note_event(&mut pipe_plugin, event);
        handle_note_event(&mut tank_plugin, event);

        let sample_rate = 48_000u32;
        let pipe_energy: f32 = (0..4096)
            .map(|_| pipe_plugin.voice_manager.process_sample(sample_rate).abs())
            .sum();
        let tank_energy: f32 = (0..4096)
            .map(|_| tank_plugin.voice_manager.process_sample(sample_rate).abs())
            .sum();

        assert_ne!(
            pipe_energy, tank_energy,
            "expected direct object param to change runtime profile output"
        );
    }
}

/// CLAP plugin format implementation
impl ClapPlugin for Corrosion {
    const CLAP_ID: &'static str = "com.corrosion.corrosion";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Industrial physical modeling synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Instrument, ClapFeature::Synthesizer];
}

/// VST3 plugin format implementation
impl Vst3Plugin for Corrosion {
    const VST3_CLASS_ID: [u8; 16] = *b"CorrosionAudio01";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

// Export plugin entry points for CLAP and VST3 hosts.
nih_export_clap!(Corrosion);
nih_export_vst3!(Corrosion);
