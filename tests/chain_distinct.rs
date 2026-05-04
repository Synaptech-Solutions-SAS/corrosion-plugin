use corrotion::dsp::ModalProfileId;
use corrotion::offline::render_behavior_metrics;
use corrotion::voice::Voice;

fn render_voice(profile: ModalProfileId) -> Vec<f32> {
    let sample_rate = 48_000u32;
    let frames = 48_000;
    let mut voice = Voice::new();

    voice.note_on(60, 100.0, profile, 0, 1.0, 0.0, 0.0, 0);

    let mut output = Vec::with_capacity(frames);
    for _ in 0..frames {
        output.push(voice.process_sample(sample_rate));
    }

    output
}

#[test]
fn chain_is_the_roughest_object_profile() {
    let pipe = render_voice(ModalProfileId::Pipe);
    let plate = render_voice(ModalProfileId::Plate);
    let tank = render_voice(ModalProfileId::Tank);
    let chain = render_voice(ModalProfileId::Chain);

    let pipe_metrics = render_behavior_metrics(&pipe);
    let plate_metrics = render_behavior_metrics(&plate);
    let tank_metrics = render_behavior_metrics(&tank);
    let chain_metrics = render_behavior_metrics(&chain);

    assert!(
        chain_metrics.roughness_proxy > pipe_metrics.roughness_proxy,
        "chain roughness={} pipe roughness={} plate roughness={} tank roughness={}",
        chain_metrics.roughness_proxy,
        pipe_metrics.roughness_proxy,
        plate_metrics.roughness_proxy,
        tank_metrics.roughness_proxy,
    );
    assert!(
        chain_metrics.roughness_proxy > plate_metrics.roughness_proxy,
        "chain roughness={} pipe roughness={} plate roughness={} tank roughness={}",
        chain_metrics.roughness_proxy,
        pipe_metrics.roughness_proxy,
        plate_metrics.roughness_proxy,
        tank_metrics.roughness_proxy,
    );
    assert!(
        chain_metrics.roughness_proxy > tank_metrics.roughness_proxy,
        "chain roughness={} pipe roughness={} plate roughness={} tank roughness={}",
        chain_metrics.roughness_proxy,
        pipe_metrics.roughness_proxy,
        plate_metrics.roughness_proxy,
        tank_metrics.roughness_proxy,
    );
}
