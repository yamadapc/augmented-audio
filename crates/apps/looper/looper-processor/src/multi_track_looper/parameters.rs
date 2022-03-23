use num_derive::{FromPrimitive, ToPrimitive};
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumIter, EnumProperty};

use crate::QuantizeMode;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LooperId(pub usize);

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumDiscriminants, PartialOrd, Ord)]
#[allow(clippy::enum_variant_names)]
pub enum ParameterId {
    ParameterIdSource(SourceParameter),
    ParameterIdEnvelope(EnvelopeParameter),
    ParameterIdLFO(usize, LFOParameter),
    ParameterIdQuantization(QuantizationParameter),
}

impl EnumProperty for ParameterId {
    fn get_str(&self, prop: &str) -> Option<&'static str> {
        match self {
            ParameterId::ParameterIdSource(parameter) => parameter.get_str(prop),
            ParameterId::ParameterIdEnvelope(parameter) => parameter.get_str(prop),
            ParameterId::ParameterIdLFO(_, parameter) => parameter.get_str(prop),
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
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty, PartialOrd, Ord)]
#[allow(clippy::enum_variant_names)]
pub enum QuantizationParameter {
    #[strum(props(type = "enum", default = "0"))]
    QuantizationParameterQuantizeMode = 0,
    #[strum(props(type = "enum", default = "0"))]
    QuantizationParameterTempoControl = 1,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, FromPrimitive, ToPrimitive)]
#[allow(clippy::enum_variant_names)]
pub enum CQuantizeMode {
    CQuantizeModeSnapClosest = 0,
    CQuantizeModeSnapNext = 1,
    CQuantizeModeNone = 2,
}

impl From<CQuantizeMode> for QuantizeMode {
    fn from(mode: CQuantizeMode) -> QuantizeMode {
        match mode {
            CQuantizeMode::CQuantizeModeSnapClosest => QuantizeMode::SnapClosest,
            CQuantizeMode::CQuantizeModeSnapNext => QuantizeMode::SnapNext,
            CQuantizeMode::CQuantizeModeNone => QuantizeMode::None,
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, FromPrimitive, ToPrimitive, PartialOrd, Ord)]
pub enum TempoControl {
    TempoControlSetGlobalTempo = 0,
    TempoControlNone = 1,
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty, PartialOrd, Ord)]
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
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumIter, EnumProperty, PartialOrd, Ord)]
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

pub fn build_default_parameters() -> (
    lockfree::map::Map<ParameterId, ParameterValue>,
    Vec<ParameterId>,
) {
    use strum::IntoEnumIterator;

    let source_parameters: Vec<ParameterId> = SourceParameter::iter()
        .map(|parameter| ParameterId::ParameterIdSource(parameter))
        .collect();
    let envelope_parameters: Vec<ParameterId> = EnvelopeParameter::iter()
        .map(|parameter| ParameterId::ParameterIdEnvelope(parameter))
        .collect();
    let lfo_parameters: Vec<ParameterId> = LFOParameter::iter()
        .map(|parameter| ParameterId::ParameterIdLFO(0, parameter))
        .collect();
    let quantization_parameters: Vec<ParameterId> = QuantizationParameter::iter()
        .map(ParameterId::ParameterIdQuantization)
        .collect();
    let parameter_ids: Vec<ParameterId> = source_parameters
        .iter()
        .chain(envelope_parameters.iter())
        .chain(lfo_parameters.iter())
        .chain(quantization_parameters.iter())
        .cloned()
        .collect();

    let result = lockfree::map::Map::new();

    parameter_ids
        .iter()
        .flat_map(|parameter_id| {
            let default_value = get_default_parameter_value(parameter_id);

            if let ParameterId::ParameterIdLFO(_, parameter) = parameter_id {
                (0..2)
                    .map(|lfo| {
                        (
                            ParameterId::ParameterIdLFO(lfo, parameter.clone()),
                            default_value.clone(),
                        )
                    })
                    .collect()
            } else {
                vec![(parameter_id.clone(), default_value)]
            }
        })
        .for_each(|(parameter_id, parameter_value)| {
            let _ = result.insert(parameter_id, parameter_value);
        });

    (result, parameter_ids)
}

fn get_default_parameter_value(parameter_id: &ParameterId) -> ParameterValue {
    use std::str::FromStr;

    match parameter_id.get_str("type").unwrap() {
        "float" => {
            let f_str = parameter_id.get_str("default").unwrap();
            let f = f32::from_str(f_str).unwrap();
            ParameterValue::Float(f)
        }
        "bool" => {
            let b_str = parameter_id.get_str("default").unwrap();
            ParameterValue::Bool(b_str == "true")
        }
        "enum" => {
            let e_str = parameter_id.get_str("default").unwrap();
            let e = usize::from_str(e_str).unwrap();
            ParameterValue::Enum(e)
        }
        "int" => {
            if let Some(i_str) = parameter_id.get_str("default") {
                let i = i32::from_str(i_str).unwrap();
                ParameterValue::Int(i)
            } else {
                ParameterValue::None
            }
        }
        _ => panic!("Invalid parameter declaration"),
    }
}
