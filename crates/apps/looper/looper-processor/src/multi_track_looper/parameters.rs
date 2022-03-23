use num_derive::{FromPrimitive, ToPrimitive};
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumIter, EnumProperty};

use crate::QuantizeMode;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct LooperId(pub usize);

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumDiscriminants)]
pub enum ParameterId {
    ParameterIdSource { parameter: SourceParameter },
    ParameterIdEnvelope { parameter: EnvelopeParameter },
    ParameterIdLFO { lfo: usize, parameter: LFOParameter },
    ParameterIdQuantization(QuantizationParameter),
}

impl EnumProperty for ParameterId {
    fn get_str(&self, prop: &str) -> Option<&'static str> {
        match self {
            ParameterId::ParameterIdSource { parameter, .. } => parameter.get_str(prop),
            ParameterId::ParameterIdEnvelope { parameter, .. } => parameter.get_str(prop),
            ParameterId::ParameterIdLFO { parameter, .. } => parameter.get_str(prop),
            ParameterId::ParameterIdQuantization(parameter) => parameter.get_str(prop),
        }
    }
}

pub type SceneId = usize;

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty, PartialOrd, Ord)]
pub enum SourceParameter {
    #[strum(props(type = "float", default = "0.0"))]
    Start = 0,
    #[strum(props(type = "float", default = "1.0"))]
    End = 1,
    #[strum(props(type = "float", default = "0.0"))]
    FadeStart = 2,
    #[strum(props(type = "float", default = "0.0"))]
    FadeEnd = 3,
    #[strum(props(type = "float", default = "1.0"))]
    Pitch = 4,
    #[strum(props(type = "float", default = "1.0"))]
    Speed = 5,
    #[strum(props(type = "bool", default = "true"))]
    LoopEnabled = 6,
    #[strum(props(type = "int"))]
    SliceId = 7,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty)]
pub enum QuantizationParameter {
    #[strum(props(type = "enum", default = "0"))]
    QuantizationParameterQuantizeMode = 0,
    #[strum(props(type = "enum", default = "0"))]
    QuantizationParameterTempoControl = 1,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum CQuantizeMode {
    CQuantizeModeSnapClosest = 0,
    CQuantizeModeSnapNext = 1,
    CQuantizeModeNone = 2,
}

impl Into<QuantizeMode> for CQuantizeMode {
    fn into(self: Self) -> QuantizeMode {
        match self {
            CQuantizeMode::CQuantizeModeSnapClosest => QuantizeMode::SnapClosest,
            CQuantizeMode::CQuantizeModeSnapNext => QuantizeMode::SnapNext,
            CQuantizeMode::CQuantizeModeNone => QuantizeMode::None,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum TempoControl {
    TempoControlSetGlobalTempo = 0,
    TempoControlNone = 1,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty)]
pub enum EnvelopeParameter {
    #[strum(props(type = "float", default = "0.0"))]
    Attack = 0,
    #[strum(props(type = "float", default = "0.0"))]
    Decay = 1,
    #[strum(props(type = "float", default = "0.3"))]
    Release = 2,
    #[strum(props(type = "float", default = "1.0"))]
    Sustain = 3,
    #[strum(props(type = "bool", default = "false"))]
    EnvelopeEnabled = 4,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty)]
pub enum LFOParameter {
    #[strum(props(type = "float", default = "1.0"))]
    Frequency = 0,
    #[strum(props(type = "float", default = "1.0"))]
    Amount = 1,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParameterValue {
    Float(f32),
    Bool(bool),
    Enum(usize),
    Int(i32),
    None,
}
