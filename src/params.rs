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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Object {
    Pipe,
    Plate,
    Tank,
}

impl Object {
    pub fn from_int(v: i32) -> Self {
        match v {
            1 => Object::Plate,
            2 => Object::Tank,
            _ => Object::Pipe,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Object::Pipe => "Pipe",
            Object::Plate => "Plate",
            Object::Tank => "Tank",
        }
    }

    pub fn to_int(self) -> i32 {
        match self {
            Object::Pipe => 0,
            Object::Plate => 1,
            Object::Tank => 2,
        }
    }
}

pub fn object_param(default: i32) -> IntParam {
    IntParam::new("Object", default, IntRange::Linear { min: 0, max: 2 })
        .with_value_to_string(Arc::new(|value| Object::from_int(value).name().to_string()))
        .with_string_to_value(Arc::new(|string| {
            let normalized = string.trim();
            [Object::Pipe, Object::Plate, Object::Tank]
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
        }
    }
}
