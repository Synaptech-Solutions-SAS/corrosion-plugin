use assert_no_alloc::assert_no_alloc;
use corrotion::dsp::ModalProfileId;
use corrotion::voice::manager::VoiceManager;
use corrotion::voice::MAX_VOICES;

#[test]
fn process_callback_is_alloc_free() {
    let sample_rate = 48_000u32;
    let total_samples = 480_000; // 10 seconds at 48kHz
    let mut manager = VoiceManager::new();

    // Trigger 8 voices before entering the no-alloc zone.
    // The process callback itself is what must stay allocation-free.
    for i in 0..MAX_VOICES {
        manager.note_on(60 + i as u8, 100.0, ModalProfileId::Pipe, 1.0, 0.0, 0.0, 2);
    }

    assert_no_alloc(|| {
        // Process 10 seconds of audio
        for _ in 0..total_samples {
            let _ = manager.process_sample(sample_rate);
        }
    });
}
