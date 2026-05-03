use nih_plug::prelude::*;

#[derive(Params)]
pub struct CorrosionParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "object"]
    pub object: IntParam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl Default for CorrosionParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Linear {
                    min: 0.0,
                    max: util::db_to_gain(12.0),
                },
            ),
            object: IntParam::new("Object", 0, IntRange::Linear { min: 0, max: 2 }),
        }
    }
}
