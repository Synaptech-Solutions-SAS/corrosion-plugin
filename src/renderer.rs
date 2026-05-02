use std::f32::consts::PI;
use std::fs;
use std::io;
use std::path::Path;

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
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    let mean_interval = intervals.iter().sum::<f32>() / intervals.len() as f32;
    let interval_deviation = intervals
        .iter()
        .map(|interval| (interval - mean_interval).abs())
        .sum::<f32>();

    interval_deviation / (intervals.len() as f32 * mean_interval.max(f32::EPSILON))
}

fn render_behavior_metrics(output: &[f32]) -> RenderBehaviorMetrics {
    let early_window = &output[512..2_560];
    let bright_window = &output[..2_048];
    let late_energy = energy_in_window(output, 4_096, 2_048);
    let early_energy = energy_in_window(output, 512, 2_048);

    RenderBehaviorMetrics {
        zero_crossings: zero_crossings(early_window),
        brightness_proxy: brightness_proxy(bright_window),
        late_to_early_energy_ratio: late_energy / early_energy,
        roughness_proxy: roughness_proxy(early_window),
    }
}

fn float_sample_to_pcm_i16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);
    if clamped >= 0.0 {
        (clamped * i16::MAX as f32).round() as i16
    } else {
        (clamped * -(i16::MIN as f32)).round() as i16
    }
}

fn write_wav_i16(path: &Path, samples: &[f32], sample_rate: u32) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let sample_count = u32::try_from(samples.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "too many samples for WAV"))?;
    let bytes_per_sample = 2_u32;
    let data_size = sample_count
        .checked_mul(bytes_per_sample)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "WAV data too large"))?;
    let riff_chunk_size = 36_u32
        .checked_add(data_size)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "WAV file too large"))?;
    let byte_rate = sample_rate
        .checked_mul(bytes_per_sample)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid WAV byte rate"))?;

    let mut wav_bytes = Vec::with_capacity(44 + samples.len() * bytes_per_sample as usize);
    wav_bytes.extend_from_slice(b"RIFF");
    wav_bytes.extend_from_slice(&riff_chunk_size.to_le_bytes());
    wav_bytes.extend_from_slice(b"WAVE");
    wav_bytes.extend_from_slice(b"fmt ");
    wav_bytes.extend_from_slice(&16_u32.to_le_bytes());
    wav_bytes.extend_from_slice(&1_u16.to_le_bytes());
    wav_bytes.extend_from_slice(&1_u16.to_le_bytes());
    wav_bytes.extend_from_slice(&sample_rate.to_le_bytes());
    wav_bytes.extend_from_slice(&byte_rate.to_le_bytes());
    wav_bytes.extend_from_slice(&2_u16.to_le_bytes());
    wav_bytes.extend_from_slice(&16_u16.to_le_bytes());
    wav_bytes.extend_from_slice(b"data");
    wav_bytes.extend_from_slice(&data_size.to_le_bytes());

    for sample in samples {
        wav_bytes.extend_from_slice(&float_sample_to_pcm_i16(*sample).to_le_bytes());
    }

    fs::write(path, wav_bytes)
}

