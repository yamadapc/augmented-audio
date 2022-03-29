use augmented_atomics::{AtomicF32, AtomicValue};
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumIter, EnumProperty};

use crate::QuantizeMode;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LooperId(pub usize);

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumDiscriminants, PartialOrd, Ord)]
#[allow(clippy::enum_variant_names)]
pub enum EntityId {
    EntityIdLooperParameter(LooperId, ParameterId),
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Eq, Hash, EnumDiscriminants, PartialOrd, Ord)]
#[allow(clippy::enum_variant_names)]
pub enum ParameterId {
    ParameterIdSource(SourceParameter),
    ParameterIdEnvelope(EnvelopeParameter),
    ParameterIdLFO(usize, LFOParameter),
    ParameterIdQuantization(QuantizationParameter),
}

impl From<EnvelopeParameter> for ParameterId {
    fn from(inner: EnvelopeParameter) -> Self {
        ParameterId::ParameterIdEnvelope(inner)
    }
}

impl From<SourceParameter> for ParameterId {
    fn from(inner: SourceParameter) -> Self {
        ParameterId::ParameterIdSource(inner)
    }
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
    #[strum(props(type = "bool", default = "false"))]
    SliceEnabled = 8,
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

#[derive(Debug)]
pub enum ParameterValue {
    Float(AtomicF32),
    Bool(AtomicBool),
    Enum(AtomicUsize),
    Int(AtomicI32),
    None,
}

impl From<i32> for ParameterValue {
    fn from(v: i32) -> Self {
        ParameterValue::Int(v.into())
    }
}

impl From<usize> for ParameterValue {
    fn from(v: usize) -> Self {
        ParameterValue::Enum(v.into())
    }
}

impl From<bool> for ParameterValue {
    fn from(v: bool) -> Self {
        ParameterValue::Bool(v.into())
    }
}

impl From<f32> for ParameterValue {
    fn from(v: f32) -> Self {
        ParameterValue::Float(v.into())
    }
}

impl PartialEq for ParameterValue {
    fn eq(&self, other: &Self) -> bool {
        use ParameterValue::*;
        match (self, other) {
            (Float(f1), Float(f2)) => (f1.get() - f2.get()).abs() < f32::EPSILON,
            (Int(i1), Int(i2)) => i1.get() == i2.get(),
            (Bool(b1), Bool(b2)) => b1.get() == b2.get(),
            (Enum(e1), Enum(e2)) => e1.get() == e2.get(),
            (None, None) => true,
            _ => false,
        }
    }
}

impl Clone for ParameterValue {
    fn clone(&self) -> Self {
        match self {
            ParameterValue::Float(inner) => ParameterValue::Float(inner.get().into()),
            ParameterValue::Bool(inner) => {
                ParameterValue::Bool(inner.load(Ordering::Relaxed).into())
            }
            ParameterValue::Enum(inner) => ParameterValue::Enum(inner.get().into()),
            ParameterValue::Int(inner) => ParameterValue::Int(inner.get().into()),
            ParameterValue::None => ParameterValue::None,
        }
    }
}

impl ParameterValue {
    pub fn set_from(&self, other: &ParameterValue) {
        match self {
            ParameterValue::Float(f) => f.set(other.as_float()),
            ParameterValue::Bool(b) => b.set(other.as_bool()),
            ParameterValue::Enum(e) => {
                e.set(other.as_enum());
            }
            ParameterValue::Int(i) => i.set(other.as_int()),
            ParameterValue::None => {}
        }
    }

    pub fn as_int(&self) -> i32 {
        if let ParameterValue::Int(inner) = self {
            inner.get()
        } else {
            0
        }
    }

    pub fn as_bool(&self) -> bool {
        if let ParameterValue::Bool(inner) = self {
            inner.get()
        } else {
            false
        }
    }

    pub fn as_float(&self) -> f32 {
        if let ParameterValue::Float(inner) = self {
            inner.get()
        } else {
            0.0
        }
    }

    pub fn as_enum(&self) -> usize {
        if let ParameterValue::Enum(inner) = self {
            inner.get()
        } else {
            0
        }
    }
}

pub fn build_parameter_indexes(parameter_ids: &Vec<ParameterId>) -> HashMap<ParameterId, usize> {
    parameter_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (id.clone(), index))
        .collect()
}

pub fn build_default_parameters() -> (HashMap<ParameterId, ParameterValue>, Vec<ParameterId>) {
    use strum::IntoEnumIterator;

    let source_parameters: Vec<ParameterId> = SourceParameter::iter()
        .map(|parameter| ParameterId::ParameterIdSource(parameter))
        .collect();
    let envelope_parameters: Vec<ParameterId> = EnvelopeParameter::iter()
        .map(|parameter| ParameterId::ParameterIdEnvelope(parameter))
        .collect();
    let lfo_parameters: Vec<ParameterId> = LFOParameter::iter()
        .map(|parameter| {
            [
                ParameterId::ParameterIdLFO(0, parameter.clone()),
                ParameterId::ParameterIdLFO(1, parameter),
            ]
        })
        .flatten()
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

    let mut result = HashMap::new();
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
            result.insert(parameter_id, parameter_value);
        });

    (result, parameter_ids)
}

fn get_default_parameter_value(parameter_id: &ParameterId) -> ParameterValue {
    use std::str::FromStr;

    match parameter_id.get_str("type").unwrap() {
        "float" => {
            let f_str = parameter_id.get_str("default").unwrap();
            let f = f32::from_str(f_str).unwrap();
            ParameterValue::Float(f.into())
        }
        "bool" => {
            let b_str = parameter_id.get_str("default").unwrap();
            ParameterValue::Bool((b_str == "true").into())
        }
        "enum" => {
            let e_str = parameter_id.get_str("default").unwrap();
            let e = usize::from_str(e_str).unwrap();
            ParameterValue::Enum(e.into())
        }
        "int" => {
            if let Some(i_str) = parameter_id.get_str("default") {
                let i = i32::from_str(i_str).unwrap();
                ParameterValue::Int(i.into())
            } else {
                ParameterValue::None
            }
        }
        _ => panic!("Invalid parameter declaration"),
    }
}
