use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::{Deserialize, Deserializer};
use crate::data::SimpleWeightedListEntry;
use crate::serde_helpers::NonEmptyVec;

#[derive(Debug, DispatchDeserialize)]
pub enum FloatProvider {
    #[dispatch(inlinable = "deserialize_constant_float")]
    Constant(ConstantFloatProvider),
    Uniform(UniformFloatProvider),
    ClampedNormal(ClampedNormalFloatProvider),
    Trapezoid(TrapezoidFloatProvider),
}

fn deserialize_constant_float<'de, D>(deserializer: D) -> Result<ConstantFloatProvider, D::Error> where D: Deserializer<'de> {
    Ok(ConstantFloatProvider {
        value: Deserialize::deserialize(deserializer)?,
    })
}

#[derive(Debug, Deserialize)]
pub struct ConstantFloatProvider {
    pub value: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct UniformFloatProvider {
    pub min_inclusive: NotNan<f32>,
    pub max_exclusive: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ClampedNormalFloatProvider {
    pub mean: NotNan<f32>,
    pub deviation: NotNan<f32>,
    pub min: NotNan<f32>,
    pub max: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct TrapezoidFloatProvider {
    pub min: NotNan<f32>,
    pub max: NotNan<f32>,
    pub plateau: NotNan<f32>,
}

#[derive(Debug, DispatchDeserialize)]
pub enum IntProvider {
    #[dispatch(inlinable = "deserialize_constant_int")]
    Constant(ConstantIntProvider),
    Uniform(UniformIntProvider),
    BiasedToBottom(BiasedToBottomIntProvider),
    Clamped(ClampedIntProvider),
    WeightedList(WeightedListIntProvider),
    ClampedNormal(ClampedNormalIntProvider)
}

fn deserialize_constant_int<'de, D>(deserializer: D) -> Result<ConstantIntProvider, D::Error> where D: Deserializer<'de> {
    Ok(ConstantIntProvider {
        value: Deserialize::deserialize(deserializer)?,
    })
}

#[derive(Debug, Deserialize)]
pub struct ConstantIntProvider {
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct UniformIntProvider {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

#[derive(Debug, Deserialize)]
pub struct BiasedToBottomIntProvider {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

#[derive(Debug, Deserialize)]
pub struct ClampedIntProvider {
    pub source: Box<IntProvider>,
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

#[derive(Debug, Deserialize)]
pub struct WeightedListIntProvider {
    pub distribution: NonEmptyVec<SimpleWeightedListEntry<IntProvider>>,
}

#[derive(Debug, Deserialize)]
pub struct ClampedNormalIntProvider {
    pub mean: NotNan<f32>,
    pub deviation: NotNan<f32>,
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}
