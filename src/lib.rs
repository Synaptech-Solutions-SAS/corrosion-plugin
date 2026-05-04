pub mod dsp;
pub mod gui;
pub mod macros;
pub mod offline;
pub mod presets;
pub mod randomizer;
pub mod voice;
mod params;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;
use voice::VoiceManager;

pub use params::{CorrosionParams, Object};
pub use presets::Preset;

pub const LIMITER_THRESHOLD: f32 = 0.9661;

#[inline]
pub fn apply_output_limiter(sample: f32) -> f32 {
    sample.clamp(-LIMITER_THRESHOLD, LIMITER_THRESHOLD)
}

#[inline]
pub fn apply_drive(sample: f32, drive: f32) -> f32 {
    if drive <= 0.0 {
        return sample;
    }
    
    let drive_gain = 1.0 + drive * 4.0;
    let amplified = sample * drive_gain;
    
    let abs_sample = amplified.abs();
    let sign = amplified.signum();
    
    let shaped = if sign > 0.0 {
        let soft_threshold = 0.3;
        let hard_threshold = 0.8;
        
        if abs_sample < soft_threshold {
            amplified
        } else if abs_sample < hard_threshold {
            let t = (abs_sample - soft_threshold) / (hard_threshold - soft_threshold);
            let eased = soft_threshold + (hard_threshold - soft_threshold) * (t * t * (3.0 - 2.0 * t));
            sign * eased
        } else {
            let t = (abs_sample - hard_threshold) / (1.0 + drive_gain);
            let compressed = hard_threshold + (1.0 - hard_threshold) * (1.0 - (-t).exp());
            sign * compressed.min(1.2)
        }
    } else {
        let soft_threshold = 0.25;
        let hard_threshold = 0.7;
        
        if abs_sample < soft_threshold {
            amplified
        } else if abs_sample < hard_threshold {
            let t = (abs_sample - soft_threshold) / (hard_threshold - soft_threshold);
            let eased = soft_threshold + (hard_threshold - soft_threshold) * (t * t * (3.0 - 2.0 * t));
            sign * eased
        } else {
            let t = (abs_sample - hard_threshold) / (1.0 + drive_gain);
            let compressed = hard_threshold + (1.0 - hard_threshold) * (1.0 - (-t).exp());
            sign * compressed.min(1.1)
        }
    };
    
    let dry_wet = shaped * drive + sample * (1.0 - drive * 0.5);
    dry_wet.clamp(-1.5, 1.5)
}

fn handle_note_event(plugin: &mut Corrosion, event: NoteEvent<()>) {
    match event {
        NoteEvent::NoteOn { note, velocity, .. } => {
            let profile = match Object::from_int(plugin.params.object.value()) {
                Object::Pipe => dsp::ModalProfileId::Pipe,
                Object::Plate => dsp::ModalProfileId::Plate,
                Object::Tank => dsp::ModalProfileId::Tank,
                Object::Chain => dsp::ModalProfileId::Chain,
            };
            let size = plugin.params.size.value();
            let rust = plugin.params.rust.value();
            let damage = plugin.params.damage.value();
            let exciter_type = plugin.params.exciter.value();
            plugin
                .voice_manager
                .note_on(note, velocity as f32, profile, size, rust, damage, exciter_type);
        }
        NoteEvent::NoteOff { note, .. } => {
            plugin.voice_manager.note_off(note);
        }
        _ => {}
    }
}

fn process_pending_events<F>(
    plugin: &mut Corrosion,
    sample_id: u32,
    next_event: &mut Option<NoteEvent<()>>,
    mut fetch_next: F,
) where
    F: FnMut() -> Option<NoteEvent<()>>,
{
    while let Some(event) = *next_event {
        if event.timing() > sample_id {
            break;
        }

        handle_note_event(plugin, event);
        *next_event = fetch_next();
    }
}

pub fn corrosion_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Corrosion {
    params: Arc<CorrosionParams>,
    voice_manager: VoiceManager,
    body_resonator: dsp::BodyResonator,
}

impl Default for Corrosion {
    fn default() -> Self {
        Self {
            params: Arc::new(CorrosionParams::default()),
            voice_manager: VoiceManager::new(),
            body_resonator: dsp::BodyResonator::new(),
        }
    }
}

impl Corrosion {
    pub fn get_state(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(&Preset::from_params(Self::NAME, &self.params)).unwrap_or_default()
    }

    pub fn load_state(&mut self, state: &[u8]) {
        if let Ok(preset) = serde_json::from_slice::<Preset>(state) {
            self.params = Arc::new(preset.into_params());
        }
    }
}

impl Plugin for Corrosion {
    const NAME: &'static str = "Corrosion";
    const VENDOR: &'static str = "Corrosion Audio";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {
        self.voice_manager = VoiceManager::new();
        self.body_resonator = dsp::BodyResonator::new();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let sample_rate = context.transport().sample_rate as u32;

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            process_pending_events(self, sample_id as u32, &mut next_event, || context.next_event());

            let drive = self.params.drive.value();
            let output_gain = self.params.output.value();
            let width = self.params.width.value();
            let body_amount = self.params.body.value();
            
            let (left_sample, right_sample) = self.voice_manager.process_sample_stereo(sample_rate, width);
            
            let mut left = left_sample;
            let mut right = right_sample;
            
            left = apply_drive(left, drive);
            right = apply_drive(right, drive);
            
            let mono_for_body = (left + right) * 0.5;
            let body_out = self.body_resonator.process_sample(mono_for_body, sample_rate, body_amount);
            let body_diff = body_out - mono_for_body;
            left += body_diff;
            right += body_diff;
            
            left *= output_gain;
            right *= output_gain;
            left = apply_output_limiter(left);
            right = apply_output_limiter(right);
            
            let mut idx = 0;
            for channel_sample in channel_samples.into_iter() {
                if idx == 0 {
                    *channel_sample = left;
                } else if idx == 1 {
                    *channel_sample = right;
                }
                idx += 1;
            }
        }

        ProcessStatus::Normal
    }

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
    use super::{handle_note_event, process_pending_events, Corrosion, Object};
    use nih_plug::prelude::NoteEvent;

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

        let mut stacked_energy = 0.0f32;
        let mut single_note_energy = 0.0f32;
        for _ in 0..48_000 {
            stacked_energy += plugin.voice_manager.process_sample(48_000).abs();
            single_note_energy += single_note_plugin.voice_manager.process_sample(48_000).abs();
        }

        assert!(
            stacked_energy > single_note_energy * 1.25,
            "stacked notes should accumulate energy, stacked={stacked_energy} single={single_note_energy}"
        );
        assert!(next_event.is_none(), "all queued events should be consumed");
    }

    #[test]
    fn object_param_displays_names() {
        let params = crate::CorrosionParams::default();

        assert_eq!(params.object.to_string(), Object::Pipe.name());
    }
}

impl ClapPlugin for Corrosion {
    const CLAP_ID: &'static str = "com.corrosion.corrotion";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Industrial physical modeling synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Synthesizer];
}

impl Vst3Plugin for Corrosion {
    const VST3_CLASS_ID: [u8; 16] = *b"CorrosionAudio01";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Corrosion);
nih_export_vst3!(Corrosion);
