use crate::dsp::{DamageAmount, ModalProfileId, ModalResonator, ResonatorCore, RustAmount, ScrapeExciter, SizeScale};

pub mod manager;

pub use manager::{VoiceManager, MAX_VOICES};

pub fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2_f32.powf((note as f32 - 69.0) / 12.0)
}

pub struct Voice {
    active: bool,
    note: u8,
    velocity: f32,
    exciter_type: i32,
    excitation_sent: bool,
    resonator: ModalResonator,
    scrape_exciter: ScrapeExciter,
    peak_hold: f32,
    frames_below_threshold: u32,
    start_frame: u64,
    excitation_value: f32,
    excitation_state: f32,
    excitation_decay: f32,
    highpass_state: f32,
    damage_amount: f32,
    rattle_phase: f32,
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
            exciter_type: 0,
            excitation_sent: false,
            resonator: ModalResonator::with_profile(ModalProfileId::Pipe),
            scrape_exciter: ScrapeExciter::new(),
            peak_hold: 0.0,
            frames_below_threshold: 0,
            start_frame: 0,
            excitation_value: 0.0,
            excitation_state: 0.0,
            excitation_decay: 0.5,
            highpass_state: 0.0,
            damage_amount: 0.0,
            rattle_phase: 0.0,
        }
    }

    pub fn note_on(
        &mut self,
        note: u8,
        velocity: f32,
        profile_id: ModalProfileId,
        start_frame: u64,
        size: f32,
        rust: f32,
        damage: f32,
        exciter_type: i32,
    ) {
        self.active = true;
        self.note = note;
        self.velocity = velocity;
        self.exciter_type = exciter_type;
        self.excitation_sent = false;
        self.peak_hold = 0.0;
        self.frames_below_threshold = 0;
        self.start_frame = start_frame;
        let velocity_norm = (velocity / 127.0).clamp(0.0, 1.0);
        let clamped_damage = damage.clamp(0.0, 1.0);
        self.damage_amount = clamped_damage;
        self.resonator = ModalResonator::with_profile_size_rust_and_damage(
            profile_id,
            SizeScale::new(size),
            RustAmount::new(rust),
            DamageAmount::new(clamped_damage),
        );

        self.scrape_exciter.set_parameters(0.5 + velocity_norm * 0.3, 0.3 + velocity_norm * 0.4, 0.2 + clamped_damage * 0.3);
        self.scrape_exciter.trigger(velocity_norm);

        self.excitation_value = velocity_norm;
        self.excitation_state = 0.0;
        self.excitation_decay = 0.90 - (velocity_norm * 0.70);
        self.highpass_state = 0.0;
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

    fn rattle_noise(&mut self, signal_level: f32) -> f32 {
        if self.damage_amount <= 0.0 {
            return 0.0;
        }

        let threshold = 0.02 + (1.0 - self.damage_amount) * 0.15;
        if signal_level < threshold {
            return 0.0;
        }

        self.rattle_phase += 1.61803398875;
        if self.rattle_phase > 1000.0 {
            self.rattle_phase -= 1000.0;
        }

        let noise = (self.rattle_phase.sin() * 43758.5453).fract() * 2.0 - 1.0;
        let highpass_noise = noise * 0.5;
        let rattle_amount = self.damage_amount * (signal_level - threshold) * 0.08;
        highpass_noise * rattle_amount
    }

    pub fn process_sample(&mut self, sample_rate: u32) -> f32 {
        let velocity_norm = self.velocity / 127.0;

        let excitation = if self.exciter_type == 1 {
            if self.active {
                let resonator_vel = self.resonator.process_sample(0.0, sample_rate);
                self.scrape_exciter.process_sample(resonator_vel)
            } else {
                0.0
            }
        } else if self.excitation_value > 0.0 {
            let value = self.excitation_value;
            self.excitation_value *= self.excitation_decay;
            if self.excitation_value < 1e-6 {
                self.excitation_value = 0.0;
            }
            value
        } else {
            0.0
        };

        let filtered_excitation = excitation;

        let sample = self.resonator.process_sample(filtered_excitation, sample_rate);

        let sample = if !sample.is_finite() {
            0.0
        } else if sample.abs() < 1e-30 {
            0.0
        } else {
            sample
        };

        let rattle = self.rattle_noise(sample.abs());
        let sample_with_rattle = sample + rattle;

        self.highpass_state = 0.8 * self.highpass_state + 0.2 * sample_with_rattle;
        let highpass = sample_with_rattle - self.highpass_state;
        let boost_amount = velocity_norm * 3.0;
        let boosted_sample = sample_with_rattle + (highpass * boost_amount);

        let clamped = boosted_sample.clamp(-1.0, 1.0);
        const DENORMAL_FLUSH: f32 = 1e-20;
        let flushed = clamped + DENORMAL_FLUSH - DENORMAL_FLUSH;

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

        flushed
    }

    pub fn process_sample_stereo(&mut self, sample_rate: u32, width: f32) -> (f32, f32) {
        let velocity_norm = self.velocity / 127.0;

        let excitation = if self.exciter_type == 1 {
            if self.active {
                let (resonator_l, resonator_r) = self.resonator.process_sample_stereo(0.0, sample_rate, width);
                let resonator_vel = (resonator_l + resonator_r) * 0.5;
                self.scrape_exciter.process_sample(resonator_vel)
            } else {
                0.0
            }
        } else if self.excitation_value > 0.0 {
            let value = self.excitation_value;
            self.excitation_value *= self.excitation_decay;
            if self.excitation_value < 1e-6 {
                self.excitation_value = 0.0;
            }
            value
        } else {
            0.0
        };

        let filtered_excitation = excitation;

        let (left, right) = self.resonator.process_sample_stereo(filtered_excitation, sample_rate, width);

        let left = if !left.is_finite() { 0.0 } else if left.abs() < 1e-30 { 0.0 } else { left };
        let right = if !right.is_finite() { 0.0 } else if right.abs() < 1e-30 { 0.0 } else { right };

        let mono = (left + right) * 0.5;
        let rattle = self.rattle_noise(mono.abs());
        let left_with_rattle = left + rattle;
        let right_with_rattle = right + rattle;

        self.highpass_state = 0.8 * self.highpass_state + 0.2 * (mono + rattle);
        let highpass = (mono + rattle) - self.highpass_state;
        let boost_amount = velocity_norm * 3.0;
        let boosted_left = left_with_rattle + (highpass * boost_amount);
        let boosted_right = right_with_rattle + (highpass * boost_amount);

        let clamped_left = boosted_left.clamp(-1.0, 1.0);
        let clamped_right = boosted_right.clamp(-1.0, 1.0);
        const DENORMAL_FLUSH: f32 = 1e-20;
        let flushed_left = clamped_left + DENORMAL_FLUSH - DENORMAL_FLUSH;
        let flushed_right = clamped_right + DENORMAL_FLUSH - DENORMAL_FLUSH;

        if self.active {
            let peak = clamped_left.abs().max(clamped_right.abs());
            self.peak_hold = self.peak_hold.max(peak);
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

        (flushed_left, flushed_right)
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
        voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);
        assert!(voice.is_active());
    }

    #[test]
    fn note_off_deactivates_voice() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);
        voice.note_off();
        assert!(!voice.is_active());
    }

    #[test]
    fn note_off_natural_decay() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);

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
        voice.note_on(60, 127.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);

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
        voice.note_on(36, 127.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);

        let sample_rate = 48_000u32;
        for _ in 0..96_000 {
            let sample = voice.process_sample(sample_rate);
            assert!(sample.is_finite(), "non-finite output: {sample}");
        }
    }

    #[test]
    fn active_voice_not_falsely_deactivated() {
        let mut voice = Voice::new();
        voice.note_on(60, 100.0, ModalProfileId::Tank, 0, 1.0, 0.0, 0.0, 0);

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
