use clap::{Parser, ValueEnum};
use corrotion::offline::{analyze_post_chain_aliasing, OfflineRenderer, RenderConfig};
use std::path::PathBuf;

/// Offline render suite selection for deterministic regression assets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Suite {
    /// Render the family comparison set only.
    Family,
    /// Render the rust variation set only.
    Rust,
    /// Render the damage variation set only.
    Damage,
    /// Render all offline comparison suites.
    All,
    /// Measure alias-like residual energy in the nonlinear post chain.
    Aliasing,
}

/// Deterministic offline comparison renderer for production QA artifacts.
#[derive(Parser, Debug)]
#[command(about = "Render deterministic offline comparison suites")]
struct Args {
    /// Which comparison suite to render.
    #[arg(long, value_enum, default_value_t = Suite::All)]
    suite: Suite,

    /// Output directory for rendered WAVs and manifest files.
    #[arg(long, default_value = "output/offline")]
    out_dir: PathBuf,

    /// Sample rate for rendered artifacts.
    #[arg(long, default_value_t = 48_000)]
    sample_rate: u32,

    /// Frame count for each offline render.
    #[arg(long, default_value_t = 48_000)]
    frame_count: usize,

    /// Frame index for the deterministic excitation impulse.
    #[arg(long, default_value_t = 0)]
    excitation_frame: usize,

    /// Impulse amplitude for the deterministic excitation.
    #[arg(long, default_value_t = 1.0)]
    excitation_amplitude: f32,
}

/// Validate CLI input before rendering so release workflows fail clearly.
fn validate_args(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.sample_rate == 0 {
        return Err("sample_rate must be greater than 0".into());
    }

    if args.frame_count == 0 {
        return Err("frame_count must be greater than 0".into());
    }

    if args.excitation_frame >= args.frame_count {
        return Err("excitation_frame must be less than frame_count".into());
    }

    if !args.excitation_amplitude.is_finite() {
        return Err("excitation_amplitude must be finite".into());
    }

    Ok(())
}

/// Render the selected deterministic QA suites and print the generated location.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    validate_args(&args)?;

    let renderer = OfflineRenderer::new(RenderConfig {
        sample_rate: args.sample_rate,
        frame_count: args.frame_count,
        excitation_frame: args.excitation_frame,
        excitation_amplitude: args.excitation_amplitude,
    });

    std::fs::create_dir_all(&args.out_dir)?;

    match args.suite {
        Suite::Family => {
            renderer.render_family_comparisons_to_dir(&args.out_dir)?;
        }
        Suite::Rust => {
            renderer.render_rust_variations_to_dir(&args.out_dir)?;
        }
        Suite::Damage => {
            renderer.render_damage_variations_to_dir(&args.out_dir)?;
        }
        Suite::All => {
            renderer.render_family_comparisons_to_dir(&args.out_dir.join("family"))?;
            renderer.render_rust_variations_to_dir(&args.out_dir.join("rust"))?;
            renderer.render_damage_variations_to_dir(&args.out_dir.join("damage"))?;
            let aliasing_report = analyze_post_chain_aliasing(args.sample_rate, args.frame_count);
            std::fs::write(
                args.out_dir.join("aliasing_report.txt"),
                aliasing_report.to_report(),
            )?;
        }
        Suite::Aliasing => {
            let aliasing_report = analyze_post_chain_aliasing(args.sample_rate, args.frame_count);
            std::fs::write(
                args.out_dir.join("aliasing_report.txt"),
                aliasing_report.to_report(),
            )?;
        }
    }

    println!(
        "Rendered {:?} suite to {}",
        args.suite,
        args.out_dir.display()
    );
    Ok(())
}
