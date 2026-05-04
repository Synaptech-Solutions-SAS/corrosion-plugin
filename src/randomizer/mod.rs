use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomizerMode {
    Safe,
    Object,
    Damage,
    Full,
}

impl RandomizerMode {
    pub fn name(self) -> &'static str {
        match self {
            RandomizerMode::Safe => "Safe",
            RandomizerMode::Object => "Object",
            RandomizerMode::Damage => "Damage",
            RandomizerMode::Full => "Full",
        }
    }
}

pub fn safe_ranges() -> ParamRanges {
    ParamRanges {
        size: (0.5, 1.5),
        rust: (0.0, 0.5),
        damage: (0.0, 0.5),
        drive: (0.0, 0.5),
        width: (0.3, 0.8),
        body: (0.0, 0.5),
        output: (0.5, 1.0),
    }
}

pub fn object_ranges() -> ParamRanges {
    ParamRanges {
        size: (0.25, 2.0),
        rust: (0.0, 0.0),
        damage: (0.0, 0.0),
        drive: (0.0, 0.0),
        width: (0.5, 0.5),
        body: (0.0, 0.0),
        output: (0.7, 0.7),
    }
}

pub fn damage_ranges() -> ParamRanges {
    ParamRanges {
        size: (0.8, 1.2),
        rust: (0.0, 1.0),
        damage: (0.0, 1.0),
        drive: (0.0, 0.3),
        width: (0.5, 0.5),
        body: (0.0, 0.0),
        output: (0.7, 0.7),
    }
}

pub fn full_ranges() -> ParamRanges {
    ParamRanges {
        size: (0.25, 2.0),
        rust: (0.0, 1.0),
        damage: (0.0, 1.0),
        drive: (0.0, 1.0),
        width: (0.0, 1.0),
        body: (0.0, 1.0),
        output: (0.5, 1.0),
    }
}

pub struct ParamRanges {
    pub size: (f32, f32),
    pub rust: (f32, f32),
    pub damage: (f32, f32),
    pub drive: (f32, f32),
    pub width: (f32, f32),
    pub body: (f32, f32),
    pub output: (f32, f32),
}

pub fn ranges_for_mode(mode: RandomizerMode) -> ParamRanges {
    match mode {
        RandomizerMode::Safe => safe_ranges(),
        RandomizerMode::Object => object_ranges(),
        RandomizerMode::Damage => damage_ranges(),
        RandomizerMode::Full => full_ranges(),
    }
}

pub fn random_value(min: f32, max: f32, t: f32) -> f32 {
    min + t * (max - min)
}

pub fn randomize_params(mode: RandomizerMode, t_values: &[f32]) -> RandomizedParams {
    let ranges = ranges_for_mode(mode);
    
    RandomizedParams {
        size: random_value(ranges.size.0, ranges.size.1, t_values[0]),
        rust: random_value(ranges.rust.0, ranges.rust.1, t_values[1]),
        damage: random_value(ranges.damage.0, ranges.damage.1, t_values[2]),
        drive: random_value(ranges.drive.0, ranges.drive.1, t_values[3]),
        width: random_value(ranges.width.0, ranges.width.1, t_values[4]),
        body: random_value(ranges.body.0, ranges.body.1, t_values[5]),
        output: random_value(ranges.output.0, ranges.output.1, t_values[6]),
    }
}

pub struct RandomizedParams {
    pub size: f32,
    pub rust: f32,
    pub damage: f32,
    pub drive: f32,
    pub width: f32,
    pub body: f32,
    pub output: f32,
}

pub fn mutate_value(current: f32, amount: f32, t: f32) -> f32 {
    let jitter = (t - 0.5) * 2.0 * amount;
    (current + jitter).clamp(0.0, 1.0)
}

pub struct SafetyConstraints;

impl SafetyConstraints {
    pub fn is_safe_patch(params: &RandomizedParams) -> bool {
        if params.output < 0.01 {
            return false;
        }
        
        if params.damage > 0.8 && params.drive > 0.8 {
            return false;
        }
        
        if params.size < 0.1 || params.size > 3.0 {
            return false;
        }
        
        true
    }
    
    pub fn clamp_to_safe_ranges(params: &mut RandomizedParams) {
        params.size = params.size.clamp(0.25, 2.0);
        params.rust = params.rust.clamp(0.0, 1.0);
        params.damage = params.damage.clamp(0.0, 1.0);
        params.drive = params.drive.clamp(0.0, 1.0);
        params.width = params.width.clamp(0.0, 1.0);
        params.body = params.body.clamp(0.0, 1.0);
        params.output = params.output.clamp(0.1, 1.2);
    }
}

pub fn mode_param() -> IntParam {
    IntParam::new(
        "Randomizer Mode",
        0,
        IntRange::Linear { min: 0, max: 3 },
    )
    .with_value_to_string(Arc::new(|value| {
        RandomizerMode::from_int(value).name().to_string()
    }))
    .with_string_to_value(Arc::new(|string| {
        let normalized = string.trim().to_lowercase();
        match normalized.as_str() {
            "safe" => Some(0),
            "object" => Some(1),
            "damage" => Some(2),
            "full" => Some(3),
            _ => None,
        }
    }))
}

impl RandomizerMode {
    pub fn from_int(v: i32) -> Self {
        match v {
            1 => RandomizerMode::Object,
            2 => RandomizerMode::Damage,
            3 => RandomizerMode::Full,
            _ => RandomizerMode::Safe,
        }
    }
    
    pub fn to_int(self) -> i32 {
        match self {
            RandomizerMode::Safe => 0,
            RandomizerMode::Object => 1,
            RandomizerMode::Damage => 2,
            RandomizerMode::Full => 3,
        }
    }
}
