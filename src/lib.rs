pub mod dsp;
pub mod offline;
pub mod presets;
pub mod voice;
mod params;

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;
#[cfg(feature = "gui")]
use nih_plug_egui::{create_egui_editor, egui, widgets};
use voice::VoiceManager;

pub use params::{CorrosionParams, Object};
pub use presets::Preset;

pub fn corrosion_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Corrosion {
    params: Arc<CorrosionParams>,
    voice_manager: VoiceManager,
}

impl Default for Corrosion {
    fn default() -> Self {
        Self {
            params: Arc::new(CorrosionParams::default()),
            voice_manager: VoiceManager::new(),
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
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let next_event = context.next_event();
        let sample_rate = context.transport().sample_rate as u32;

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            if let Some(event) = next_event {
                if event.timing() == sample_id as u32 {
                    match event {
                        NoteEvent::NoteOn { note, velocity, .. } => {
                            let profile = match Object::from_int(self.params.object.value()) {
                                Object::Pipe => dsp::ModalProfileId::Pipe,
                                Object::Plate => dsp::ModalProfileId::Plate,
                                Object::Tank => dsp::ModalProfileId::Tank,
                            };
                            let size = self.params.size.value();
                            let rust = self.params.rust.value();
                            let damage = self.params.damage.value();
                            self.voice_manager.note_on(note, velocity as f32, profile, size, rust, damage);
                        }
                        NoteEvent::NoteOff { note, .. } => {
                            self.voice_manager.note_off(note);
                        }
                        _ => {}
                    }
                }
            }

            let drive = self.params.drive.value();
            let output_gain = self.params.output.value();
            let mut sample = self.voice_manager.process_sample(sample_rate);
            sample = (sample * (1.0 + drive * 3.0)).tanh();
            sample *= output_gain;
            sample = sample.clamp(-1.0, 1.0);
            let mut idx = 0;
            for channel_sample in channel_samples.into_iter() {
                if idx < 2 {
                    *channel_sample = sample;
                }
                idx += 1;
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        #[cfg(feature = "gui")]
        {
            let params = self.params.clone();
            create_egui_editor(
                self.params.editor_state.clone(),
                (),
                |_, _| {},
                move |egui_ctx, setter, _state| {
                    egui::CentralPanel::default().show(egui_ctx, |ui| {
                        ui.heading("Corrosion");
                        ui.separator();

                        ui.label("Object");
                        ui.add(widgets::ParamSlider::for_param(&params.object, setter));

                        ui.add_space(8.0);

                        ui.label("Size");
                        ui.add(widgets::ParamSlider::for_param(&params.size, setter));

                        ui.add_space(8.0);

                        ui.label("Rust");
                        ui.add(widgets::ParamSlider::for_param(&params.rust, setter));

                        ui.add_space(8.0);

                        ui.label("Damage");
                        ui.add(widgets::ParamSlider::for_param(&params.damage, setter));

                        ui.add_space(8.0);

                        ui.label("Drive");
                        ui.add(widgets::ParamSlider::for_param(&params.drive, setter));

                        ui.add_space(8.0);

                        ui.label("Output");
                        ui.add(widgets::ParamSlider::for_param(&params.output, setter));
                    });
                },
            )
        }
        #[cfg(not(feature = "gui"))]
        {
            None
        }
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
