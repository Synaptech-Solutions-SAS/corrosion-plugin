//! Multistage envelope generator used by strike, friction, and sustain voices.
//!
//! The voice module drives this envelope at sample rate to shape excitation
//! and release behavior without allocating or consulting host state.

/// 6-Stage Multistage Envelope Generator (MSEG)
///
/// Stage order: Onset -> Attack -> Hold -> Decay -> Sustain -> Release
/// Supports looping between stages and curved transitions.
#[derive(Clone, Debug)]
/// Envelope state and controls for a multi-stage generator.
pub struct MSEG {
    // Stage timing (seconds)
    t_onset: f32,
    t_attack: f32,
    t_hold: f32,
    t_decay: f32,
    t_release: f32,

    // Stage levels (0.0 - 1.0)
    l_onset: f32,
    l_peak: f32,
    l_sustain: f32,
    l_end: f32,

    // Curve tensions (0.0 = linear, -1.0 = exponential/log, 1.0 = logarithmic/exp)
    c_onset: f32,
    c_attack: f32,
    c_decay: f32,
    c_release: f32,

    // Looping
    loop_mode: LoopMode,
    loop_start_stage: u8,
    loop_end_stage: u8,

    // Global modifiers
    global_time_scale: f32,
    velocity_to_level: f32,
    velocity_to_time: f32,

    // Runtime state
    gate: bool,
    stage: Stage,
    stage_time: f32,
    current_level: f32,
    sample_rate: f32,
    velocity_scale: f32,
    env_amount: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Individual envelope stages.
pub enum Stage {
    Idle,
    Onset,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Looping behavior for the sustain/decay region.
pub enum LoopMode {
    Off,
    Forward,
    PingPong,
}

impl MSEG {
    /// Create the default envelope configuration.
    pub fn new() -> Self {
        Self {
            t_onset: 0.01,
            t_attack: 0.05,
            t_hold: 0.02,
            t_decay: 0.3,
            t_release: 0.1,

            l_onset: 0.1,
            l_peak: 1.0,
            l_sustain: 0.5,
            l_end: 0.0,

            c_onset: 0.0,
            c_attack: -0.5,
            c_decay: 0.3,
            c_release: 0.0,

            loop_mode: LoopMode::Off,
            loop_start_stage: 3,
            loop_end_stage: 4,

            global_time_scale: 1.0,
            velocity_to_level: 1.0,
            velocity_to_time: 0.0,

            gate: false,
            stage: Stage::Idle,
            stage_time: 0.0,
            current_level: 0.0,
            sample_rate: 48000.0,
            velocity_scale: 1.0,
            env_amount: 1.0,
        }
    }

    /// Configure as a simple AR envelope for hits.
    pub fn hit_envelope(attack_ms: f32, release_ms: f32) -> Self {
        let mut env = Self::new();
        env.t_onset = 0.0;
        env.l_onset = 0.0;
        env.t_attack = attack_ms / 1000.0;
        env.t_hold = 0.0;
        env.t_decay = 0.0;
        env.t_release = release_ms / 1000.0;
        env.l_sustain = 0.0;
        env.loop_mode = LoopMode::Off;
        env
    }

    /// Configure for continuous friction behavior.
    pub fn scrape_envelope() -> Self {
        let mut env = Self::new();
        env.t_onset = 0.005;
        env.t_attack = 0.1;
        env.t_hold = 0.05;
        env.t_decay = 0.2;
        env.loop_mode = LoopMode::Forward;
        env.loop_start_stage = 3;
        env.loop_end_stage = 4;
        env
    }

    /// Configure for tension-rise and snap behavior.
    pub fn tension_rise_envelope() -> Self {
        let mut env = Self::new();
        env.t_onset = 0.0;
        env.t_attack = 0.5;
        env.t_hold = 0.0;
        env.t_decay = 0.1;
        env.l_peak = 0.9;
        env.l_sustain = 0.0;
        env.c_attack = -0.8;
        env
    }

    /// Set the sample rate used for per-sample timing.
    pub fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }

    /// Set stage durations in seconds.
    pub fn set_stage_times(
        &mut self,
        onset: f32,
        attack: f32,
        hold: f32,
        decay: f32,
        release: f32,
    ) {
        self.t_onset = onset.max(0.0);
        self.t_attack = attack.max(0.0);
        self.t_hold = hold.max(0.0);
        self.t_decay = decay.max(0.0);
        self.t_release = release.max(0.0);
    }

    /// Set stage levels in the 0..1 range.
    pub fn set_stage_levels(&mut self, onset: f32, peak: f32, sustain: f32, end: f32) {
        self.l_onset = onset.clamp(0.0, 1.0);
        self.l_peak = peak.clamp(0.0, 1.0);
        self.l_sustain = sustain.clamp(0.0, 1.0);
        self.l_end = end.clamp(0.0, 1.0);
    }

