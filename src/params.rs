use nih_plug::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[cfg(feature = "gui")]
use nih_plug_egui::EguiState;

#[derive(Params)]
pub struct CorrosionParams {
    #[cfg(feature = "gui")]
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    #[id = "exciter"]
    pub exciter: IntParam,
    #[id = "object"]
    pub object: IntParam,
    #[id = "size"]
    pub size: FloatParam,
    #[id = "rust"]
    pub rust: FloatParam,
    #[id = "damage"]
    pub damage: FloatParam,
    #[id = "drive"]
    pub drive: FloatParam,
    #[id = "output"]
    pub output: FloatParam,
    #[id = "width"]
    pub width: FloatParam,
    #[id = "body"]
    pub body: FloatParam,
    #[id = "mass"]
    pub mass: FloatParam,
    #[id = "corrosion"]
    pub corrosion: FloatParam,
    #[id = "violence"]
    pub violence: FloatParam,
    #[id = "damage_macro"]
    pub damage_macro: FloatParam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ExciterType {
    Hit,
    Scrape,
}

impl ExciterType {
    pub fn from_int(v: i32) -> Self {
        match v {
            1 => ExciterType::Scrape,
            _ => ExciterType::Hit,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ExciterType::Hit => "Hit",
            ExciterType::Scrape => "Scrape",
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            ExciterType::Hit => 0,
            ExciterType::Scrape => 1,
        }
    }
}

pub fn exciter_param(default: i32) -> IntParam {
    IntParam::new("Exciter", default, IntRange::Linear { min: 0, max: 1 })
        .with_value_to_string(Arc::new(|value| ExciterType::from_int(value).name().to_string()))
        .with_string_to_value(Arc::new(|string| {
            let normalized = string.trim();
            [ExciterType::Hit, ExciterType::Scrape]
                .into_iter()
                .find(|exciter| exciter.name().eq_ignore_ascii_case(normalized))
                .map(ExciterType::to_int)
        }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Object {
    Pipe,
    Plate,
    Tank,
    Chain,
}

impl Object {
    pub fn from_int(v: i32) -> Self {
        match v {
            1 => Object::Plate,
            2 => Object::Tank,
            3 => Object::Chain,
            _ => Object::Pipe,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Object::Pipe => "Pipe",
            Object::Plate => "Plate",
            Object::Tank => "Tank",
            Object::Chain => "Chain",
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            Object::Pipe => 0,
            Object::Plate => 1,
            Object::Tank => 2,
            Object::Chain => 3,
        }
    }
}

pub fn object_param(default: i32) -> IntParam {
    IntParam::new("Object", default, IntRange::Linear { min: 0, max: 3 })
        .with_value_to_string(Arc::new(|value| Object::from_int(value).name().to_string()))
        .with_string_to_value(Arc::new(|string| {
            let normalized = string.trim();
            [Object::Pipe, Object::Plate, Object::Tank, Object::Chain]
                .into_iter()
                .find(|object| object.name().eq_ignore_ascii_case(normalized))
                .map(Object::to_int)
        }))
}

impl Default for CorrosionParams {
    fn default() -> Self {
        Self {
            #[cfg(feature = "gui")]
            editor_state: EguiState::from_size(400, 300),
            exciter: exciter_param(0),
            object: object_param(0),
            size: FloatParam::new(
                "Size",
                1.0,
                FloatRange::Linear {
                    min: 0.25,
                    max: 2.0,
                },
            ),
            rust: FloatParam::new(
                "Rust",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            ),
            damage: FloatParam::new(
                "Damage",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            ),
            drive: FloatParam::new(
                "Drive",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            ),
            output: FloatParam::new(
                "Output",
                util::db_to_gain(0.0),
                FloatRange::Linear {
                    min: 0.0,
                    max: util::db_to_gain(12.0),
                },
            ),
            width: FloatParam::new(
                "Width",
                0.5,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            ),
            body: FloatParam::new(
                "Body",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            ),
            mass: crate::macros::mass_param(),
            corrosion: crate::macros::corrosion_param(),
            violence: crate::macros::violence_param(),
            damage_macro: crate::macros::damage_macro_param(),
        }
    }
}
