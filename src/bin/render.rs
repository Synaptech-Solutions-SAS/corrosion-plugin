use clap::{Parser, ValueEnum};
use corrotion::offline::RenderConfig;
use corrotion::{dsp::ModalProfileId, offline::OfflineRenderer};
use std::path::Path;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Suite {
    Family,
    Rust,
    Damage,
    All,
}

#[derive(Parser, Debug)]
#[command(about = "Debug-focused offline renderer for comparison suites")]
struct Args {
    #[arg(long, value_enum, default_value = "all")]
    suite: Suite,
    #[arg(long, default_value = "output")]
    out_root: String,
    #[arg(long, default_value_t = 48_000)]
    sample_rate: u32,
    #[arg(long, default_value_t = 1.0)]
    duration: f32,
    #[arg(long, default_value_t = 0)]
    excitation_frame: usize,
    #[arg(long, default_value_t = 1.0)]
    excitation_amplitude: f32,
}

fn object_name(profile_id: ModalProfileId) -> &'static str {
    match profile_id {
        ModalProfileId::Pipe => "Pipe",
        ModalProfileId::Plate => "Plate",
        ModalProfileId::Tank => "Tank",
        ModalProfileId::Chain => "Chain",
    }
}

fn render_config_from_args(args: &Args) -> Result<RenderConfig, Box<dyn std::error::Error>> {
    if args.sample_rate == 0 {
        return Err("sample_rate must be greater than 0".into());
    }
    if !(args.duration.is_finite() && args.duration > 0.0) {
        return Err("duration must be finite and greater than 0".into());
    }

    let frame_count = (args.sample_rate as f32 * args.duration) as usize;
    if frame_count == 0 {
        return Err("duration and sample_rate resulted in zero frames".into());
    }
    if args.excitation_frame >= frame_count {
        return Err(
            format!(
                "excitation_frame ({}) must be smaller than frame_count ({frame_count})",
                args.excitation_frame
            )
            .into(),
        );
    }

    Ok(RenderConfig {
        sample_rate: args.sample_rate,
        frame_count,
        excitation_frame: args.excitation_frame,
        excitation_amplitude: args.excitation_amplitude,
    })
}

fn render_family_suite(
    renderer: &OfflineRenderer,
    out_root: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let output_dir = out_root.join("family-comparisons");
    let artifacts = renderer.render_family_comparisons_to_dir(&output_dir)?;

    for artifact in &artifacts {
        println!(
            concat!(
                "suite=family object={} slug={} wav={} summary={} ",
                "sample_rate={} frames={} peak={:.6} rms={:.6} checksum={}"
            ),
            object_name(artifact.profile_id),
            artifact.slug,
            artifact.paths.wav_path,
            artifact.paths.summary_path,
            artifact.summary.sample_rate,
            artifact.summary.frame_count,
            artifact.summary.peak,
            artifact.summary.rms,
            artifact.summary.checksum,
        );
    }

    Ok(artifacts.len())
}

fn render_rust_suite(
    renderer: &OfflineRenderer,
    out_root: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let output_dir = out_root.join("rust-variations");
    let artifacts = renderer.render_rust_variations_to_dir(&output_dir)?;

    for artifact in &artifacts {
        println!(
            concat!(
                "suite=rust object={} profile={} variant={} rust_amount={:.3} wav={} summary={} ",
                "sample_rate={} frames={} peak={:.6} rms={:.6} brightness={:.6} ",
                "roughness={:.6} late_to_early={:.6} checksum={}"
            ),
            object_name(artifact.profile_id),
            artifact.profile_slug,
            artifact.variant_slug,
            artifact.rust_amount,
            artifact.paths.wav_path,
            artifact.paths.summary_path,
            artifact.summary.sample_rate,
            artifact.summary.frame_count,
            artifact.summary.peak,
            artifact.summary.rms,
            artifact.behavior_metrics.brightness_proxy,
            artifact.behavior_metrics.roughness_proxy,
            artifact.behavior_metrics.late_to_early_energy_ratio,
            artifact.summary.checksum,
        );
    }

    Ok(artifacts.len())
}

fn render_damage_suite(
    renderer: &OfflineRenderer,
    out_root: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let output_dir = out_root.join("damage-variations");
    let artifacts = renderer.render_damage_variations_to_dir(&output_dir)?;

    for artifact in &artifacts {
        println!(
            concat!(
                "suite=damage object={} profile={} variant={} damage_amount={:.3} wav={} summary={} ",
                "sample_rate={} frames={} peak={:.6} rms={:.6} brightness={:.6} ",
                "roughness={:.6} late_to_early={:.6} checksum={}"
            ),
            object_name(artifact.profile_id),
            artifact.profile_slug,
            artifact.variant_slug,
            artifact.damage_amount,
            artifact.paths.wav_path,
            artifact.paths.summary_path,
            artifact.summary.sample_rate,
            artifact.summary.frame_count,
            artifact.summary.peak,
            artifact.summary.rms,
            artifact.behavior_metrics.brightness_proxy,
            artifact.behavior_metrics.roughness_proxy,
            artifact.behavior_metrics.late_to_early_energy_ratio,
            artifact.summary.checksum,
        );
    }

    Ok(artifacts.len())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let out_root = Path::new(&args.out_root);
    let config = render_config_from_args(&args)?;
    let renderer = OfflineRenderer::new(config);

    let total_artifacts = match args.suite {
        Suite::Family => render_family_suite(&renderer, out_root)?,
        Suite::Rust => render_rust_suite(&renderer, out_root)?,
        Suite::Damage => render_damage_suite(&renderer, out_root)?,
        Suite::All => {
            render_family_suite(&renderer, out_root)?
                + render_rust_suite(&renderer, out_root)?
                + render_damage_suite(&renderer, out_root)?
        }
    };

    println!(
        concat!(
            "render_complete suite={:?} out_root={} sample_rate={} ",
            "duration={} frames={} excitation_frame={} excitation_amplitude={} artifacts={}"
        ),
        args.suite,
        out_root.display(),
        config.sample_rate,
        args.duration,
        config.frame_count,
        config.excitation_frame,
        config.excitation_amplitude,
        total_artifacts,
    );

    Ok(())
}
