use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::params::{object_param, CorrosionParams, Object};

pub const PRESET_VERSION: &str = "1";

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
                min: 0.25,
                max: 2.0,
            },
        );
        params.rust = nih_plug::prelude::FloatParam::new(
            "Rust",
            self.rust,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 1.0 },
        );
        params.damage = nih_plug::prelude::FloatParam::new(
            "Damage",
            self.damage,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 1.0 },
        );
        params.drive = nih_plug::prelude::FloatParam::new(
            "Drive",
            self.drive,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 1.0 },
        );
        params.output = nih_plug::prelude::FloatParam::new(
            "Output",
            self.output,
            nih_plug::prelude::FloatRange::Linear {
                min: 0.0,
                max: nih_plug::prelude::util::db_to_gain(12.0),
            },
        );
        params.width = nih_plug::prelude::FloatParam::new(
            "Width",
            self.width,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 1.0 },
        );
        params.body = nih_plug::prelude::FloatParam::new(
            "Body",
            self.body,
            nih_plug::prelude::FloatRange::Linear { min: 0.0, max: 1.0 },
        );
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
