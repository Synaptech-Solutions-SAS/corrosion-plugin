use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::params::{object_param, CorrosionParams, Object};
use nih_plug::prelude::{util, FloatParam, FloatRange, IntParam, IntRange};

pub const PRESET_VERSION: &str = "3";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresetParameters {
    pub ui_scale: i32,
    pub env_attack: f32,
    pub env_decay: f32,
    pub env_sustain: f32,
    pub env_release: f32,
    pub mseg_onset: f32,
    pub mseg_attack: f32,
    pub mseg_hold: f32,
    pub mseg_decay: f32,
    pub mseg_sustain: f32,
    pub mseg_release: f32,
    pub env_amount: f32,
    pub velocity_to_peak: f32,
    pub loop_mode: i32,
    pub loop_start_stage: i32,
    pub loop_end_stage: i32,
    pub sync_rate: f32,
    pub global_time_scale: f32,
    pub velocity_to_level: f32,
    pub velocity_to_time: f32,
    pub curve_tension: f32,
    pub exciter_pressure: f32,
    pub exciter_speed: f32,
    pub exciter_roughness: f32,
    pub strike_position: f32,
    pub coupling_stiffness: f32,
    pub position_wander: f32,
    pub position_envelope: f32,
    pub fundamental_anchor: f32,
    pub res_damping: f32,
    pub res_brightness: f32,
    pub thickness: f32,
    pub heat: f32,
    pub sludge: f32,
    pub filter_cutoff: f32,
    pub filter_resonance: f32,
    pub component_tolerance: f32,
    pub drive_amount: f32,
    pub bias_starvation: f32,
    pub chaos_depth: f32,
    pub spread_width: f32,
    pub listener_proximity: f32,
    pub chassis_material: f32,
    pub chassis_volume: f32,
    pub space_mode: i32,
    pub space_amount: f32,
    pub factory_size: f32,
    pub machinery_clutter: f32,
    pub wall_impedance: f32,
    pub spring_tension: f32,
    pub wire_stiffness: f32,
    pub spring_tank_size: f32,
    pub delay_time: f32,
    pub machinery_movement: f32,
    pub high_frequency_damping: f32,
    pub analog_ceiling: f32,
    pub diode_softness: f32,
}

impl Default for PresetParameters {
    fn default() -> Self {
        Self::from_params(&CorrosionParams::default())
    }
}

impl PresetParameters {
    pub fn from_params(params: &CorrosionParams) -> Self {
        Self {
            ui_scale: params.ui_scale.value(),
            env_attack: params.env_attack.value(),
            env_decay: params.env_decay.value(),
            env_sustain: params.env_sustain.value(),
            env_release: params.env_release.value(),
            mseg_onset: params.mseg_onset.value(),
            mseg_attack: params.mseg_attack.value(),
            mseg_hold: params.mseg_hold.value(),
            mseg_decay: params.mseg_decay.value(),
            mseg_sustain: params.mseg_sustain.value(),
            mseg_release: params.mseg_release.value(),
            env_amount: params.env_amount.value(),
            velocity_to_peak: params.velocity_to_peak.value(),
            loop_mode: params.loop_mode.value(),
            loop_start_stage: params.loop_start_stage.value(),
            loop_end_stage: params.loop_end_stage.value(),
            sync_rate: params.sync_rate.value(),
            global_time_scale: params.global_time_scale.value(),
            velocity_to_level: params.velocity_to_level.value(),
            velocity_to_time: params.velocity_to_time.value(),
            curve_tension: params.curve_tension.value(),
            exciter_pressure: params.exciter_pressure.value(),
            exciter_speed: params.exciter_speed.value(),
            exciter_roughness: params.exciter_roughness.value(),
            strike_position: params.strike_position.value(),
            coupling_stiffness: params.coupling_stiffness.value(),
            position_wander: params.position_wander.value(),
            position_envelope: params.position_envelope.value(),
            fundamental_anchor: params.fundamental_anchor.value(),
            res_damping: params.res_damping.value(),
            res_brightness: params.res_brightness.value(),
            thickness: params.thickness.value(),
            heat: params.heat.value(),
            sludge: params.sludge.value(),
            filter_cutoff: params.filter_cutoff.value(),
            filter_resonance: params.filter_resonance.value(),
            component_tolerance: params.component_tolerance.value(),
            drive_amount: params.drive_amount.value(),
            bias_starvation: params.bias_starvation.value(),
            chaos_depth: params.chaos_depth.value(),
            spread_width: params.spread_width.value(),
            listener_proximity: params.listener_proximity.value(),
            chassis_material: params.chassis_material.value(),
            chassis_volume: params.chassis_volume.value(),
            space_mode: params.space_mode.value(),
            space_amount: params.space_amount.value(),
            factory_size: params.factory_size.value(),
            machinery_clutter: params.machinery_clutter.value(),
            wall_impedance: params.wall_impedance.value(),
            spring_tension: params.spring_tension.value(),
            wire_stiffness: params.wire_stiffness.value(),
            spring_tank_size: params.spring_tank_size.value(),
            delay_time: params.delay_time.value(),
            machinery_movement: params.machinery_movement.value(),
            high_frequency_damping: params.high_frequency_damping.value(),
            analog_ceiling: params.analog_ceiling.value(),
            diode_softness: params.diode_softness.value(),
        }
    }

