use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};

use num_derive::{FromPrimitive, ToPrimitive};
use rustc_hash::FxHashMap;
use serde_derive::{Deserialize, Serialize};
use strum::EnumProperty;
use strum_macros::{EnumDiscriminants, EnumIter, EnumProperty};

use augmented_atomics::{AtomicF32, AtomicValue};

use crate::QuantizeMode;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LooperId(pub usize);

#[repr(C)]
#[derive(
    Debug, PartialEq, Clone, Eq, Hash, EnumDiscriminants, PartialOrd, Ord, Serialize, Deserialize,
)]
#[allow(clippy::enum_variant_names)]
pub enum EntityId {
    #[serde(rename = "LP")]
    EntityIdLooperParameter(LooperId, ParameterId),
    #[serde(rename = "RB")]
    EntityIdRecordButton,
}

#[repr(C)]
#[derive(
    Debug, PartialEq, Clone, Eq, Hash, EnumDiscriminants, PartialOrd, Ord, Serialize, Deserialize,
)]
#[allow(clippy::enum_variant_names)]
pub enum ParameterId {
    #[serde(rename = "S")]
    ParameterIdSource(SourceParameter),
    #[serde(rename = "E")]
    ParameterIdEnvelope(EnvelopeParameter),
    #[serde(rename = "L")]
    ParameterIdLFO(usize, LFOParameter),
    #[serde(rename = "Q")]
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

impl From<QuantizationParameter> for ParameterId {
    fn from(inner: QuantizationParameter) -> Self {
        ParameterId::ParameterIdQuantization(inner)
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
#[derive(
    Debug,
    PartialEq,
    Clone,
    Eq,
    Hash,
    EnumIter,
    EnumProperty,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    FromPrimitive,
    ToPrimitive,
)]
#[serde(try_from = "usize")]
#[serde(into = "usize")]
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
    #[strum(props(type = "int", default = "0"))]
    SliceId = 7,
    #[strum(props(type = "bool", default = "false"))]
    SliceEnabled = 8,
}

#[repr(C)]
#[derive(
    Debug,
    PartialEq,
    Clone,
    Eq,
    Hash,
    EnumIter,
    EnumProperty,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    FromPrimitive,
    ToPrimitive,
)]
#[serde(try_from = "usize")]
#[serde(into = "usize")]
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
#[derive(
    Debug,
    PartialEq,
    Clone,
    Eq,
    Hash,
    EnumIter,
    EnumProperty,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    FromPrimitive,
    ToPrimitive,
)]
#[serde(try_from = "usize")]
#[serde(into = "usize")]
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
#[derive(
    Debug,
    PartialEq,
    Clone,
    Eq,
    Hash,
    EnumIter,
    EnumProperty,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    FromPrimitive,
    ToPrimitive,
)]
#[serde(try_from = "usize")]
#[serde(into = "usize")]
pub enum LFOParameter {
    #[strum(props(type = "float", default = "1.0"))]
    Frequency = 0,
    #[strum(props(type = "float", default = "1.0"))]
    Amount = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParameterValue {
    #[serde(rename = "F")]
    Float(AtomicF32),
    #[serde(rename = "B")]
    Bool(AtomicBool),
    #[serde(rename = "E")]
    Enum(AtomicUsize),
    #[serde(rename = "I")]
    Int(AtomicI32),
}

// MARK: Convert from primitive value into "boxed" parameter-value

macro_rules! from_primitive {
    ($variant: path, $primitive: ident) => {
        impl From<$primitive> for ParameterValue {
            fn from(v: $primitive) -> Self {
                $variant(v.into())
            }
        }
    };
}

from_primitive!(ParameterValue::Int, i32);
from_primitive!(ParameterValue::Enum, usize);
from_primitive!(ParameterValue::Bool, bool);
from_primitive!(ParameterValue::Float, f32);

impl PartialEq for ParameterValue {
    fn eq(&self, other: &Self) -> bool {
        use ParameterValue::*;
        match (self, other) {
            (Float(f1), Float(f2)) => (f1.get() - f2.get()).abs() < f32::EPSILON,
            (Int(i1), Int(i2)) => i1.get() == i2.get(),
            (Bool(b1), Bool(b2)) => b1.get() == b2.get(),
            (Enum(e1), Enum(e2)) => e1.get() == e2.get(),
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

pub fn build_parameter_indexes(parameter_ids: &[ParameterId]) -> FxHashMap<ParameterId, usize> {
    parameter_ids
        .iter()
        .enumerate()
        .map(|(index, id)| (id.clone(), index))
        .collect()
}

/// This function uses `strum` derived enumerator traits to extract all possible `ParameterId`s in
/// order
pub fn build_parameter_ids() -> Vec<ParameterId> {
    use strum::IntoEnumIterator;

    let source_parameters: Vec<ParameterId> = SourceParameter::iter()
        .map(ParameterId::ParameterIdSource)
        .collect();
    let envelope_parameters: Vec<ParameterId> = EnvelopeParameter::iter()
        .map(ParameterId::ParameterIdEnvelope)
        .collect();
    let lfo_parameters: Vec<ParameterId> = LFOParameter::iter()
        .flat_map(|parameter| {
            [
                ParameterId::ParameterIdLFO(0, parameter.clone()),
                ParameterId::ParameterIdLFO(1, parameter),
            ]
        })
        .collect();
    let quantization_parameters: Vec<ParameterId> = QuantizationParameter::iter()
        .map(ParameterId::ParameterIdQuantization)
        .collect();

    source_parameters
        .iter()
        .chain(envelope_parameters.iter())
        .chain(lfo_parameters.iter())
        .chain(quantization_parameters.iter())
        .cloned()
        .collect()
}

pub fn build_default_parameters() -> (FxHashMap<ParameterId, ParameterValue>, Vec<ParameterId>) {
    let parameter_ids = build_parameter_ids();

    let mut result = FxHashMap::default();
    parameter_ids
        .iter()
        .flat_map(|parameter_id| {
            let default_value = get_default_parameter_value(parameter_id);

            // Since there are two LFOs, we must create parameter ids for both of them
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
            let i_str = parameter_id.get_str("default").unwrap();
            let i = i32::from_str(i_str).unwrap();
            ParameterValue::Int(i.into())
        }
        _ => panic!("Invalid parameter declaration"),
    }
}

// MARK: Automatic conversion to/from usize for storage

#[derive(Debug, thiserror::Error)]
pub enum FromPrimitiveError {
    #[error("Failed to parse enum from primitive")]
    FailedToRead,
}

macro_rules! usize_conversion {
    ($target: ident) => {
        impl std::convert::TryFrom<usize> for $target {
            type Error = FromPrimitiveError;

            fn try_from(u: usize) -> Result<Self, Self::Error> {
                use num::FromPrimitive;
                if let Some(s) = Self::from_usize(u) {
                    Ok(s)
                } else {
                    Err(FromPrimitiveError::FailedToRead)
                }
            }
        }

        impl From<$target> for usize {
            fn from(t: $target) -> usize {
                use num::ToPrimitive;
                t.to_usize().unwrap()
            }
        }
    };
}

usize_conversion!(SourceParameter);
usize_conversion!(QuantizationParameter);
usize_conversion!(EnvelopeParameter);
usize_conversion!(LFOParameter);