fn path_display_string(path: &Path) -> String {
    path.display().to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExcitationInput {
    samples: Vec<f32>,
}

impl ExcitationInput {
    pub fn impulse(frame_count: usize, excitation_frame: usize, amplitude: f32) -> Self {
        let mut samples = vec![0.0; frame_count];

        if let Some(sample) = samples.get_mut(excitation_frame) {
            *sample = amplitude;
        }

        Self { samples }
    }

    pub fn frame_count(&self) -> usize {
        self.samples.len()
    }

    pub fn sample(&self, frame: usize) -> f32 {
        self.samples[frame]
    }

    #[cfg(test)]
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}

pub trait ResonatorCore {
    fn process_sample(&mut self, excitation: f32, sample_rate: u32) -> f32;
}

pub struct OfflineRenderer {
    config: RenderConfig,
}

impl OfflineRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    pub fn render<R: ResonatorCore>(&self, mut resonator: R) -> (Vec<f32>, RenderSummary) {
        let excitation = self.deterministic_excitation();

        self.render_with_excitation(&excitation, &mut resonator)
    }

    pub fn deterministic_excitation(&self) -> ExcitationInput {
        ExcitationInput::impulse(
            self.config.frame_count,
            self.config.excitation_frame,
            self.config.excitation_amplitude,
        )
    }

    pub fn render_with_excitation<R: ResonatorCore>(
        &self,
        excitation: &ExcitationInput,
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
    pub fn render_to_path<R: ResonatorCore>(
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
                self.render(PlaceholderResonator::with_profile(spec.profile_id));
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
            let (output, summary) = self.render(PlaceholderResonator::with_profile_size_and_rust(
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
                    concat!(
                        "profile={} ",
                        "variant={} ",
                        "rust_amount={:.3} ",
                        "wav_path={} ",
                        "summary_path={} ",
                        "checksum={} ",
                        "peak={:.9} ",
                        "rms={:.9} ",
                        "brightness_proxy={:.9} ",
                        "late_to_early_energy_ratio={:.9}"
                    ),
                    artifact.profile_slug,
                    artifact.variant_slug,
                    artifact.rust_amount,
                    artifact.paths.wav_path,
                    artifact.paths.summary_path,
                    artifact.summary.checksum,
                    artifact.summary.peak,
                    artifact.summary.rms,
                    artifact.behavior_metrics.brightness_proxy,
                    artifact.behavior_metrics.late_to_early_energy_ratio,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(output_dir.join("rust_variation_manifest.txt"), manifest)?;

        Ok(artifacts)
    }

    pub fn render_damage_variations_to_dir(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<DamageVariationRenderArtifact>, Box<dyn std::error::Error>> {
        fs::create_dir_all(output_dir)?;

        let mut artifacts = Vec::with_capacity(DAMAGE_VARIATION_SPECS.len());

        for spec in DAMAGE_VARIATION_SPECS {
            let (output, summary) =
                self.render(PlaceholderResonator::with_profile_size_rust_and_damage(
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
                    concat!(
                        "profile={} ",
                        "variant={} ",
                        "damage_amount={:.3} ",
                        "wav_path={} ",
                        "summary_path={} ",
                        "checksum={} ",
                        "peak={:.9} ",
                        "rms={:.9} ",
                        "roughness_proxy={:.9} ",
                        "late_to_early_energy_ratio={:.9}"
                    ),
                    artifact.profile_slug,
                    artifact.variant_slug,
                    artifact.damage_amount,
                    artifact.paths.wav_path,
                    artifact.paths.summary_path,
                    artifact.summary.checksum,
                    artifact.summary.peak,
                    artifact.summary.rms,
                    artifact.behavior_metrics.roughness_proxy,
                    artifact.behavior_metrics.late_to_early_energy_ratio,
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(output_dir.join("damage_variation_manifest.txt"), manifest)?;

        Ok(artifacts)
    }
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalProfileId {
    Pipe,
    Plate,
    Tank,
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimeModeCountEstimate {
    pub profile_id: ModalProfileId,
    pub canonical_mode_count: usize,
    pub safe_realtime_mode_count: usize,
    pub offline_peak_mode_count: usize,
}

impl RealtimeModeCountEstimate {
    #[cfg_attr(not(test), allow(dead_code))]
    fn for_profile(profile_id: ModalProfileId) -> Self {
        let profile = ModalProfile::from_id(profile_id);
        let canonical_mode_count = profile.modes.len();
        let offline_peak_mode_count = profile
            .scaled_mode_specs(
                SizeScale::default(),
                RustAmount::default(),
                DamageAmount::new(1.0),
            )
            .len();

        Self {
            profile_id,
            canonical_mode_count,
            safe_realtime_mode_count: canonical_mode_count,
            offline_peak_mode_count,
        }
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn realtime_mode_count_estimate(profile_id: ModalProfileId) -> RealtimeModeCountEstimate {
    RealtimeModeCountEstimate::for_profile(profile_id)
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn realtime_mode_count_estimates() -> [RealtimeModeCountEstimate; 3] {
    [
        RealtimeModeCountEstimate::for_profile(ModalProfileId::Pipe),
        RealtimeModeCountEstimate::for_profile(ModalProfileId::Plate),
        RealtimeModeCountEstimate::for_profile(ModalProfileId::Tank),
    ]
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn safe_realtime_shared_mode_limit() -> usize {
    realtime_mode_count_estimates()
        .into_iter()
        .map(|estimate| estimate.safe_realtime_mode_count)
        .max()
        .unwrap_or(0)
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn offline_peak_shared_mode_limit() -> usize {
    realtime_mode_count_estimates()
        .into_iter()
        .map(|estimate| estimate.offline_peak_mode_count)
        .max()
        .unwrap_or(0)
}

#[derive(Clone, Copy, Debug)]
struct ModalProfile {
    id: ModalProfileId,
    modes: &'static [ModalModeSpec],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeScale {
    factor: f32,
}

impl SizeScale {
    const NEUTRAL: Self = Self { factor: 1.0 };
    const MIN_FACTOR: f32 = 0.25;

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn new(factor: f32) -> Self {
        let sanitized = if factor.is_finite() {
            factor.max(Self::MIN_FACTOR)
        } else {
            Self::NEUTRAL.factor
        };

        Self { factor: sanitized }
    }

    fn factor(self) -> f32 {
        self.factor
    }
}

impl Default for SizeScale {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RustAmount {
    amount: f32,
}

impl RustAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for RustAmount {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DamageAmount {
    amount: f32,
}

impl DamageAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for DamageAmount {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

const PIPE_MODAL_PROFILE_MODES: [ModalModeSpec; 6] = [
    ModalModeSpec::new(220.0, 2.05, 0.0152),
    ModalModeSpec::new(439.5, 1.72, 0.0135),
    ModalModeSpec::new(660.0, 1.36, 0.0112),
    ModalModeSpec::new(881.0, 1.05, 0.0088),
    ModalModeSpec::new(1_103.0, 0.81, 0.0066),
    ModalModeSpec::new(1_327.0, 0.62, 0.0048),
];

#[cfg_attr(not(test), allow(dead_code))]
const PLATE_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(286.0, 0.94, 0.0092),
    ModalModeSpec::new(463.0, 0.82, 0.0089),
    ModalModeSpec::new(731.0, 0.72, 0.0084),
    ModalModeSpec::new(1_036.0, 0.61, 0.0076),
    ModalModeSpec::new(1_394.0, 0.52, 0.0068),
    ModalModeSpec::new(1_811.0, 0.44, 0.0059),
    ModalModeSpec::new(2_297.0, 0.37, 0.0050),
    ModalModeSpec::new(2_860.0, 0.31, 0.0042),
];

#[cfg_attr(not(test), allow(dead_code))]
const TANK_MODAL_PROFILE_MODES: [ModalModeSpec; 8] = [
    ModalModeSpec::new(96.0, 2.90, 0.0260),
    ModalModeSpec::new(151.0, 2.55, 0.0218),
    ModalModeSpec::new(226.0, 2.22, 0.0178),
    ModalModeSpec::new(318.0, 1.90, 0.0139),
    ModalModeSpec::new(439.0, 1.56, 0.0104),
    ModalModeSpec::new(588.0, 1.22, 0.0077),
    ModalModeSpec::new(774.0, 0.94, 0.0056),
    ModalModeSpec::new(1_002.0, 0.72, 0.0040),
];

impl ModalProfile {
    #[cfg_attr(not(test), allow(dead_code))]
    const fn from_id(id: ModalProfileId) -> Self {
        match id {
            ModalProfileId::Pipe => Self::pipe(),
            ModalProfileId::Plate => Self::plate(),
            ModalProfileId::Tank => Self::tank(),
        }
    }

    const fn pipe() -> Self {
        Self {
            id: ModalProfileId::Pipe,
            modes: &PIPE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    const fn plate() -> Self {
        Self {
            id: ModalProfileId::Plate,
            modes: &PLATE_MODAL_PROFILE_MODES,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    const fn tank() -> Self {
        Self {
            id: ModalProfileId::Tank,
            modes: &TANK_MODAL_PROFILE_MODES,
        }
    }

    fn scaled_mode_specs(
        self,
        size_scale: SizeScale,
        rust_amount: RustAmount,
        damage_amount: DamageAmount,
    ) -> Vec<ModalModeSpec> {
        self.modes
            .iter()
            .enumerate()
            .flat_map(|(index, mode)| {
                mode.scaled_for_size(size_scale, index, self.modes.len())
                    .corroded(rust_amount, index, self.modes.len())
                    .damaged(damage_amount, index, self.modes.len())
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct PlaceholderResonator {
    profile: ModalProfileId,
    #[cfg_attr(not(test), allow(dead_code))]
    size_scale: SizeScale,
    #[cfg_attr(not(test), allow(dead_code))]
    rust_amount: RustAmount,
    #[cfg_attr(not(test), allow(dead_code))]
    damage_amount: DamageAmount,
    modes: Vec<SecondOrderMode>,
}

impl PlaceholderResonator {
    fn from_profile(
        profile: ModalProfile,
        size_scale: SizeScale,
        rust_amount: RustAmount,
        damage_amount: DamageAmount,
    ) -> Self {
        Self {
            profile: profile.id,
            size_scale,
            rust_amount,
            damage_amount,
            modes: profile
                .scaled_mode_specs(size_scale, rust_amount, damage_amount)
                .iter()
                .copied()
                .map(SecondOrderMode::new)
                .collect(),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn with_profile(profile_id: ModalProfileId) -> Self {
        Self::from_profile(
            ModalProfile::from_id(profile_id),
            SizeScale::default(),
            RustAmount::default(),
            DamageAmount::default(),
        )
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn with_profile_and_size(profile_id: ModalProfileId, size_scale: SizeScale) -> Self {
        Self::from_profile(
            ModalProfile::from_id(profile_id),
            size_scale,
            RustAmount::default(),
            DamageAmount::default(),
        )
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn with_profile_size_and_rust(
        profile_id: ModalProfileId,
        size_scale: SizeScale,
        rust_amount: RustAmount,
    ) -> Self {
        Self::from_profile(
            ModalProfile::from_id(profile_id),
            size_scale,
            rust_amount,
            DamageAmount::default(),
        )
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn with_profile_size_rust_and_damage(
        profile_id: ModalProfileId,
        size_scale: SizeScale,
        rust_amount: RustAmount,
        damage_amount: DamageAmount,
    ) -> Self {
        Self::from_profile(
            ModalProfile::from_id(profile_id),
            size_scale,
            rust_amount,
            damage_amount,
        )
    }
}

impl Default for PlaceholderResonator {
    fn default() -> Self {
        Self::from_profile(
            ModalProfile::pipe(),
            SizeScale::default(),
            RustAmount::default(),
            DamageAmount::default(),
        )
    }
}

impl ResonatorCore for PlaceholderResonator {
    fn process_sample(&mut self, excitation: f32, sample_rate: u32) -> f32 {
        let mode_sum = self
            .modes
            .iter_mut()
            .map(|mode| mode.process(excitation, sample_rate))
            .sum::<f32>();

        match self.profile {
            ModalProfileId::Pipe => mode_sum,
            ModalProfileId::Plate => mode_sum,
            ModalProfileId::Tank => mode_sum,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ModalModeSpec {
    frequency_hz: f32,
    decay_seconds: f32,
    gain: f32,
}

impl ModalModeSpec {
    const fn new(frequency_hz: f32, decay_seconds: f32, gain: f32) -> Self {
        Self {
            frequency_hz,
            decay_seconds,
            gain,
        }
    }

    fn scaled_for_size(self, size_scale: SizeScale, mode_index: usize, mode_count: usize) -> Self {
        let scale = size_scale.factor();
        let low_mode_weight = if mode_count <= 1 {
            1.0
        } else {
            1.0 - (mode_index as f32 / (mode_count - 1) as f32)
        };
        let resonance_tilt = 1.0 + (scale - 1.0) * 0.35 * low_mode_weight;

        Self {
            frequency_hz: self.frequency_hz / scale,
            decay_seconds: self.decay_seconds * scale,
            gain: (self.gain * resonance_tilt).max(f32::EPSILON),
        }
    }

    fn corroded(self, rust_amount: RustAmount, mode_index: usize, mode_count: usize) -> Self {
        let rust = rust_amount.amount();
        let high_mode_weight = if mode_count <= 1 {
            0.0
        } else {
            mode_index as f32 / (mode_count - 1) as f32
        };
        let brightness_loss = 1.0 - rust * (0.20 + 0.65 * high_mode_weight);
        let decay_loss = 1.0 - rust * (0.30 + 0.45 * high_mode_weight);

        Self {
            frequency_hz: self.frequency_hz,
            decay_seconds: (self.decay_seconds * decay_loss).max(f32::EPSILON),
            gain: (self.gain * brightness_loss).max(f32::EPSILON),
        }
    }

    fn damaged(
        self,
        damage_amount: DamageAmount,
        mode_index: usize,
        mode_count: usize,
    ) -> Vec<Self> {
        let damage = damage_amount.amount();
        if damage <= 0.0 {
            return vec![self];
        }

        let mode_position = if mode_count <= 1 {
            0.0
        } else {
            mode_index as f32 / (mode_count - 1) as f32
        };
        let low_mode_weight = 1.0 - mode_position;
        let detune_direction = if mode_index % 2 == 0 { -1.0 } else { 1.0 };
        let primary_detune = damage * (0.004 + 0.018 * mode_position);
        let companion_detune = damage * (0.010 + 0.030 * mode_position);
        let primary_decay_tilt = 1.0 - damage * (0.10 + 0.18 * low_mode_weight);
        let primary_gain_tilt = 1.0 + damage * (0.05 + 0.12 * mode_position);
        let companion_gain = self.gain * damage * (0.16 + 0.20 * mode_position);
        let companion_decay =
            self.decay_seconds * (0.28 + 0.18 * low_mode_weight + 0.14 * (1.0 - damage));

        let primary = Self {
            frequency_hz: (self.frequency_hz * (1.0 + primary_detune * detune_direction))
                .max(f32::EPSILON),
            decay_seconds: (self.decay_seconds * primary_decay_tilt).max(f32::EPSILON),
            gain: (self.gain * primary_gain_tilt).max(f32::EPSILON),
        };
        let companion = Self {
            frequency_hz: (self.frequency_hz * (1.0 - companion_detune * detune_direction))
                .max(f32::EPSILON),
            decay_seconds: companion_decay.max(f32::EPSILON),
            gain: companion_gain.max(f32::EPSILON),
        };

        vec![primary, companion]
    }
}

#[derive(Clone, Copy, Debug)]
struct ResonatorCoefficients {
    b0: f32,
    a1: f32,
    a2: f32,
}

impl ResonatorCoefficients {
    fn for_mode(mode: ModalModeSpec, sample_rate: u32) -> Self {
        let safe_sample_rate = sample_rate.max(1) as f32;
        let decay_seconds = mode.decay_seconds.max(f32::EPSILON);
        let omega = 2.0 * PI * mode.frequency_hz / safe_sample_rate;
        let r = (-1.0 / (decay_seconds * safe_sample_rate)).exp();

        Self {
            b0: mode.gain,
            a1: -2.0 * r * omega.cos(),
            a2: r * r,
        }
    }
}

#[derive(Clone, Debug)]
struct SecondOrderMode {
    spec: ModalModeSpec,
    coefficients: ResonatorCoefficients,
    cached_sample_rate: Option<u32>,
    y1: f32,
    y2: f32,
}

impl SecondOrderMode {
    fn new(spec: ModalModeSpec) -> Self {
        Self {
            spec,
            coefficients: ResonatorCoefficients::for_mode(spec, 48_000),
            cached_sample_rate: None,
            y1: 0.0,
            y2: 0.0,
        }
    }

    fn process(&mut self, excitation: f32, sample_rate: u32) -> f32 {
        if self.cached_sample_rate != Some(sample_rate) {
            self.coefficients = ResonatorCoefficients::for_mode(self.spec, sample_rate);
            self.cached_sample_rate = Some(sample_rate);
        }

        let sample = self.coefficients.b0 * excitation
            - self.coefficients.a1 * self.y1
            - self.coefficients.a2 * self.y2;

        self.y2 = self.y1;
        self.y1 = sample;
        sample
    }
}

#[cfg(test)]
mod tests {
    use super::{
        offline_peak_shared_mode_limit, realtime_mode_count_estimate, render_behavior_metrics,
        safe_realtime_shared_mode_limit, write_wav_i16, DamageAmount, DamageVariationRenderSpec,
        ExcitationInput, ModalModeSpec, ModalProfile, ModalProfileId, OfflineRenderer,
        PlaceholderResonator, RealtimeModeCountEstimate, RenderConfig, ResonatorCoefficients,
        RustAmount, RustVariationRenderSpec, SecondOrderMode, SizeScale, DAMAGE_VARIATION_SPECS,
        FAMILY_COMPARISON_SPECS, PIPE_MODAL_PROFILE_MODES, PLATE_MODAL_PROFILE_MODES,
        RUST_VARIATION_SPECS, TANK_MODAL_PROFILE_MODES,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    struct ProfileMetrics {
        weighted_frequency: f32,
        average_decay: f32,
        low_band_gain_ratio: f32,
        harmonic_deviation: f32,
    }

    fn approx_eq(left: f32, right: f32, epsilon: f32) {
        assert!(
            (left - right).abs() <= epsilon,
            "left={left} right={right} epsilon={epsilon}"
        );
    }

    fn profile_metrics(profile: ModalProfile) -> ProfileMetrics {
        profile_metrics_for_modes(profile.modes)
    }

    fn profile_metrics_for_modes(modes: &[ModalModeSpec]) -> ProfileMetrics {
        let weighted_frequency_sum = modes
            .iter()
            .map(|mode| mode.frequency_hz * mode.gain)
            .sum::<f32>();
        let total_gain = modes.iter().map(|mode| mode.gain).sum::<f32>();
        let average_decay =
            modes.iter().map(|mode| mode.decay_seconds).sum::<f32>() / modes.len() as f32;
        let low_band_gain_ratio =
            modes.iter().take(3).map(|mode| mode.gain).sum::<f32>() / total_gain;
        let fundamental = modes[0].frequency_hz;
        let harmonic_deviation = modes
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, mode)| {
                let harmonic_number = (index + 1) as f32;
                ((mode.frequency_hz / fundamental) - harmonic_number).abs()
            })
            .sum::<f32>()
            / (modes.len() - 1) as f32;

        ProfileMetrics {
            weighted_frequency: weighted_frequency_sum / total_gain,
            average_decay,
            low_band_gain_ratio,
            harmonic_deviation,
        }
    }

    fn unique_test_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("corrotion_{name}_{nanos}"))
    }

    fn assert_size_scaling_direction(profile_id: ModalProfileId) {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });
        let base_profile = ModalProfile::from_id(profile_id);
        let smaller_scale = SizeScale::new(0.7);
        let larger_scale = SizeScale::new(1.6);

        let smaller_profile_metrics = profile_metrics_for_modes(&base_profile.scaled_mode_specs(
            smaller_scale,
            RustAmount::default(),
            DamageAmount::default(),
        ));
        let larger_profile_metrics = profile_metrics_for_modes(&base_profile.scaled_mode_specs(
            larger_scale,
            RustAmount::default(),
            DamageAmount::default(),
        ));

        let (smaller_output, smaller_summary) = renderer.render(
            PlaceholderResonator::with_profile_and_size(profile_id, smaller_scale),
        );
        let (larger_output, larger_summary) = renderer.render(
            PlaceholderResonator::with_profile_and_size(profile_id, larger_scale),
        );

        let smaller_render_metrics = render_behavior_metrics(&smaller_output);
        let larger_render_metrics = render_behavior_metrics(&larger_output);

        assert!(
            smaller_profile_metrics.weighted_frequency > larger_profile_metrics.weighted_frequency
        );
        assert!(larger_profile_metrics.average_decay > smaller_profile_metrics.average_decay);
        assert!(
            larger_profile_metrics.low_band_gain_ratio
                > smaller_profile_metrics.low_band_gain_ratio
        );
        approx_eq(
            smaller_profile_metrics.harmonic_deviation,
            larger_profile_metrics.harmonic_deviation,
            1.0e-5,
        );

        assert_ne!(smaller_summary.checksum, larger_summary.checksum);
        assert!(smaller_render_metrics.zero_crossings > larger_render_metrics.zero_crossings);
        assert!(smaller_render_metrics.brightness_proxy > larger_render_metrics.brightness_proxy);
        assert!(
            larger_render_metrics.late_to_early_energy_ratio
                > smaller_render_metrics.late_to_early_energy_ratio
        );
    }

    #[test]
    fn render_is_deterministic_for_same_configuration() {
        let renderer = OfflineRenderer::new(RenderConfig::default());

        let (_, first_summary) = renderer.render(PlaceholderResonator::default());
        let (_, second_summary) = renderer.render(PlaceholderResonator::default());

        assert_eq!(first_summary, second_summary);
    }

    #[test]
    fn deterministic_excitation_is_explicit_and_repeatable() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8,
            excitation_frame: 3,
            excitation_amplitude: 0.75,
            ..RenderConfig::default()
        });

        let first_excitation = renderer.deterministic_excitation();
        let second_excitation = renderer.deterministic_excitation();

        assert_eq!(first_excitation, second_excitation);
        assert_eq!(first_excitation, ExcitationInput::impulse(8, 3, 0.75),);
        assert_eq!(
            first_excitation.samples(),
            &[0.0, 0.0, 0.0, 0.75, 0.0, 0.0, 0.0, 0.0],
        );
    }

    #[test]
    fn render_with_explicit_excitation_matches_default_render_path() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 512,
            excitation_frame: 4,
            excitation_amplitude: 0.5,
            ..RenderConfig::default()
        });

        let excitation = renderer.deterministic_excitation();
        let (_, implicit_summary) = renderer.render(PlaceholderResonator::default());
        let (_, explicit_summary) =
            renderer.render_with_excitation(&excitation, &mut PlaceholderResonator::default());

        assert_eq!(implicit_summary, explicit_summary);
    }

    #[test]
    fn family_comparison_specs_cover_pipe_plate_and_tank_in_order() {
        assert_eq!(
            FAMILY_COMPARISON_SPECS,
            [
                super::ComparisonRenderSpec {
                    profile_id: ModalProfileId::Pipe,
                    slug: "pipe",
                },
                super::ComparisonRenderSpec {
                    profile_id: ModalProfileId::Plate,
                    slug: "plate",
                },
                super::ComparisonRenderSpec {
                    profile_id: ModalProfileId::Tank,
                    slug: "tank",
                },
            ]
        );
    }

    #[test]
    fn wav_writer_emits_pcm_header_and_payload() {
        let output_dir = unique_test_dir("wav_writer");
        let wav_path = output_dir.join("test.wav");
        let samples = [1.0_f32, -1.0, 0.0, 0.5];

        write_wav_i16(&wav_path, &samples, 48_000).expect("wav write should succeed");

        let wav_bytes = fs::read(&wav_path).expect("wav should exist");
        assert_eq!(&wav_bytes[0..4], b"RIFF");
        assert_eq!(&wav_bytes[8..12], b"WAVE");
        assert_eq!(&wav_bytes[12..16], b"fmt ");
        assert_eq!(&wav_bytes[36..40], b"data");
        assert_eq!(
            u32::from_le_bytes(wav_bytes[24..28].try_into().unwrap()),
            48_000
        );
        assert_eq!(
            u16::from_le_bytes(wav_bytes[34..36].try_into().unwrap()),
            16
        );
        assert_eq!(u32::from_le_bytes(wav_bytes[40..44].try_into().unwrap()), 8);

        fs::remove_dir_all(output_dir).expect("temp output cleanup should succeed");
    }

    #[test]
    fn family_comparison_render_writes_expected_artifacts() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 4_096,
            ..RenderConfig::default()
        });
        let output_dir = unique_test_dir("family_comparison");

        let artifacts = renderer
            .render_family_comparisons_to_dir(&output_dir)
            .expect("family comparison render should succeed");

        assert_eq!(artifacts.len(), FAMILY_COMPARISON_SPECS.len());

        for (artifact, spec) in artifacts.iter().zip(FAMILY_COMPARISON_SPECS.iter()) {
            assert_eq!(artifact.profile_id, spec.profile_id);
            assert_eq!(artifact.slug, spec.slug);

            let wav_path = output_dir.join(format!("{}_comparison.wav", spec.slug));
            let summary_path = output_dir.join(format!("{}_comparison_summary.txt", spec.slug));

            assert_eq!(artifact.paths.wav_path, wav_path.display().to_string());
            assert_eq!(
                artifact.paths.summary_path,
                summary_path.display().to_string()
            );
            assert!(wav_path.exists());
            assert!(summary_path.exists());

            let summary_report =
                fs::read_to_string(&summary_path).expect("comparison summary should be readable");
            assert!(summary_report.contains(&format!("family={}", spec.slug)));
            assert!(summary_report.contains("Corrosion offline renderer scaffold"));
        }

        let manifest = fs::read_to_string(output_dir.join("family_comparison_manifest.txt"))
            .expect("manifest should be readable");
        assert!(manifest.contains("family=pipe"));
        assert!(manifest.contains("family=plate"));
        assert!(manifest.contains("family=tank"));

        fs::remove_dir_all(output_dir).expect("temp output cleanup should succeed");
    }

    #[test]
    fn rust_variation_specs_cover_low_and_high_pipe_examples_in_order() {
        assert_eq!(
            RUST_VARIATION_SPECS,
            [
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
            ]
        );
    }

    #[test]
    fn rust_variation_render_writes_expected_artifacts() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });
        let output_dir = unique_test_dir("rust_variation");

        let artifacts = renderer
            .render_rust_variations_to_dir(&output_dir)
            .expect("rust variation render should succeed");

        assert_eq!(artifacts.len(), RUST_VARIATION_SPECS.len());

        for (artifact, spec) in artifacts.iter().zip(RUST_VARIATION_SPECS.iter()) {
            assert_eq!(artifact.profile_id, spec.profile_id);
            assert_eq!(artifact.profile_slug, spec.profile_slug);
            assert_eq!(artifact.variant_slug, spec.variant_slug);
            approx_eq(artifact.rust_amount, spec.rust_amount, 1.0e-6);

            let wav_path =
                output_dir.join(format!("{}_{}.wav", spec.profile_slug, spec.variant_slug));
            let summary_path = output_dir.join(format!(
                "{}_{}_summary.txt",
                spec.profile_slug, spec.variant_slug
            ));

            assert_eq!(artifact.paths.wav_path, wav_path.display().to_string());
            assert_eq!(
                artifact.paths.summary_path,
                summary_path.display().to_string()
            );
            assert!(wav_path.exists());
            assert!(summary_path.exists());

            let summary_report =
                fs::read_to_string(&summary_path).expect("rust summary should be readable");
            assert!(summary_report.contains(&format!("profile={}", spec.profile_slug)));
            assert!(summary_report.contains(&format!("variant={}", spec.variant_slug)));
            assert!(summary_report.contains(&format!("rust_amount={:.3}", spec.rust_amount)));
            assert!(summary_report.contains("brightness_proxy="));
            assert!(summary_report.contains("late_to_early_energy_ratio="));
        }

        let manifest = fs::read_to_string(output_dir.join("rust_variation_manifest.txt"))
            .expect("rust manifest should be readable");
        assert!(manifest.contains("variant=low_rust"));
        assert!(manifest.contains("variant=high_rust"));

        fs::remove_dir_all(output_dir).expect("temp output cleanup should succeed");
    }

    #[test]
    fn damage_variation_specs_cover_low_and_high_pipe_examples_in_order() {
        assert_eq!(
            DAMAGE_VARIATION_SPECS,
            [
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
            ]
        );
    }

    #[test]
    fn damage_variation_render_writes_expected_artifacts() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });
        let output_dir = unique_test_dir("damage_variation");

        let artifacts = renderer
            .render_damage_variations_to_dir(&output_dir)
            .expect("damage variation render should succeed");

        assert_eq!(artifacts.len(), DAMAGE_VARIATION_SPECS.len());

        for (artifact, spec) in artifacts.iter().zip(DAMAGE_VARIATION_SPECS.iter()) {
            assert_eq!(artifact.profile_id, spec.profile_id);
            assert_eq!(artifact.profile_slug, spec.profile_slug);
            assert_eq!(artifact.variant_slug, spec.variant_slug);
            approx_eq(artifact.damage_amount, spec.damage_amount, 1.0e-6);

            let wav_path =
                output_dir.join(format!("{}_{}.wav", spec.profile_slug, spec.variant_slug));
            let summary_path = output_dir.join(format!(
                "{}_{}_summary.txt",
                spec.profile_slug, spec.variant_slug
            ));

            assert_eq!(artifact.paths.wav_path, wav_path.display().to_string());
            assert_eq!(
                artifact.paths.summary_path,
                summary_path.display().to_string()
            );
            assert!(wav_path.exists());
            assert!(summary_path.exists());

            let summary_report =
                fs::read_to_string(&summary_path).expect("damage summary should be readable");
            assert!(summary_report.contains(&format!("profile={}", spec.profile_slug)));
            assert!(summary_report.contains(&format!("variant={}", spec.variant_slug)));
            assert!(summary_report.contains(&format!("damage_amount={:.3}", spec.damage_amount)));
            assert!(summary_report.contains("roughness_proxy="));
            assert!(summary_report.contains("late_to_early_energy_ratio="));
        }

        let manifest = fs::read_to_string(output_dir.join("damage_variation_manifest.txt"))
            .expect("damage manifest should be readable");
        assert!(manifest.contains("variant=low_damage"));
        assert!(manifest.contains("variant=high_damage"));

        fs::remove_dir_all(output_dir).expect("temp output cleanup should succeed");
    }

    #[test]
    fn render_produces_non_silent_decaying_output() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 4_096,
            ..RenderConfig::default()
        });

        let (output, summary) = renderer.render(PlaceholderResonator::default());
        let first_window_energy = output
            .iter()
            .take(512)
            .map(|sample| sample.abs())
            .sum::<f32>();
        let tail_window_energy = output
            .iter()
            .rev()
            .take(512)
            .map(|sample| sample.abs())
            .sum::<f32>();

        assert!(summary.peak > 0.0);
        assert!(summary.rms > 0.0);
        assert!(first_window_energy > tail_window_energy);
    }

    #[test]
    fn rust_amount_clamps_to_supported_range() {
        assert_eq!(RustAmount::new(-0.25), RustAmount::default());
        assert_eq!(RustAmount::new(1.5), RustAmount::new(1.0));
        assert_eq!(RustAmount::new(f32::NAN), RustAmount::default());
    }

    #[test]
    fn damage_amount_clamps_to_supported_range() {
        assert_eq!(DamageAmount::new(-0.25), DamageAmount::default());
        assert_eq!(DamageAmount::new(1.5), DamageAmount::new(1.0));
        assert_eq!(DamageAmount::new(f32::NAN), DamageAmount::default());
    }

    #[test]
    fn default_resonator_uses_explicit_pipe_profile() {
        let resonator = PlaceholderResonator::default();

        assert_eq!(resonator.profile, ModalProfileId::Pipe);
        assert_eq!(resonator.size_scale, SizeScale::default());
        assert_eq!(resonator.rust_amount, RustAmount::default());
        assert_eq!(resonator.damage_amount, DamageAmount::default());
        assert_eq!(resonator.modes.len(), PIPE_MODAL_PROFILE_MODES.len());

        let first_mode = resonator.modes.first().expect("pipe profile has modes");
        let last_mode = resonator.modes.last().expect("pipe profile has modes");

        approx_eq(first_mode.spec.frequency_hz, 220.0, 1.0e-6);
        assert!(first_mode.spec.decay_seconds > last_mode.spec.decay_seconds);
        assert!(first_mode.spec.gain > last_mode.spec.gain);
    }

    #[test]
    fn pipe_profile_is_curated_for_tubular_ring_direction() {
        let profile = ModalProfile::pipe();
        let mut previous_frequency = 0.0;
        let mut previous_gain = f32::INFINITY;
        let fundamental = profile.modes[0].frequency_hz;

        assert_eq!(profile.id, ModalProfileId::Pipe);
        assert!(profile.modes.len() >= 6);

        for mode in profile.modes {
            assert!(mode.frequency_hz > previous_frequency);
            assert!(mode.gain < previous_gain);
            assert!(mode.decay_seconds > 0.0);
            previous_frequency = mode.frequency_hz;
            previous_gain = mode.gain;
        }

        let second_ratio = profile.modes[1].frequency_hz / fundamental;
        let fourth_ratio = profile.modes[3].frequency_hz / fundamental;

        assert!((second_ratio - 2.0).abs() < 0.02);
        assert!((fourth_ratio - 4.0).abs() < 0.04);
        assert!(profile.modes[0].decay_seconds >= 1.5);
    }

    #[test]
    fn plate_profile_is_available_through_shared_profile_system() {
        let profile = ModalProfile::from_id(ModalProfileId::Plate);
        let resonator = PlaceholderResonator::with_profile(ModalProfileId::Plate);

        assert_eq!(profile.id, ModalProfileId::Plate);
        assert_eq!(profile.modes.len(), PLATE_MODAL_PROFILE_MODES.len());
        assert_eq!(resonator.profile, ModalProfileId::Plate);
        assert_eq!(resonator.size_scale, SizeScale::default());
        assert_eq!(resonator.modes.len(), PLATE_MODAL_PROFILE_MODES.len());
    }

    #[test]
    fn plate_profile_is_curated_for_flatter_metallic_ring_direction() {
        let profile = ModalProfile::plate();
        let mut previous_frequency = 0.0;
        let mut previous_gain = f32::INFINITY;
        let fundamental = profile.modes[0].frequency_hz;
        let mut largest_ratio_deviation = 0.0_f32;

        assert_eq!(profile.id, ModalProfileId::Plate);
        assert!(profile.modes.len() >= 7);

        for (index, mode) in profile.modes.iter().enumerate() {
            assert!(mode.frequency_hz > previous_frequency);
            assert!(mode.gain < previous_gain);
            assert!(mode.decay_seconds > 0.0);

            if index > 0 {
                let harmonic_number = (index + 1) as f32;
                let ratio = mode.frequency_hz / fundamental;
                largest_ratio_deviation =
                    largest_ratio_deviation.max((ratio - harmonic_number).abs());
            }

            previous_frequency = mode.frequency_hz;
            previous_gain = mode.gain;
        }

        assert!(largest_ratio_deviation > 0.20);
        assert!(profile.modes[0].decay_seconds < ModalProfile::pipe().modes[0].decay_seconds);
    }

    #[test]
    fn modal_profiles_follow_distinct_family_metric_directions() {
        let pipe_metrics = profile_metrics(ModalProfile::pipe());
        let plate_metrics = profile_metrics(ModalProfile::plate());
        let tank_metrics = profile_metrics(ModalProfile::tank());

        assert!(pipe_metrics.harmonic_deviation < plate_metrics.harmonic_deviation);
        assert!(pipe_metrics.harmonic_deviation < tank_metrics.harmonic_deviation);
        assert!(plate_metrics.weighted_frequency > pipe_metrics.weighted_frequency);
        assert!(pipe_metrics.weighted_frequency > tank_metrics.weighted_frequency);
        assert!(tank_metrics.average_decay > pipe_metrics.average_decay);
        assert!(pipe_metrics.average_decay > plate_metrics.average_decay);
    }

    #[test]
    fn plate_profile_render_stays_deterministic_and_differs_from_pipe() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 4_096,
            ..RenderConfig::default()
        });

        let (_, first_plate_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Plate));
        let (_, second_plate_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Plate));
        let (_, pipe_summary) = renderer.render(PlaceholderResonator::default());

        assert_eq!(first_plate_summary, second_plate_summary);
        assert_ne!(first_plate_summary.checksum, pipe_summary.checksum);
        assert_ne!(
            first_plate_summary.first_samples,
            pipe_summary.first_samples
        );
        assert!(first_plate_summary.peak > 0.0);
        assert!(first_plate_summary.rms > 0.0);
    }

    #[test]
    fn tank_profile_is_available_through_shared_profile_system() {
        let profile = ModalProfile::from_id(ModalProfileId::Tank);
        let resonator = PlaceholderResonator::with_profile(ModalProfileId::Tank);

        assert_eq!(profile.id, ModalProfileId::Tank);
        assert_eq!(profile.modes.len(), TANK_MODAL_PROFILE_MODES.len());
        assert_eq!(resonator.profile, ModalProfileId::Tank);
        assert_eq!(resonator.size_scale, SizeScale::default());
        assert_eq!(resonator.modes.len(), TANK_MODAL_PROFILE_MODES.len());
    }

    #[test]
    fn size_scaling_lowers_and_extends_pipe_modes() {
        assert_size_scaling_direction(ModalProfileId::Pipe);
    }

    #[test]
    fn size_scaling_lowers_and_extends_plate_modes() {
        assert_size_scaling_direction(ModalProfileId::Plate);
    }

    #[test]
    fn size_scaling_lowers_and_extends_tank_modes() {
        assert_size_scaling_direction(ModalProfileId::Tank);
    }

    #[test]
    fn tank_profile_is_curated_for_boomier_cavity_like_metallic_direction() {
        let profile = ModalProfile::tank();
        let pipe = ModalProfile::pipe();
        let plate = ModalProfile::plate();
        let mut previous_frequency = 0.0;
        let mut previous_gain = f32::INFINITY;
        let fundamental = profile.modes[0].frequency_hz;
        let mut low_band_gain = 0.0_f32;
        let mut high_band_gain = 0.0_f32;
        let mut largest_harmonic_deviation = 0.0_f32;

        assert_eq!(profile.id, ModalProfileId::Tank);
        assert!(profile.modes.len() >= 8);

        for (index, mode) in profile.modes.iter().enumerate() {
            assert!(mode.frequency_hz > previous_frequency);
            assert!(mode.gain < previous_gain);
            assert!(mode.decay_seconds > 0.0);

            if mode.frequency_hz < 400.0 {
                low_band_gain += mode.gain;
            } else {
                high_band_gain += mode.gain;
            }

            if index > 0 {
                let harmonic_number = (index + 1) as f32;
                let ratio = mode.frequency_hz / fundamental;
                largest_harmonic_deviation =
                    largest_harmonic_deviation.max((ratio - harmonic_number).abs());
            }

            previous_frequency = mode.frequency_hz;
            previous_gain = mode.gain;
        }

        assert!(fundamental < pipe.modes[0].frequency_hz);
        assert!(fundamental < plate.modes[0].frequency_hz);
        assert!(profile.modes[0].decay_seconds > pipe.modes[0].decay_seconds);
        assert!(profile.modes[0].decay_seconds > plate.modes[0].decay_seconds);
        assert!(low_band_gain > high_band_gain);
        assert!(largest_harmonic_deviation > 0.35);
    }

    #[test]
    fn tank_profile_render_stays_deterministic_and_differs_from_pipe_and_plate() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 4_096,
            ..RenderConfig::default()
        });

        let (_, first_tank_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Tank));
        let (_, second_tank_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Tank));
        let (_, pipe_summary) = renderer.render(PlaceholderResonator::default());
        let (_, plate_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Plate));

        assert_eq!(first_tank_summary, second_tank_summary);
        assert_ne!(first_tank_summary.checksum, pipe_summary.checksum);
        assert_ne!(first_tank_summary.checksum, plate_summary.checksum);
        assert_ne!(first_tank_summary.first_samples, pipe_summary.first_samples);
        assert_ne!(
            first_tank_summary.first_samples,
            plate_summary.first_samples
        );
        assert!(first_tank_summary.peak > 0.0);
        assert!(first_tank_summary.rms > 0.0);
    }

    #[test]
    fn family_renders_separate_by_brightness_and_decay_behavior() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });

        let (pipe_output, pipe_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Pipe));
        let (plate_output, plate_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Plate));
        let (tank_output, tank_summary) =
            renderer.render(PlaceholderResonator::with_profile(ModalProfileId::Tank));

        let pipe_metrics = render_behavior_metrics(&pipe_output);
        let plate_metrics = render_behavior_metrics(&plate_output);
        let tank_metrics = render_behavior_metrics(&tank_output);

        assert_ne!(pipe_summary.checksum, plate_summary.checksum);
        assert_ne!(pipe_summary.checksum, tank_summary.checksum);
        assert_ne!(plate_summary.checksum, tank_summary.checksum);

        assert!(plate_metrics.zero_crossings > pipe_metrics.zero_crossings);
        assert!(pipe_metrics.zero_crossings > tank_metrics.zero_crossings);
        assert!(plate_metrics.brightness_proxy > pipe_metrics.brightness_proxy);
        assert!(pipe_metrics.brightness_proxy > tank_metrics.brightness_proxy);
        assert!(tank_metrics.late_to_early_energy_ratio > pipe_metrics.late_to_early_energy_ratio);
        assert!(pipe_metrics.late_to_early_energy_ratio > plate_metrics.late_to_early_energy_ratio);
    }

    #[test]
    fn rust_transform_darkens_and_shortens_shared_pipe_modes() {
        let profile = ModalProfile::pipe();
        let clean_metrics = profile_metrics_for_modes(&profile.scaled_mode_specs(
            SizeScale::default(),
            RustAmount::default(),
            DamageAmount::default(),
        ));
        let rusted_metrics = profile_metrics_for_modes(&profile.scaled_mode_specs(
            SizeScale::default(),
            RustAmount::new(0.85),
            DamageAmount::default(),
        ));

        assert!(rusted_metrics.weighted_frequency < clean_metrics.weighted_frequency);
        assert!(rusted_metrics.average_decay < clean_metrics.average_decay);
        assert!(rusted_metrics.low_band_gain_ratio > clean_metrics.low_band_gain_ratio);
    }

    #[test]
    fn rust_render_is_deterministic_and_reduces_brightness_and_decay() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });

        let (clean_output, clean_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_and_rust(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::default(),
            ));
        let (rusted_output, rusted_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_and_rust(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::new(0.85),
            ));
        let (_, repeated_rusted_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_and_rust(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::new(0.85),
            ));

        let clean_metrics = render_behavior_metrics(&clean_output);
        let rusted_metrics = render_behavior_metrics(&rusted_output);

        assert_eq!(rusted_summary, repeated_rusted_summary);
        assert_ne!(clean_summary.checksum, rusted_summary.checksum);
        assert!(rusted_metrics.brightness_proxy < clean_metrics.brightness_proxy);
        assert!(
            rusted_metrics.late_to_early_energy_ratio < clean_metrics.late_to_early_energy_ratio
        );
        assert!(rusted_summary.peak < clean_summary.peak);
    }

    #[test]
    fn high_rust_render_is_darker_and_shorter_than_low_rust_render() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });

        let (low_output, low_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_and_rust(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::new(0.25),
            ));
        let (high_output, high_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_and_rust(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::new(0.85),
            ));

        let low_metrics = render_behavior_metrics(&low_output);
        let high_metrics = render_behavior_metrics(&high_output);

        assert_ne!(low_summary.checksum, high_summary.checksum);
        assert!(high_metrics.brightness_proxy < low_metrics.brightness_proxy);
        assert!(high_metrics.late_to_early_energy_ratio < low_metrics.late_to_early_energy_ratio);
        assert!(high_summary.peak < low_summary.peak);
    }

    #[test]
    fn damage_transform_detunes_and_expands_shared_pipe_modes() {
        let profile = ModalProfile::pipe();
        let clean_modes = profile.scaled_mode_specs(
            SizeScale::default(),
            RustAmount::default(),
            DamageAmount::default(),
        );
        let damaged_modes = profile.scaled_mode_specs(
            SizeScale::default(),
            RustAmount::default(),
            DamageAmount::new(0.85),
        );
        let clean_metrics = profile_metrics_for_modes(&clean_modes);
        let damaged_metrics = profile_metrics_for_modes(&damaged_modes);

        assert_eq!(clean_modes.len(), PIPE_MODAL_PROFILE_MODES.len());
        assert_eq!(damaged_modes.len(), PIPE_MODAL_PROFILE_MODES.len() * 2);
        assert!(damaged_metrics.harmonic_deviation > clean_metrics.harmonic_deviation);
        assert!(damaged_metrics.average_decay < clean_metrics.average_decay);
    }

    #[test]
    fn damage_render_is_deterministic_and_increases_roughness() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });

        let (clean_output, clean_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_rust_and_damage(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::default(),
                DamageAmount::default(),
            ));
        let (damaged_output, damaged_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_rust_and_damage(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::default(),
                DamageAmount::new(0.85),
            ));
        let (_, repeated_damaged_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_rust_and_damage(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::default(),
                DamageAmount::new(0.85),
            ));

        let clean_metrics = render_behavior_metrics(&clean_output);
        let damaged_metrics = render_behavior_metrics(&damaged_output);

        assert_eq!(damaged_summary, repeated_damaged_summary);
        assert_ne!(clean_summary.checksum, damaged_summary.checksum);
        assert!(damaged_metrics.roughness_proxy > clean_metrics.roughness_proxy);
        assert!(
            damaged_metrics.late_to_early_energy_ratio < clean_metrics.late_to_early_energy_ratio
        );
    }

    #[test]
    fn high_damage_render_is_rougher_and_shorter_than_low_damage_render() {
        let renderer = OfflineRenderer::new(RenderConfig {
            frame_count: 8_192,
            ..RenderConfig::default()
        });

        let (low_output, low_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_rust_and_damage(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::default(),
                DamageAmount::new(0.25),
            ));
        let (high_output, high_summary) =
            renderer.render(PlaceholderResonator::with_profile_size_rust_and_damage(
                ModalProfileId::Pipe,
                SizeScale::default(),
                RustAmount::default(),
                DamageAmount::new(0.85),
            ));

        let low_metrics = render_behavior_metrics(&low_output);
        let high_metrics = render_behavior_metrics(&high_output);

        assert_ne!(low_summary.checksum, high_summary.checksum);
        assert!(high_metrics.roughness_proxy > low_metrics.roughness_proxy);
        assert!(high_metrics.late_to_early_energy_ratio < low_metrics.late_to_early_energy_ratio);
        assert!(high_summary.peak >= low_summary.peak);
    }

    #[test]
    fn realtime_mode_count_estimates_stay_grounded_in_current_profile_tables() {
        assert_eq!(
            realtime_mode_count_estimate(ModalProfileId::Pipe),
            RealtimeModeCountEstimate {
                profile_id: ModalProfileId::Pipe,
                canonical_mode_count: PIPE_MODAL_PROFILE_MODES.len(),
                safe_realtime_mode_count: PIPE_MODAL_PROFILE_MODES.len(),
                offline_peak_mode_count: PIPE_MODAL_PROFILE_MODES.len() * 2,
            }
        );
        assert_eq!(
            realtime_mode_count_estimate(ModalProfileId::Plate),
            RealtimeModeCountEstimate {
                profile_id: ModalProfileId::Plate,
                canonical_mode_count: PLATE_MODAL_PROFILE_MODES.len(),
                safe_realtime_mode_count: PLATE_MODAL_PROFILE_MODES.len(),
                offline_peak_mode_count: PLATE_MODAL_PROFILE_MODES.len() * 2,
            }
        );
        assert_eq!(
            realtime_mode_count_estimate(ModalProfileId::Tank),
            RealtimeModeCountEstimate {
                profile_id: ModalProfileId::Tank,
                canonical_mode_count: TANK_MODAL_PROFILE_MODES.len(),
                safe_realtime_mode_count: TANK_MODAL_PROFILE_MODES.len(),
                offline_peak_mode_count: TANK_MODAL_PROFILE_MODES.len() * 2,
            }
        );
    }

    #[test]
    fn shared_mode_limits_follow_current_family_extremes() {
        assert_eq!(
            safe_realtime_shared_mode_limit(),
            PLATE_MODAL_PROFILE_MODES.len()
        );
        assert_eq!(
            offline_peak_shared_mode_limit(),
            PLATE_MODAL_PROFILE_MODES.len() * 2
        );
    }

    #[test]
    fn resonator_coefficients_follow_spec_formula() {
        let mode = ModalModeSpec::new(330.0, 0.72, 0.12);
        let coefficients = ResonatorCoefficients::for_mode(mode, 48_000);
        let omega = 2.0 * std::f32::consts::PI * mode.frequency_hz / 48_000.0;
        let r = (-1.0 / (mode.decay_seconds * 48_000.0)).exp();

        approx_eq(coefficients.b0, mode.gain, 1.0e-8);
        approx_eq(coefficients.a1, -2.0 * r * omega.cos(), 1.0e-6);
        approx_eq(coefficients.a2, r * r, 1.0e-6);
    }

    #[test]
    fn second_order_mode_obeys_difference_equation() {
        let spec = ModalModeSpec::new(220.0, 0.95, 0.18);
        let mut mode = SecondOrderMode::new(spec);
        let sample_rate = 48_000;
        let coefficients = ResonatorCoefficients::for_mode(spec, sample_rate);

        let y0 = mode.process(1.0, sample_rate);
        let y1 = mode.process(0.0, sample_rate);
        let y2 = mode.process(0.0, sample_rate);

        approx_eq(y0, coefficients.b0, 1.0e-6);
        approx_eq(y1, -coefficients.a1 * y0, 1.0e-5);
        approx_eq(y2, -coefficients.a1 * y1 - coefficients.a2 * y0, 1.0e-5);
    }

    #[test]
    fn resonator_output_stays_finite_over_long_render() {
        let renderer = OfflineRenderer::new(RenderConfig::default());
        let (output, summary) = renderer.render(PlaceholderResonator::default());

        assert!(output.iter().all(|sample| sample.is_finite()));
        assert!(summary.peak.is_finite());
        assert!(summary.rms.is_finite());
    }
}