    fn apply_to(self, params: &mut CorrosionParams) {
        params.ui_scale = IntParam::new(
            "UI Scale",
            self.ui_scale,
            IntRange::Linear { min: 0, max: 4 },
        );
        params.env_attack = float_param("Attack", self.env_attack, 0.001, 2.0);
        params.env_decay = float_param("Decay", self.env_decay, 0.01, 5.0);
        params.env_sustain = float_param("Sustain", self.env_sustain, 0.0, 1.0);
        params.env_release = float_param("Release", self.env_release, 0.01, 5.0);
        params.mseg_onset = float_param("Onset", self.mseg_onset, 0.001, 1.0);
        params.mseg_attack = float_param("MSEG Attack", self.mseg_attack, 0.001, 2.0);
        params.mseg_hold = float_param("Hold", self.mseg_hold, 0.0, 2.0);
        params.mseg_decay = float_param("MSEG Decay", self.mseg_decay, 0.01, 5.0);
        params.mseg_sustain = float_param("MSEG Sustain", self.mseg_sustain, 0.0, 1.0);
        params.mseg_release = float_param("MSEG Release", self.mseg_release, 0.01, 5.0);
        params.env_amount = float_param("Env Amount", self.env_amount, 0.0, 1.0);
        params.velocity_to_peak = float_param("Velocity To Peak", self.velocity_to_peak, 0.0, 1.0);
        params.loop_mode = IntParam::new(
            "Loop Mode",
            self.loop_mode,
            IntRange::Linear { min: 0, max: 2 },
        );
        params.loop_start_stage = IntParam::new(
            "Loop Start Stage",
            self.loop_start_stage,
            IntRange::Linear { min: 0, max: 5 },
        );
        params.loop_end_stage = IntParam::new(
            "Loop End Stage",
            self.loop_end_stage,
            IntRange::Linear { min: 0, max: 5 },
        );
        params.sync_rate = float_param("Sync Rate", self.sync_rate, 0.0, 1.0);
        params.global_time_scale =
            float_param("Global Time Scale", self.global_time_scale, 0.1, 10.0);
        params.velocity_to_level =
            float_param("Velocity To Level", self.velocity_to_level, 0.0, 1.0);
        params.velocity_to_time = float_param("Velocity To Time", self.velocity_to_time, 0.0, 1.0);
        params.curve_tension = float_param("Curve Tension", self.curve_tension, -1.0, 1.0);
        params.exciter_pressure = float_param("Pressure", self.exciter_pressure, 0.0, 1.0);
        params.exciter_speed = float_param("Speed", self.exciter_speed, 0.0, 1.0);
        params.exciter_roughness = float_param("Roughness", self.exciter_roughness, 0.0, 1.0);
        params.strike_position = float_param("Strike Position", self.strike_position, 0.0, 1.0);
        params.coupling_stiffness =
            float_param("Coupling Stiffness", self.coupling_stiffness, 0.0, 1.0);
        params.position_wander = float_param("Position Wander", self.position_wander, 0.0, 1.0);
        params.position_envelope =
            float_param("Position Envelope", self.position_envelope, 0.0, 1.0);
        params.fundamental_anchor =
            float_param("Fundamental Anchor", self.fundamental_anchor, 0.0, 1.0);
        params.res_damping = float_param("Damping", self.res_damping, 0.0, 1.0);
        params.res_brightness = float_param("Brightness", self.res_brightness, 0.0, 1.0);
        params.thickness = float_param("Thickness", self.thickness, 0.0, 1.0);
        params.heat = float_param("Heat", self.heat, 0.0, 1.0);
        params.sludge = float_param("Sludge", self.sludge, 0.0, 1.0);
        params.filter_cutoff = FloatParam::new(
            "Filter Cutoff",
            self.filter_cutoff,
            FloatRange::Skewed {
                min: 20.0,
                max: 20000.0,
                factor: 0.5,
            },
        );
        params.filter_resonance = float_param("Filter Resonance", self.filter_resonance, 0.0, 1.0);
        params.component_tolerance =
            float_param("Component Tolerance", self.component_tolerance, 0.0, 1.0);
        params.drive_amount = float_param("Drive Amount", self.drive_amount, 0.0, 5.0);
        params.bias_starvation = float_param("Bias Starvation", self.bias_starvation, 0.0, 1.0);
        params.chaos_depth = float_param("Chaos Depth", self.chaos_depth, 0.0, 1.0);
        params.spread_width = float_param("Spread Width", self.spread_width, 0.0, 1.0);
        params.listener_proximity =
            float_param("Listener Proximity", self.listener_proximity, 0.0, 1.0);
        params.chassis_material = float_param("Chassis Material", self.chassis_material, 0.0, 1.0);
        params.chassis_volume = float_param("Chassis Volume", self.chassis_volume, 0.0, 1.0);
        params.space_mode = IntParam::new(
            "Space Mode",
            self.space_mode,
            IntRange::Linear { min: 0, max: 3 },
        );
        params.space_amount = float_param("Space Amount", self.space_amount, 0.0, 1.0);
        params.factory_size = float_param("Factory Size", self.factory_size, 0.0, 1.0);
        params.machinery_clutter =
            float_param("Machinery Clutter", self.machinery_clutter, 0.0, 1.0);
        params.wall_impedance = float_param("Wall Impedance", self.wall_impedance, 0.0, 1.0);
        params.spring_tension = float_param("Spring Tension", self.spring_tension, 0.0, 1.0);
        params.wire_stiffness = float_param("Wire Stiffness", self.wire_stiffness, 0.0, 1.0);
        params.spring_tank_size = float_param("Spring Tank Size", self.spring_tank_size, 0.0, 1.0);
        params.delay_time = float_param("Delay Time", self.delay_time, 0.0, 1.0);
        params.machinery_movement =
            float_param("Machinery Movement", self.machinery_movement, 0.0, 1.0);
        params.high_frequency_damping = float_param(
            "High Frequency Damping",
            self.high_frequency_damping,
            0.0,
            1.0,
        );
        params.analog_ceiling = float_param("Analog Ceiling", self.analog_ceiling, 0.5, 1.0);
        params.diode_softness = float_param("Diode Softness", self.diode_softness, 0.0, 1.0);
    }
}

