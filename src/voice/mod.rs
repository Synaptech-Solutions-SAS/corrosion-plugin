use crate::dsp::{DamageAmount, ModalProfileId, PlaceholderResonator, ResonatorCore, RustAmount, SizeScale};

pub mod manager;

pub use manager::{VoiceManager, MAX_VOICES};

pub fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2_f32.powf((note as f32 - 69.0) / 12.0)
}

pub struct Voice {
    active: bool,
    note: u8,
    velocity: f32,
    excitation_sent: bool,
    resonator: PlaceholderResonator,
    peak_hold: f32,
    frames_below_threshold: u32,
    start_frame: u64,
}

const TAIL_ENERGY_THRESHOLD: f32 = 1e-4;
const TAIL_DEACTIVATE_FRAMES: u32 = 4800;
const PEAK_DECAY: f32 = 0.999;

impl Voice {
    pub fn new() -> Self {
        Self {
            active: false,
            note: 60,
            velocity: 0.0,
            excitation_sent: false,
            resonator: PlaceholderResonator::with_profile(ModalProfileId::Pipe),
            peak_hold: 0.0,
            frames_below_threshold: 0,
            start_frame: 0,
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: f32, profile_id: ModalProfileId, start_frame: u64) {
        self.active = true;
        self.note = note;
        self.velocity = velocity;
        self.excitation_sent = false;
        self.peak_hold = 0.0;
        self.frames_below_threshold = 0;
        self.start_frame = start_frame;
        self.resonator = PlaceholderResonator::with_profile_size_rust_and_damage(
            profile_id,
            SizeScale::default(),
            RustAmount::default(),
            DamageAmount::default(),
        );
    }

    pub fn note_off(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn note(&self) -> u8 {
        self.note
    }

    pub fn peak_hold(&self) -> f32 {
        self.peak_hold
    }

    pub fn start_frame(&self) -> u64 {
        self.start_frame
    }

    pub fn process_sample(&mut self, sample_rate: u32) -> f32 {
        let excitation = if !self.excitation_sent && self.velocity > 0.0 {
            self.excitation_sent = true;
            self.velocity / 127.0
        } else {
            0.0
        };

        let sample = self.resonator.process_sample(excitation, sample_rate);

        let sample = if !sample.is_finite() {
            0.0
        } else if sample.abs() < 1e-30 {
            0.0
        } else {
            sample
        };

        let clamped = sample.clamp(-1.0, 1.0);

        if self.active {
            self.peak_hold = self.peak_hold.max(clamped.abs());
            self.peak_hold *= PEAK_DECAY;
            if self.peak_hold < TAIL_ENERGY_THRESHOLD {
                self.frames_below_threshold += 1;
                if self.frames_below_threshold >= TAIL_DEACTIVATE_FRAMES {
                    self.active = false;
                }
            } else {
                self.frames_below_threshold = 0;
            }
        }

        clamped
    }
}

impl Default for Voice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{midi_to_hz, Voice};
    use crate::dsp::ModalProfileId;

    #[test]
    fn midi_69_is_a440() {
        assert!((midi_to_hz(69) - 440.0).abs() < 1e-3);
    }

    #[test]
    fn midi_57_is_a220() {
        assert!((midi_to_hz(57) - 220.0).abs() < 1e-3);
    }

    #[test]
    fn midi_60_is_c4() {
        let hz = midi_to_hz(60);
        assert!((hz - 261.626).abs() < 0.1);
    }

    #[test]
    fn voice_starts_inactive() {
        let voice = Voice::new();
        assert!(!voice.is_active());
    }

    #[test]
    fn note_on_activates_voice() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Pipe, 0);
        assert!(voice.is_active());
    }

    #[test]
    fn note_off_deactivates_voice() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Pipe, 0);
        voice.note_off();
        assert!(!voice.is_active());
    }

    #[test]
    fn note_off_natural_decay() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Pipe, 0);

        let sample_rate = 48_000u32;
        let note_off_frame = 4800;
        let total_frames = 48_000;

        let mut output = Vec::with_capacity(total_frames);
        for frame in 0..total_frames {
            if frame == note_off_frame {
                voice.note_off();
            }
            output.push(voice.process_sample(sample_rate));
        }

        let pre_rms: f32 = output[..note_off_frame]
            .iter()
            .map(|s| s * s)
            .sum::<f32>()
            .sqrt()
            / note_off_frame as f32;
        let post_rms: f32 = output[note_off_frame..]
            .iter()
            .map(|s| s * s)
            .sum::<f32>()
            .sqrt()
            / (total_frames - note_off_frame) as f32;

        assert!(pre_rms > 0.0, "pre-off should be audible");
        assert!(
            post_rms > pre_rms * 0.10,
            "post-off RMS ({post_rms}) should be > 10% of pre-off RMS ({pre_rms}) — natural decay, not cut"
        );
    }

    #[test]
    fn output_clamped_to_unit_range() {
        let mut voice = Voice::new();
        voice.note_on(60, 127.0, ModalProfileId::Pipe, 0);

        let sample_rate = 48_000u32;
        for _ in 0..48_000 {
            let sample = voice.process_sample(sample_rate);
            assert!(
                sample >= -1.0 && sample <= 1.0,
                "output sample {sample} outside [-1, 1]"
            );
        }
    }

    #[test]
    fn output_finite_over_long_render() {
        let mut voice = Voice::new();
        voice.note_on(36, 127.0, ModalProfileId::Pipe, 0);

        let sample_rate = 48_000u32;
        for _ in 0..96_000 {
            let sample = voice.process_sample(sample_rate);
            assert!(sample.is_finite(), "non-finite output: {sample}");
        }
    }

    #[test]
    fn active_voice_not_falsely_deactivated() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Tank, 0);

        let sample_rate = 48_000u32;
        let half_second = 48_000 / 2;

        for _ in 0..half_second {
            voice.process_sample(sample_rate);
        }

        assert!(
            voice.is_active(),
            "voice should still be active at t=0.5s for tank profile"
        );
    }
}