    /// Set curve tensions for onset, attack, decay, and release.
    pub fn set_curves(&mut self, onset: f32, attack: f32, decay: f32, release: f32) {
        self.c_onset = onset.clamp(-1.0, 1.0);
        self.c_attack = attack.clamp(-1.0, 1.0);
        self.c_decay = decay.clamp(-1.0, 1.0);
        self.c_release = release.clamp(-1.0, 1.0);
    }

    /// Set looping behavior and loop bounds.
    pub fn set_loop(&mut self, mode: LoopMode, start_stage: u8, end_stage: u8) {
        self.loop_mode = mode;
        self.loop_start_stage = start_stage.clamp(1, 4);
        self.loop_end_stage = end_stage.clamp(1, 4);
    }

    /// Set how velocity affects level and time.
    pub fn set_velocity_response(&mut self, to_level: f32, to_time: f32) {
        self.velocity_to_level = to_level.clamp(0.0, 1.0);
        self.velocity_to_time = to_time.clamp(0.0, 1.0);
    }

    /// Set the global time scale.
    pub fn set_global_time_scale(&mut self, scale: f32) {
        self.global_time_scale = scale.clamp(0.1, 10.0);
    }

    /// Trigger the envelope with a normalized velocity.
    pub fn trigger(&mut self, velocity: f32) {
        self.gate = true;
        self.stage = Stage::Onset;
        self.stage_time = 0.0;
        self.velocity_scale = 0.5 + velocity * 0.5;
        self.current_level = 0.0;
    }

    /// Enter the release stage if the envelope is active.
    pub fn release(&mut self) {
        self.gate = false;
        if self.stage != Stage::Idle && self.stage != Stage::Release {
            self.stage = Stage::Release;
            self.stage_time = 0.0;
        }
    }

    /// Set the final output amount.
    pub fn set_env_amount(&mut self, amount: f32) {
        self.env_amount = amount.clamp(0.0, 1.0);
    }

    /// Process one sample and return the envelope level.
    pub fn process_sample(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let time_scale =
            self.global_time_scale * (1.0 - self.velocity_to_time * (1.0 - self.velocity_scale));

        self.stage_time += dt * time_scale;

        let (target_level, stage_duration, curve) = match self.stage {
            Stage::Idle => {
                self.current_level = 0.0;
                return 0.0;
            }
            Stage::Onset => {
                if self.t_onset <= 0.0 {
                    self.advance_stage();
                    return self.process_sample();
                }
                if self.stage_time >= self.t_onset {
                    self.advance_stage();
                    return self.process_sample();
                }
                (self.l_onset, self.t_onset, self.c_onset)
            }
            Stage::Attack => {
                if self.stage_time >= self.t_attack {
                    self.advance_stage();
                    return self.process_sample();
                }
                (self.l_peak, self.t_attack, self.c_attack)
            }
            Stage::Hold => {
                if self.stage_time >= self.t_hold {
                    self.advance_stage();
                    return self.process_sample();
                }
                (self.l_peak, self.t_hold, 0.0)
            }
            Stage::Decay => {
                if self.stage_time >= self.t_decay {
                    self.advance_stage();
                    return self.process_sample();
                }
                (self.l_sustain, self.t_decay, self.c_decay)
            }
            Stage::Sustain => {
                if !self.gate {
                    self.advance_stage();
                    return self.process_sample();
                }
                if self.loop_mode != LoopMode::Off && self.stage_time >= 0.0 {
                    self.handle_loop();
                    return self.process_sample();
                }
                (self.l_sustain, 1.0, 0.0)
            }
            Stage::Release => {
                if self.stage_time >= self.t_release {
                    self.stage = Stage::Idle;
                    self.current_level = self.l_end;
                    return self.current_level * self.env_amount;
                }
                (self.l_end, self.t_release, self.c_release)
            }
        };

        let progress = (self.stage_time / stage_duration).clamp(0.0, 1.0);
        let curved_progress = Self::apply_curve(progress, curve);

        let start_level = match self.stage {
            Stage::Onset => 0.0,
            Stage::Attack => self.l_onset,
            Stage::Hold => self.l_peak,
            Stage::Decay => self.l_peak,
            Stage::Sustain => self.l_sustain,
            Stage::Release => self.l_sustain,
            _ => 0.0,
        };

        self.current_level = start_level + (target_level - start_level) * curved_progress;

        let velocity_adjusted = 0.5
            + (self.current_level - 0.5) * self.velocity_scale * self.velocity_to_level
            + self.current_level * (1.0 - self.velocity_to_level);

        velocity_adjusted.clamp(0.0, 1.0) * self.env_amount
    }

    fn advance_stage(&mut self) {
        self.stage_time = 0.0;
        self.stage = match self.stage {
            Stage::Onset => Stage::Attack,
            Stage::Attack => Stage::Hold,
            Stage::Hold => Stage::Decay,
            Stage::Decay => {
                if self.gate {
                    Stage::Sustain
                } else {
                    Stage::Release
                }
            }
            Stage::Sustain => {
                if self.gate {
                    Stage::Sustain
                } else {
                    Stage::Release
                }
            }
            Stage::Release => Stage::Idle,
            Stage::Idle => Stage::Idle,
        };
    }

