//! Fixed-size voice pool and scheduling logic for Corrosion.
//!
//! `VoiceManager` owns the polyphonic voice array, handles note-on
//! and note-off dispatch, and performs quietest-voice stealing when
//! all slots are occupied. `src/lib.rs` calls into this module from
//! the host audio callback.

use super::{Voice, VoiceControls};

/// Maximum number of simultaneously active voices.
pub const MAX_VOICES: usize = 8;

/// Fixed-size polyphonic voice pool with allocation-free scheduling.
pub struct VoiceManager {
    voices: [Voice; MAX_VOICES],
    frame_counter: u64,
}

impl VoiceManager {
/// Create a voice manager with all slots initialized.
///
/// # Returns
///
/// A manager ready to receive note events.
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

/// Trigger a note using default voice controls.
///
/// This is the simple wrapper used when the caller does not need
/// per-note control overrides.
///
/// # Arguments
///
/// * `note` - MIDI note number for the new voice.
/// * `velocity` - MIDI velocity used to scale excitation energy.
/// * `profile_id` - Resonator profile to load for the voice.
/// * `size` - Material size parameter forwarded to the voice.
/// * `rust` - Material rust parameter forwarded to the voice.
/// * `damage` - Material damage parameter forwarded to the voice.
/// * `exciter_type` - Integer selector for the exciter model.
    pub fn note_on(
        &mut self,
        note: u8,
        velocity: f32,
        profile_id: crate::dsp::ModalProfileId,
        size: f32,
        rust: f32,
        damage: f32,
        exciter_type: i32,
    ) {
        self.note_on_with_controls(
            note,
            velocity,
            profile_id,
            size,
            rust,
            damage,
            exciter_type,
            VoiceControls::default(),
        );
    }

/// Trigger a note with explicit voice controls.
///
/// # Arguments
///
/// * `note` - MIDI note number for the new voice.
/// * `velocity` - MIDI velocity used to scale excitation energy.
/// * `profile_id` - Resonator profile to load for the voice.
/// * `size` - Material size parameter forwarded to the voice.
/// * `rust` - Material rust parameter forwarded to the voice.
/// * `damage` - Material damage parameter forwarded to the voice.
/// * `exciter_type` - Integer selector for the exciter model.
/// * `controls` - Complete per-voice control snapshot.
    pub fn note_on_with_controls(
        &mut self,
        note: u8,
        velocity: f32,
        profile_id: crate::dsp::ModalProfileId,
        size: f32,
        rust: f32,
        damage: f32,
        exciter_type: i32,
        controls: VoiceControls,
    ) {
        // First try: find an inactive voice; this keeps the common case
        // deterministic and avoids unnecessary stealing.
        if let Some(idx) = self.voices.iter().position(|v| !v.is_active()) {
            self.voices[idx].note_on_with_controls(
                note,
                velocity,
                profile_id,
                self.frame_counter,
                size,
                rust,
                damage,
                exciter_type,
                controls,
            );
            return;
        }

        // Steal the quietest voice when all slots are occupied. Peak hold is
        // the primary score; older voices lose ties so long tails are preferred
        // over recently triggered notes.
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
            self.voices[idx].note_on_with_controls(
                note,
                velocity,
                profile_id,
                self.frame_counter,
                size,
                rust,
                damage,
                exciter_type,
                controls,
            );
        }
    }

    pub fn note_off(&mut self, note: u8) {
        if let Some(idx) = self
            .voices
            .iter()
            .position(|v| v.is_active() && v.note() == note)
        {
            self.voices[idx].note_off();
        }
    }

/// Render the current mono sample for the whole voice pool.
///
/// # Arguments
///
/// * `sample_rate` - Current host sample rate in hertz.
///
/// # Returns
///
/// The mixed mono output sample.
    pub fn process_sample(&mut self, sample_rate: u32) -> f32 {
        // Advance the shared frame clock so note age can participate in
        // tie-breaking during voice stealing.
        // Advance the shared frame clock so note age can participate in
        // tie-breaking during voice stealing.
        self.frame_counter += 1;
        let mut sum = 0.0f32;
        for voice in &mut self.voices {
            sum += voice.process_sample(sample_rate);
        }
        sum / MAX_VOICES as f32
    }

