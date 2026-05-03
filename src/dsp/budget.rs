#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RealtimeModeCountEstimate {
    pub profile_id: crate::dsp::ModalProfileId,
    pub canonical_mode_count: usize,
    pub safe_realtime_mode_count: usize,
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
pub fn realtime_mode_count_estimate(
    profile_id: crate::dsp::ModalProfileId,
) -> RealtimeModeCountEstimate {
    RealtimeModeCountEstimate::for_profile(profile_id)
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn realtime_mode_count_estimates() -> [RealtimeModeCountEstimate; 3] {
    [
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Pipe),
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Plate),
        RealtimeModeCountEstimate::for_profile(crate::dsp::ModalProfileId::Tank),
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