fn float_param(name: &'static str, value: f32, min: f32, max: f32) -> FloatParam {
    FloatParam::new(name, value, FloatRange::Linear { min, max })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub version: String,
    pub object: Object,
    pub exciter: i32,
    pub size: f32,
    pub rust: f32,
    pub damage: f32,
    pub drive: f32,
    pub output: f32,
    pub width: f32,
    pub body: f32,
    #[serde(default)]
    pub extra: PresetParameters,
}

impl Preset {
    pub fn from_params(name: impl Into<String>, params: &CorrosionParams) -> Self {
        Self {
            name: name.into(),
            version: PRESET_VERSION.to_string(),
            object: Object::from_int(params.object.value()),
            exciter: params.exciter.value(),
            size: params.size.value(),
            rust: params.rust.value(),
            damage: params.damage.value(),
            drive: params.drive.value(),
            output: params.output.value(),
            width: params.width.value(),
            body: params.body.value(),
            extra: PresetParameters::from_params(params),
        }
    }

    pub fn into_params(self) -> CorrosionParams {
        let mut params = CorrosionParams::default();
        params.object = object_param(self.object.to_int());
        params.exciter = crate::params::exciter_param(self.exciter);
        params.size = nih_plug::prelude::FloatParam::new(
            "Size",
            self.size,
            nih_plug::prelude::FloatRange::Linear {
                min: 0.05,
                max: 10.0,
            },
        );
        params.rust = nih_plug::prelude::FloatParam::new(
            "Rust",
            self.rust,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 5.0 },
        );
        params.damage = nih_plug::prelude::FloatParam::new(
            "Damage",
            self.damage,
            nih_plug::prelude::FloatRange::Linear {
                min: 0.0,
                max: 10.0,
            },
        );
        params.drive = nih_plug::prelude::FloatParam::new(
            "Drive",
            self.drive,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 5.0 },
        );
        params.output = nih_plug::prelude::FloatParam::new(
            "Output",
            self.output,
            nih_plug::prelude::FloatRange::Linear {
                min: 0.0,
                max: util::db_to_gain(40.0),
            },
        );
        params.width = nih_plug::prelude::FloatParam::new(
            "Width",
            self.width,
            nih_plug::prelude::FloatRange::Linear {
                min: -2.0,
                max: 3.0,
            },
        );
        params.body = nih_plug::prelude::FloatParam::new(
            "Body",
            self.body,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 5.0 },
        );
        self.extra.apply_to(&mut params);
        params
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, json)
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(io::Error::other)
    }
}
