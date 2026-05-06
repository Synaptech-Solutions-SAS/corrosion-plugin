//! Realtime mode-budget estimates.
//!
//! The voice layer uses these helpers to choose safe polyphonic mode counts
//! without allocating or recalculating expensive profile expansions at audio
//! rate.

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Summary of safe and peak mode counts for a modal profile.
pub struct RealtimeModeCountEstimate {
    /// Source profile identifier.
    pub profile_id: crate::dsp::ModalProfileId,
    /// Canonical mode count in the base profile.
    pub canonical_mode_count: usize,
    /// Conservative realtime limit for the profile.
    pub safe_realtime_mode_count: usize,
    /// Peak mode count after offline expansion.
    pub offline_peak_mode_count: usize,
}

impl RealtimeModeCountEstimate {
    #[cfg_attr(not(test), allow(dead_code))]
    fn for_profile(profile_id: crate::dsp::ModalProfileId) -> Self {
        let profile = crate::dsp::ModalProfile::from_id(profile_id);
        let canonical_mode_count = profile.modes.len();
        let offline_peak_mode_count = profile
            .scaled_mode_specs(
                crate::dsp::SizeScale::default(),
                crate::dsp::RustAmount::default(),
                crate::dsp::DamageAmount::new(1.0),
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
/// Build an estimate for a single modal profile.
pub fn realtime_mode_count_estimate(
    profile_id: crate::dsp::ModalProfileId,
) -> RealtimeModeCountEstimate {
    RealtimeModeCountEstimate::for_profile(profile_id)
}

#[cfg_attr(not(test), allow(dead_code))]
/// Build the standard profile set used by realtime budgeting.
pub fn realtime_mode_count_estimates() -> [RealtimeModeCountEstimate; 4] {
    [
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Pipe),
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Plate),
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Tank),
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Chain),
    ]
}

#[cfg_attr(not(test), allow(dead_code))]
/// Return the largest conservative realtime mode count across core profiles.
pub fn safe_realtime_shared_mode_limit() -> usize {
    realtime_mode_count_estimates()
        .into_iter()
        .map(|estimate| estimate.safe_realtime_mode_count)
        .max()
        .unwrap_or(0)
}

#[cfg_attr(not(test), allow(dead_code))]
/// Return the largest offline-expanded mode count across core profiles.
pub fn offline_peak_shared_mode_limit() -> usize {
    realtime_mode_count_estimates()
        .into_iter()
        .map(|estimate| estimate.offline_peak_mode_count)
        .max()
        .unwrap_or(0)
}