    fn handle_loop(&mut self) {
        match self.loop_mode {
            LoopMode::Forward => {
                self.stage = match self.loop_start_stage {
                    1 => Stage::Attack,
                    2 => Stage::Hold,
                    3 => Stage::Decay,
                    4 => Stage::Sustain,
                    _ => Stage::Decay,
                };
                self.stage_time = 0.0;
            }
            LoopMode::PingPong => {
                self.stage = Stage::Decay;
                self.stage_time = 0.0;
            }
            LoopMode::Off => {}
        }
    }

    /// Apply a curve tension to linear progress.
    fn apply_curve(t: f32, curve: f32) -> f32 {
        if curve.abs() < 0.001 {
            return t;
        }

        if curve < 0.0 {
            let exponent = 1.0 + curve.abs();
            t.powf(exponent)
        } else {
            let exponent = 1.0 + curve;
            1.0 - (1.0 - t).powf(exponent)
        }
    }

    /// Return whether the envelope is active.
    pub fn is_active(&self) -> bool {
        self.stage != Stage::Idle
    }

    /// Return the current stage.
    pub fn stage(&self) -> Stage {
        self.stage
    }

    /// Return the current level.
    pub fn current_level(&self) -> f32 {
        self.current_level
    }
}

impl Default for MSEG {
    /// Construct the default envelope.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mseg_starts_idle() {
        let mseg = MSEG::new();
        assert!(!mseg.is_active());
        assert_eq!(mseg.stage(), Stage::Idle);
    }

    #[test]
    fn mseg_triggers_to_onset() {
        let mut mseg = MSEG::new();
        mseg.trigger(1.0);
        assert!(mseg.is_active());
        assert_eq!(mseg.stage(), Stage::Onset);
    }

    #[test]
    fn mseg_advances_through_stages() {
        let mut mseg = MSEG::new();
        mseg.set_sample_rate(1000.0);
        mseg.set_stage_times(0.01, 0.01, 0.01, 0.01, 0.01);
        mseg.trigger(1.0);

        let mut stages_seen = vec![];
        for _ in 0..100 {
            stages_seen.push(mseg.stage());
            mseg.process_sample();
        }

        assert!(
            stages_seen.contains(&Stage::Onset),
            "Should see Onset stage"
        );
        assert!(
            stages_seen.contains(&Stage::Attack),
            "Should see Attack stage"
        );
        assert!(stages_seen.contains(&Stage::Hold), "Should see Hold stage");
    }

    #[test]
    fn mseg_reaches_peak() {
        let mut mseg = MSEG::new();
        mseg.set_sample_rate(1000.0);
        mseg.set_stage_times(0.0, 0.01, 0.0, 0.0, 0.01);
        mseg.set_stage_levels(0.0, 1.0, 0.0, 0.0);
        mseg.trigger(1.0);

        let mut max_level: f32 = 0.0;
        for _ in 0..100 {
            let level = mseg.process_sample();
            max_level = max_level.max(level);
        }

        assert!(
            max_level > 0.5,
            "MSEG should reach significant level, got {}",
            max_level
        );
    }

    #[test]
    fn mseg_release_works() {
        let mut mseg = MSEG::new();
        mseg.set_sample_rate(1000.0);
        mseg.set_stage_times(0.0, 0.001, 0.001, 0.0, 0.01);
        mseg.trigger(1.0);

        for _ in 0..5 {
            mseg.process_sample();
        }

        mseg.release();
        assert_eq!(mseg.stage(), Stage::Release);

        for _ in 0..50 {
            mseg.process_sample();
        }
        assert!(!mseg.is_active());
    }

    #[test]
    fn curve_application() {
        assert!((MSEG::apply_curve(0.5, 0.0) - 0.5).abs() < 0.001);

        let convex = MSEG::apply_curve(0.5, -0.5);
        assert!(
            convex < 0.5,
            "Convex curve should be below linear at midpoint"
        );

        let concave = MSEG::apply_curve(0.5, 0.5);
        assert!(
            concave > 0.5,
            "Concave curve should be above linear at midpoint"
        );
    }

    #[test]
    fn velocity_affects_level() {
        let mut mseg_low = MSEG::new();
        let mut mseg_high = MSEG::new();

        mseg_low.set_sample_rate(1000.0);
        mseg_high.set_sample_rate(1000.0);
        mseg_low.set_stage_times(0.0, 0.01, 0.0, 0.0, 0.01);
        mseg_high.set_stage_times(0.0, 0.01, 0.0, 0.0, 0.01);
        mseg_low.set_velocity_response(1.0, 0.0);
        mseg_high.set_velocity_response(1.0, 0.0);

        mseg_low.trigger(0.3);
        mseg_high.trigger(1.0);

        let max_low: f32 = (0..100)
            .map(|_| mseg_low.process_sample())
            .fold(0.0, f32::max);
        let max_high: f32 = (0..100)
            .map(|_| mseg_high.process_sample())
            .fold(0.0, f32::max);

        assert!(
            max_high > max_low,
            "High velocity should give higher peak: high={}, low={}",
            max_high,
            max_low
        );
    }
}
