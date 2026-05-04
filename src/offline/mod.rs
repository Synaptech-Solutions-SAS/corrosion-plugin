use std::fs;
use std::io;
use std::path::Path;

use crate::dsp::{DamageAmount, ModalProfileId, ModalResonator, RustAmount, SizeScale};

#[derive(Clone, Copy, Debug)]
pub struct RenderConfig {
    pub sample_rate: u32,
    pub frame_count: usize,
    pub excitation_frame: usize,
    pub excitation_amplitude: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            frame_count: 48_000,
            excitation_frame: 0,
            excitation_amplitude: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderSummary {
    pub sample_rate: u32,
    pub frame_count: usize,
    pub peak: f32,
    pub rms: f32,
    pub checksum: u64,
    pub first_samples: Vec<f32>,
}

impl RenderSummary {
    pub fn to_report(&self) -> String {
        let first_samples = self
            .first_samples
            .iter()
            .map(|sample| format!("{sample:.9}"))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            concat!(
                "Corrosion offline renderer scaffold\n",
                "sample_rate={sample_rate}\n",
                "frame_count={frame_count}\n",
                "peak={peak:.9}\n",
                "rms={rms:.9}\n",
                "checksum={checksum}\n",
                "first_samples=[{first_samples}]\n"
            ),
            sample_rate = self.sample_rate,
            frame_count = self.frame_count,
            peak = self.peak,
            rms = self.rms,
            checksum = self.checksum,
            first_samples = first_samples,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderBehaviorMetrics {
    pub zero_crossings: usize,
    pub brightness_proxy: f32,
    pub late_to_early_energy_ratio: f32,
    pub roughness_proxy: f32,
}

impl RenderBehaviorMetrics {
    fn to_report(&self) -> String {
        format!(
            concat!(
                "zero_crossings={}\n",
                "brightness_proxy={:.9}\n",
                "late_to_early_energy_ratio={:.9}\n",
                "roughness_proxy={:.9}\n"
            ),
            self.zero_crossings,
            self.brightness_proxy,
            self.late_to_early_energy_ratio,
            self.roughness_proxy,
        )
    }
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComparisonRenderSpec {
    pub profile_id: ModalProfileId,
    pub slug: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComparisonArtifactPaths {
    pub wav_path: String,
    pub summary_path: String,
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonRenderArtifact {
    pub profile_id: ModalProfileId,
    pub slug: &'static str,
    pub paths: ComparisonArtifactPaths,
    pub summary: RenderSummary,
}

#[cfg_attr(not(test), allow(dead_code))]
impl ComparisonRenderArtifact {
    fn summary_report(&self) -> String {
        format!(
            concat!("family={}\n", "wav_path={}\n", "summary_path={}\n", "{}"),
            self.slug,
            self.paths.wav_path,
            self.paths.summary_path,
            self.summary.to_report(),
        )
    }
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RustVariationRenderSpec {
    pub profile_id: ModalProfileId,
    pub profile_slug: &'static str,
    pub variant_slug: &'static str,
    pub rust_amount: f32,
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Debug, PartialEq)]
pub struct RustVariationRenderArtifact {
    pub profile_id: ModalProfileId,
    pub profile_slug: &'static str,
    pub variant_slug: &'static str,
    pub rust_amount: f32,
    pub paths: ComparisonArtifactPaths,
    pub summary: RenderSummary,
    pub behavior_metrics: RenderBehaviorMetrics,
}

#[cfg_attr(not(test), allow(dead_code))]
impl RustVariationRenderArtifact {
    fn summary_report(&self) -> String {
        format!(
            concat!(
                "profile={}\n",
                "variant={}\n",
                "rust_amount={:.3}\n",
                "wav_path={}\n",
                "summary_path={}\n",
                "{}",
                "{}"
            ),
            self.profile_slug,
            self.variant_slug,
            self.rust_amount,
            self.paths.wav_path,
            self.paths.summary_path,
            self.summary.to_report(),
            self.behavior_metrics.to_report(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DamageVariationRenderSpec {
    pub profile_id: ModalProfileId,
    pub profile_slug: &'static str,
    pub variant_slug: &'static str,
    pub damage_amount: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DamageVariationRenderArtifact {
    pub profile_id: ModalProfileId,
    pub profile_slug: &'static str,
    pub variant_slug: &'static str,
    pub damage_amount: f32,
    pub paths: ComparisonArtifactPaths,
    pub summary: RenderSummary,
    pub behavior_metrics: RenderBehaviorMetrics,
}

impl DamageVariationRenderArtifact {
    fn summary_report(&self) -> String {
        format!(
            concat!(
                "profile={}\n",
                "variant={}\n",
                "damage_amount={:.3}\n",
                "wav_path={}\n",
                "summary_path={}\n",
                "{}",
                "{}"
            ),
            self.profile_slug,
            self.variant_slug,
            self.damage_amount,
            self.paths.wav_path,
            self.paths.summary_path,
            self.summary.to_report(),
            self.behavior_metrics.to_report(),
        )
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub const FAMILY_COMPARISON_SPECS: [ComparisonRenderSpec; 3] = [
    ComparisonRenderSpec {
        profile_id: ModalProfileId::Pipe,
        slug: "pipe",
    },
    ComparisonRenderSpec {
        profile_id: ModalProfileId::Plate,
        slug: "plate",
    },
    ComparisonRenderSpec {
        profile_id: ModalProfileId::Tank,
        slug: "tank",
    },
];

#[cfg_attr(not(test), allow(dead_code))]
pub const RUST_VARIATION_SPECS: [RustVariationRenderSpec; 2] = [
    RustVariationRenderSpec {
        profile_id: ModalProfileId::Pipe,
        profile_slug: "pipe",
        variant_slug: "low_rust",
        rust_amount: 0.25,
    },
    RustVariationRenderSpec {
        profile_id: ModalProfileId::Pipe,
        profile_slug: "pipe",
        variant_slug: "high_rust",
        rust_amount: 0.85,
    },
];

pub const DAMAGE_VARIATION_SPECS: [DamageVariationRenderSpec; 2] = [
    DamageVariationRenderSpec {
        profile_id: ModalProfileId::Pipe,
        profile_slug: "pipe",
        variant_slug: "low_damage",
        damage_amount: 0.25,
    },
    DamageVariationRenderSpec {
        profile_id: ModalProfileId::Pipe,
        profile_slug: "pipe",
        variant_slug: "high_damage",
        damage_amount: 0.85,
    },
];

fn energy_in_window(samples: &[f32], start: usize, len: usize) -> f32 {
    samples[start..start + len]
        .iter()
        .map(|sample| sample.abs())
        .sum::<f32>()
}

fn zero_crossings(samples: &[f32]) -> usize {
    samples
        .windows(2)
        .filter(|pair| {
            let first = pair[0];
            let second = pair[1];
            (first > 0.0 && second < 0.0) || (first < 0.0 && second > 0.0)
        })
        .count()
}

fn brightness_proxy(samples: &[f32]) -> f32 {
    let absolute_motion = samples
        .windows(2)
        .map(|pair| (pair[1] - pair[0]).abs())
        .sum::<f32>();
    let absolute_level = samples.iter().map(|sample| sample.abs()).sum::<f32>();

    absolute_motion / absolute_level.max(f32::EPSILON)
}

fn roughness_proxy(samples: &[f32]) -> f32 {
    let crossing_positions = samples
        .windows(2)
        .enumerate()
        .filter_map(|(index, pair)| {
            let first = pair[0];
            let second = pair[1];
            ((first > 0.0 && second < 0.0) || (first < 0.0 && second > 0.0)).then_some(index as f32)
        })
        .collect::<Vec<_>>();

    if crossing_positions.len() < 3 {
        return 0.0;
    }

    let intervals = crossing_positions
        .windows(2)
        .map(|pair| (pair[1] - pair[0]).abs())
        .collect::<Vec<_>>();

    let mean_interval = intervals.iter().sum::<f32>() / intervals.len() as f32;

    intervals
        .iter()
        .map(|interval| (interval - mean_interval).abs())
        .sum::<f32>()
        / intervals.len() as f32
}

pub fn render_behavior_metrics(output: &[f32]) -> RenderBehaviorMetrics {
    let frame_count = output.len();
    let early_len = (frame_count / 4).max(1);
    let late_len = frame_count - early_len;

    RenderBehaviorMetrics {
        zero_crossings: zero_crossings(output),
        brightness_proxy: brightness_proxy(output),
        late_to_early_energy_ratio: energy_in_window(output, early_len, late_len)
            / energy_in_window(output, 0, early_len).max(f32::EPSILON),
        roughness_proxy: roughness_proxy(output),
    }
}

fn float_sample_to_pcm_i16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);
    (clamped * i16::MAX as f32) as i16
}

pub(crate) fn write_wav_i16(path: &Path, samples: &[f32], sample_rate: u32) -> io::Result<()> {
    let data_bytes: Vec<u8> = samples
        .iter()
        .flat_map(|sample| {
            let pcm = float_sample_to_pcm_i16(*sample);
            [(pcm & 0xFF) as u8, ((pcm >> 8) & 0xFF) as u8]
        })
        .collect();

    let data_len = data_bytes.len() as u32;
    let chunk_size = 36 + data_len;
    let byte_rate = sample_rate * 2;

    let mut wav = Vec::with_capacity(44 + data_bytes.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&chunk_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(&data_bytes);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, wav)
}

fn path_display_string(path: &Path) -> String {
    path.display().to_string()
}

pub struct OfflineRenderer {
    config: RenderConfig,
}

impl OfflineRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    pub fn render<R: crate::dsp::ResonatorCore>(
        &self,
        mut resonator: R,
    ) -> (Vec<f32>, RenderSummary) {
        let excitation = self.deterministic_excitation();

        self.render_with_excitation(&excitation, &mut resonator)
    }

    pub fn deterministic_excitation(&self) -> crate::dsp::ExcitationInput {
        crate::dsp::ExcitationInput::impulse(
            self.config.frame_count,
            self.config.excitation_frame,
            self.config.excitation_amplitude,
        )
    }

    pub fn render_with_excitation<R: crate::dsp::ResonatorCore>(
        &self,
        excitation: &crate::dsp::ExcitationInput,
        resonator: &mut R,
    ) -> (Vec<f32>, RenderSummary) {
        assert_eq!(
            excitation.frame_count(),
            self.config.frame_count,
            "excitation length must match render frame count"
        );

        let mut output = Vec::with_capacity(self.config.frame_count);
        let mut peak = 0.0_f32;
        let mut energy_sum = 0.0_f64;
        let mut checksum = 0_u64;

        for frame in 0..self.config.frame_count {
            let sample =
                resonator.process_sample(excitation.sample(frame), self.config.sample_rate);
            peak = peak.max(sample.abs());
            energy_sum += f64::from(sample) * f64::from(sample);
            checksum = checksum
                .wrapping_mul(1_099_511_628_211)
                .wrapping_add(u64::from(sample.to_bits()));
            output.push(sample);
        }

        let first_samples = output.iter().take(16).copied().collect::<Vec<_>>();
        let rms = (energy_sum / self.config.frame_count as f64).sqrt() as f32;

        let summary = RenderSummary {
            sample_rate: self.config.sample_rate,
            frame_count: self.config.frame_count,
            peak,
            rms,
            checksum,
            first_samples,
        };

        (output, summary)
    }

    #[allow(dead_code)]
    pub fn render_to_path<R: crate::dsp::ResonatorCore>(
        &self,
        resonator: R,
        path: &Path,
    ) -> Result<RenderSummary, Box<dyn std::error::Error>> {
        let (_output, summary) = self.render(resonator);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, summary.to_report())?;
        Ok(summary)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn render_family_comparisons_to_dir(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<ComparisonRenderArtifact>, Box<dyn std::error::Error>> {
        fs::create_dir_all(output_dir)?;

        let mut artifacts = Vec::with_capacity(FAMILY_COMPARISON_SPECS.len());

        for spec in FAMILY_COMPARISON_SPECS {
            let (output, summary) =
                self.render(ModalResonator::with_profile(spec.profile_id));
            let wav_path = output_dir.join(format!("{}_comparison.wav", spec.slug));
            let summary_path = output_dir.join(format!("{}_comparison_summary.txt", spec.slug));

            write_wav_i16(&wav_path, &output, self.config.sample_rate)?;

            let artifact = ComparisonRenderArtifact {
                profile_id: spec.profile_id,
                slug: spec.slug,
                paths: ComparisonArtifactPaths {
                    wav_path: path_display_string(&wav_path),
                    summary_path: path_display_string(&summary_path),
                },
                summary,
            };

            fs::write(&summary_path, artifact.summary_report())?;
            artifacts.push(artifact);
        }

        let manifest = artifacts
            .iter()
            .map(|artifact| {
                format!(
                    "family={} wav_path={} summary_path={} checksum={} peak={:.9} rms={:.9}",
                    artifact.slug,
                    artifact.paths.wav_path,
                    artifact.paths.summary_path,
                    artifact.summary.checksum,
                    artifact.summary.peak,
                    artifact.summary.rms,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(output_dir.join("family_comparison_manifest.txt"), manifest)?;

        Ok(artifacts)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn render_rust_variations_to_dir(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<RustVariationRenderArtifact>, Box<dyn std::error::Error>> {
        fs::create_dir_all(output_dir)?;

        let mut artifacts = Vec::with_capacity(RUST_VARIATION_SPECS.len());

        for spec in RUST_VARIATION_SPECS {
            let (output, summary) = self.render(ModalResonator::with_profile_size_and_rust(
                spec.profile_id,
                SizeScale::default(),
                RustAmount::new(spec.rust_amount),
            ));
            let behavior_metrics = render_behavior_metrics(&output);
            let wav_path =
                output_dir.join(format!("{}_{}.wav", spec.profile_slug, spec.variant_slug));
            let summary_path = output_dir.join(format!(
                "{}_{}_summary.txt",
                spec.profile_slug, spec.variant_slug
            ));

            write_wav_i16(&wav_path, &output, self.config.sample_rate)?;

            let artifact = RustVariationRenderArtifact {
                profile_id: spec.profile_id,
                profile_slug: spec.profile_slug,
                variant_slug: spec.variant_slug,
                rust_amount: spec.rust_amount,
                paths: ComparisonArtifactPaths {
                    wav_path: path_display_string(&wav_path),
                    summary_path: path_display_string(&summary_path),
                },
                summary,
                behavior_metrics,
            };

            fs::write(&summary_path, artifact.summary_report())?;
            artifacts.push(artifact);
        }

        let manifest = artifacts
            .iter()
            .map(|artifact| {
                format!(
                    "profile={} variant={} rust={:.2} wav_path={} summary_path={} checksum={} peak={:.9} rms={:.9}",
                    artifact.profile_slug,
                    artifact.variant_slug,
                    artifact.rust_amount,
                    artifact.paths.wav_path,
                    artifact.paths.summary_path,
                    artifact.summary.checksum,
                    artifact.summary.peak,
                    artifact.summary.rms,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(output_dir.join("rust_variation_manifest.txt"), manifest)?;

        Ok(artifacts)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn render_damage_variations_to_dir(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<DamageVariationRenderArtifact>, Box<dyn std::error::Error>> {
        fs::create_dir_all(output_dir)?;

        let mut artifacts = Vec::with_capacity(DAMAGE_VARIATION_SPECS.len());

        for spec in DAMAGE_VARIATION_SPECS {
            let (output, summary) =
                self.render(ModalResonator::with_profile_size_rust_and_damage(
                    spec.profile_id,
                    SizeScale::default(),
                    RustAmount::default(),
                    DamageAmount::new(spec.damage_amount),
                ));
            let behavior_metrics = render_behavior_metrics(&output);
            let wav_path =
                output_dir.join(format!("{}_{}.wav", spec.profile_slug, spec.variant_slug));
            let summary_path = output_dir.join(format!(
                "{}_{}_summary.txt",
                spec.profile_slug, spec.variant_slug
            ));

            write_wav_i16(&wav_path, &output, self.config.sample_rate)?;

            let artifact = DamageVariationRenderArtifact {
                profile_id: spec.profile_id,
                profile_slug: spec.profile_slug,
                variant_slug: spec.variant_slug,
                damage_amount: spec.damage_amount,
                paths: ComparisonArtifactPaths {
                    wav_path: path_display_string(&wav_path),
                    summary_path: path_display_string(&summary_path),
                },
                summary,
                behavior_metrics,
            };

            fs::write(&summary_path, artifact.summary_report())?;
            artifacts.push(artifact);
        }

        let manifest = artifacts
            .iter()
            .map(|artifact| {
                format!(
                    "profile={} variant={} damage={:.2} wav_path={} summary_path={} checksum={} peak={:.9} rms={:.9}",
                    artifact.profile_slug,
                    artifact.variant_slug,
                    artifact.damage_amount,
                    artifact.paths.wav_path,
                    artifact.paths.summary_path,
                    artifact.summary.checksum,
                    artifact.summary.peak,
                    artifact.summary.rms,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(output_dir.join("damage_variation_manifest.txt"), manifest)?;

        Ok(artifacts)
    }
}