/// Render the current stereo sample pair for the whole voice pool.
///
/// # Arguments
///
/// * `sample_rate` - Current host sample rate in hertz.
/// * `width` - Stereo width forwarded to each voice.
///
/// # Returns
///
/// The mixed `(left, right)` output samples.
    pub fn process_sample_stereo(&mut self, sample_rate: u32, width: f32) -> (f32, f32) {
        // Advance the shared frame clock so note age can participate in
        // tie-breaking during voice stealing.
        self.frame_counter += 1;
        let mut left_sum = 0.0f32;
        let mut right_sum = 0.0f32;
        for voice in &mut self.voices {
            let (left, right) = voice.process_sample_stereo(sample_rate, width);
            left_sum += left;
            right_sum += right;
        }
        let scale = 1.0 / MAX_VOICES as f32;
        (left_sum * scale, right_sum * scale)
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
            manager.note_on(
                note,
                100.0,
                crate::dsp::ModalProfileId::Pipe,
                1.0,
                0.0,
                0.0,
                0,
            );
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
            manager.note_on(
                48 + note,
                100.0,
                crate::dsp::ModalProfileId::Pipe,
                1.0,
                0.0,
                0.0,
                0,
            );
        }
        manager.note_on(
            72,
            100.0,
            crate::dsp::ModalProfileId::Pipe,
            1.0,
            0.0,
            0.0,
            0,
        );
        let sample_rate = 48_000u32;
        let sample = manager.process_sample(sample_rate);
        assert!(sample.is_finite());
    }

    #[test]
    fn note_off_deactivates_correct_voice() {
        let mut manager = VoiceManager::new();
        manager.note_on(
            60,
            100.0,
            crate::dsp::ModalProfileId::Pipe,
            1.0,
            0.0,
            0.0,
            0,
        );
        manager.note_on(
            64,
            100.0,
            crate::dsp::ModalProfileId::Pipe,
            1.0,
            0.0,
            0.0,
            0,
        );
        manager.note_off(60);

        let active_count = manager.voices.iter().filter(|v| v.is_active()).count();
        assert_eq!(active_count, 1, "only one voice should remain active");
    }

    #[test]
    fn steals_quietest_voice_when_all_active() {
        let mut manager = VoiceManager::new();
        // Fill all 8 slots
        for note in 0..MAX_VOICES as u8 {
            manager.note_on(
                48 + note,
                100.0,
                crate::dsp::ModalProfileId::Pipe,
                1.0,
                0.0,
                0.0,
                0,
            );
        }
        // Render enough frames for most voices to decay below their initial peak
        let sample_rate = 48_000u32;
        for _ in 0..48_000 {
            manager.process_sample(sample_rate);
        }
        // Record which notes were active before the 9th note
        let active_notes_before: Vec<u8> = manager
            .voices
            .iter()
            .filter(|v| v.is_active())
            .map(|v| v.note())
            .collect();

        // Send a 9th note — should steal the quietest (most decayed) voice
        manager.note_on(
            80,
            100.0,
            crate::dsp::ModalProfileId::Pipe,
            1.0,
            0.0,
            0.0,
            0,
        );

        // The new note should be present in the active voices
        let has_new_note = manager
            .voices
            .iter()
            .any(|v| v.is_active() && v.note() == 80);
        assert!(has_new_note, "9th note should have stolen a voice slot");

        // Still only 8 active voices
        let active_count = manager.voices.iter().filter(|v| v.is_active()).count();
        assert_eq!(active_count, 8, "should still have exactly 8 active voices");

        // At least one original note should have been stolen
        let active_notes_after: Vec<u8> = manager
            .voices
            .iter()
            .filter(|v| v.is_active())
            .map(|v| v.note())
            .collect();
        let stolen: Vec<u8> = active_notes_before
            .into_iter()
            .filter(|n| !active_notes_after.contains(n))
            .collect();
        assert!(
            !stolen.is_empty(),
            "at least one original voice should have been stolen"
        );
    }
}
