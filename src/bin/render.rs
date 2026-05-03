use corrotion::offline::{OfflineRenderer, RenderConfig};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RenderConfig::default();
    let output_dir = Path::new("output/damage-variations");

    let artifacts = OfflineRenderer::new(config).render_damage_variations_to_dir(output_dir)?;

    println!("Offline render complete.");
    println!(
        "Damage variation artifacts written to {}",
        output_dir.display()
    );

    for artifact in artifacts {
        println!(
            concat!(
                "profile={} variant={} damage_amount={:.2} wav={} summary={} ",
                "sample_rate={} frames={} peak={:.6} rms={:.6} ",
                "roughness_proxy={:.6} late_to_early_energy_ratio={:.6} checksum={}"
            ),
            artifact.profile_slug,
            artifact.variant_slug,
            artifact.damage_amount,
            artifact.paths.wav_path,
            artifact.paths.summary_path,
            artifact.summary.sample_rate,
            artifact.summary.frame_count,
            artifact.summary.peak,
            artifact.summary.rms,
            artifact.behavior_metrics.roughness_proxy,
            artifact.behavior_metrics.late_to_early_energy_ratio,
            artifact.summary.checksum,
        );
    }

    Ok(())
}
