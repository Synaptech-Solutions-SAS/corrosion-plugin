use super::Voice;

pub const MAX_VOICES: usize = 8;

pub struct VoiceManager {
    voices: [Voice; MAX_VOICES],
    frame_counter: u64,
}

impl VoiceManager {
    pub fn new() -> Self {
        Self {
            voices: [
                Voice::new(),
                Voice::new(),
                Voice::new(),
                Voice::new(),
                Voice::new(),
                Voice::new(),
                Voice::new(),
                Voice::new(),
            ],
            frame_counter: 0,
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: f32, profile_id: crate::dsp::ModalProfileId) {
        // First try: find an inactive voice
        if let Some(idx) = self.voices.iter().position(|v| !v.is_active()) {
            self.voices[idx].note_on(note, velocity, profile_id, self.frame_counter);
            return;
        }

        // Steal: find the voice with lowest peak_hold, break ties by oldest start_frame, then lowest index
        let steal_idx = self
            .voices
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let peak_cmp = a.peak_hold().partial_cmp(&b.peak_hold()).unwrap();
                if peak_cmp != std::cmp::Ordering::Equal {
                    return peak_cmp;
                }
                let age_cmp = a.start_frame().cmp(&b.start_frame());
                if age_cmp != std::cmp::Ordering::Equal {
                    return age_cmp;
                }
                std::cmp::Ordering::Equal
            })
            .map(|(idx, _)| idx);

        if let Some(idx) = steal_idx {
            self.voices[idx].note_on(note, velocity, profile_id, self.frame_counter);
        }
    }

    pub fn note_off(&mut self, note: u8) {
        if let Some(idx) = self.voices.iter().position(|v| v.is_active() && v.note() == note) {
            self.voices[idx].note_off();
        }
    }

    pub fn process_sample(&mut self, sample_rate: u32) -> f32 {
        self.frame_counter += 1;
        let mut sum = 0.0f32;
        for voice in &mut self.voices {
            sum += voice.process_sample(sample_rate);
        }
        sum / MAX_VOICES as f32
    }
}

impl Default for VoiceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{VoiceManager, MAX_VOICES};

    #[test]
    fn eight_simultaneous_notes_audible() {
        let mut manager = VoiceManager::new();
        let notes = [48u8, 51, 55, 59, 62, 65, 69, 72];
        for &note in &notes {
            manager.note_on(note, 100.0, crate::dsp::ModalProfileId::Pipe);
        }

        let sample_rate = 48_000u32;
        let mut peak = 0.0f32;
        for _ in 0..48_000 {
            let sample = manager.process_sample(sample_rate);
            peak = peak.max(sample.abs());
            assert!(sample.is_finite(), "non-finite output");
        }
        assert!(
            peak > 0.01,
            "8 simultaneous notes should be audible, peak={peak}"
        );
    }

    #[test]
    fn ninth_note_does_not_crash() {
        let mut manager = VoiceManager::new();
        for note in 0..12u8 {
            manager.note_on(48 + note, 100.0, crate::dsp::ModalProfileId::Pipe);
        }
        manager.note_on(72, 100.0, crate::dsp::ModalProfileId::Pipe);
        let sample_rate = 48_000u32;
        let sample = manager.process_sample(sample_rate);
        assert!(sample.is_finite());
    }

    #[test]
    fn note_off_deactivates_correct_voice() {
        let mut manager = VoiceManager::new();
        manager.note_on(60, 100.0, crate::dsp::ModalProfileId::Pipe);
        manager.note_on(64, 100.0, crate::dsp::ModalProfileId::Pipe);
        manager.note_off(60);

        let active_count = manager.voices.iter().filter(|v| v.is_active()).count();
        assert_eq!(active_count, 1, "only one voice should remain active");
    }

    #[test]
    fn steals_quietest_voice_when_all_active() {
        let mut manager = VoiceManager::new();
        // Fill all 8 slots
        for note in 0..MAX_VOICES as u8 {
            manager.note_on(48 + note, 100.0, crate::dsp::ModalProfileId::Pipe);
        }
        // Render enough frames for most voices to decay below their initial peak
        let sample_rate = 48_000u32;
        for _ in 0..48_000 {
            manager.process_sample(sample_rate);
        }
        // Record which notes were active before the 9th note
        let active_notes_before: Vec<u8> =
            manager.voices.iter().filter(|v| v.is_active()).map(|v| v.note()).collect();

        // Send a 9th note — should steal the quietest (most decayed) voice
        manager.note_on(80, 100.0, crate::dsp::ModalProfileId::Pipe);

        // The new note should be present in the active voices
        let has_new_note = manager.voices.iter().any(|v| v.is_active() && v.note() == 80);
        assert!(has_new_note, "9th note should have stolen a voice slot");

        // Still only 8 active voices
        let active_count = manager.voices.iter().filter(|v| v.is_active()).count();
        assert_eq!(active_count, 8, "should still have exactly 8 active voices");

        // At least one original note should have been stolen
        let active_notes_after: Vec<u8> =
            manager.voices.iter().filter(|v| v.is_active()).map(|v| v.note()).collect();
        let stolen: Vec<u8> = active_notes_before
            .into_iter()
            .filter(|n| !active_notes_after.contains(n))
            .collect();
        assert!(!stolen.is_empty(), "at least one original voice should have been stolen");
    }
}
