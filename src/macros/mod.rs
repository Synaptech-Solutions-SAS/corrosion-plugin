use nih_plug::prelude::*;

/// Calculates the Size value from Mass macro (0-1).
pub fn mass_to_size(mass: f32) -> f32 {
    let mass = mass.clamp(0.0, 1.0);
    0.25 + mass * 1.75
}

/// Calculates the Object index from Mass macro (0-1).
pub fn mass_to_object(mass: f32) -> i32 {
    let mass = mass.clamp(0.0, 1.0);
    if mass < 0.25 {
        0
    } else if mass < 0.5 {
        1
    } else if mass < 0.75 {
        2
    } else {
        3
    }
}

/// Calculates Rust value from Corrosion macro (0-1).
pub fn corrosion_to_rust(corrosion: f32) -> f32 {
    corrosion.clamp(0.0, 1.0)
}

/// Calculates Body value from Corrosion macro (0-1).
pub fn corrosion_to_body(corrosion: f32) -> f32 {
    corrosion.clamp(0.0, 1.0) * 0.8
}

/// Calculates Drive value from Violence macro (0-1).
pub fn violence_to_drive(violence: f32) -> f32 {
    violence.clamp(0.0, 1.0)
}

/// Calculates Damage value from Damage macro (0-1).
pub fn damage_to_damage(damage: f32) -> f32 {
    damage.clamp(0.0, 1.0)
}

pub fn mass_param() -> FloatParam {
    FloatParam::new(
        "Mass",
        0.5,
        FloatRange::Linear {
            min: 0.0,
            max: 1.0,
        },
    )
}

pub fn corrosion_param() -> FloatParam {
    FloatParam::new(
        "Corrosion",
        0.0,
        FloatRange::Linear {
            min: 0.0,
            max: 1.0,
        },
    )
}

pub fn violence_param() -> FloatParam {
    FloatParam::new(
        "Violence",
        0.2,
        FloatRange::Linear {
            min: 0.0,
            max: 1.0,
        },
    )
}

pub fn damage_macro_param() -> FloatParam {
    FloatParam::new(
        "Damage Macro",
        0.0,
        FloatRange::Linear {
            min: 0.0,
            max: 1.0,
        },
    )
}
