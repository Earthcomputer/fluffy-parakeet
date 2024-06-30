use crate::data::SimpleWeightedListEntry;
use crate::serde_helpers::NonEmptyVec;
use datapack_macros::DispatchDeserialize;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use util::ranged::{value_too_big_error, value_too_small_error};

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum FloatProvider {
    #[dispatch(inlinable = "deserialize_constant_float")]
    Constant(ConstantFloatProvider),
    Uniform(UniformFloatProvider),
    ClampedNormal(ClampedNormalFloatProvider),
    Trapezoid(TrapezoidFloatProvider),
}

fn deserialize_constant_float<'de, D>(deserializer: D) -> Result<ConstantFloatProvider, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ConstantFloatProvider {
        value: Deserialize::deserialize(deserializer)?,
    })
}

impl FloatProvider {
    pub fn min_value(&self) -> f32 {
        match self {
            FloatProvider::Constant(provider) => provider.value,
            FloatProvider::Uniform(provider) => provider.min_inclusive,
            FloatProvider::ClampedNormal(provider) => provider.min,
            FloatProvider::Trapezoid(provider) => provider.min,
        }
    }

    pub fn max_value(&self) -> f32 {
        match self {
            FloatProvider::Constant(provider) => provider.value,
            FloatProvider::Uniform(provider) => provider.max_exclusive,
            FloatProvider::ClampedNormal(provider) => provider.max,
            FloatProvider::Trapezoid(provider) => provider.max,
        }
    }

    pub fn deserialize_ranged<'de, D>(
        deserializer: D,
        min: f32,
        max: f32,
    ) -> Result<FloatProvider, D::Error>
    where
        D: Deserializer<'de>,
    {
        let provider = FloatProvider::deserialize(deserializer)?;
        if provider.min_value() < min {
            return Err(value_too_small_error(
                Unexpected::Other("float provider out of range"),
                min,
            ));
        }
        if provider.max_value() > max {
            return Err(value_too_big_error(
                Unexpected::Other("float provider out of range"),
                max,
            ));
        }
        Ok(provider)
    }

    pub fn deserialize_non_negative<'de, D>(deserializer: D) -> Result<FloatProvider, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::deserialize_ranged(deserializer, 0.0, f32::INFINITY)
    }
}

#[macro_export]
macro_rules! float_provider_deserializer {
    ($name:ident, $min:expr, $max:expr) => {
        fn $name<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
            T: From<FloatProvider>,
        {
            Ok(T::from(FloatProvider::deserialize_ranged(
                deserializer,
                $min,
                $max,
            )?))
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct ConstantFloatProvider {
    pub value: f32,
}

#[derive(Debug, Deserialize)]
pub struct UniformFloatProvider {
    pub min_inclusive: f32,
    pub max_exclusive: f32,
}

#[derive(Debug, Deserialize)]
pub struct ClampedNormalFloatProvider {
    pub mean: f32,
    pub deviation: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Deserialize)]
pub struct TrapezoidFloatProvider {
    pub min: f32,
    pub max: f32,
    pub plateau: f32,
}

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum IntProvider {
    #[dispatch(inlinable = "deserialize_constant_int")]
    Constant(ConstantIntProvider),
    Uniform(UniformIntProvider),
    BiasedToBottom(BiasedToBottomIntProvider),
    Clamped(ClampedIntProvider),
    WeightedList(WeightedListIntProvider),
    ClampedNormal(ClampedNormalIntProvider),
}

fn deserialize_constant_int<'de, D>(deserializer: D) -> Result<ConstantIntProvider, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ConstantIntProvider {
        value: Deserialize::deserialize(deserializer)?,
    })
}

impl IntProvider {
    pub fn min_value(&self) -> i32 {
        match self {
            IntProvider::Constant(provider) => provider.value,
            IntProvider::Uniform(provider) => provider.min_inclusive,
            IntProvider::BiasedToBottom(provider) => provider.min_inclusive,
            IntProvider::Clamped(provider) => provider.min_inclusive,
            IntProvider::WeightedList(provider) => provider
                .distribution
                .iter()
                .map(|entry| entry.data.min_value())
                .min()
                .unwrap_or(i32::MAX),
            IntProvider::ClampedNormal(provider) => provider.min_inclusive,
        }
    }

    pub fn max_value(&self) -> i32 {
        match self {
            IntProvider::Constant(provider) => provider.value,
            IntProvider::Uniform(provider) => provider.max_inclusive,
            IntProvider::BiasedToBottom(provider) => provider.max_inclusive,
            IntProvider::Clamped(provider) => provider.max_inclusive,
            IntProvider::WeightedList(provider) => provider
                .distribution
                .iter()
                .map(|entry| entry.data.max_value())
                .max()
                .unwrap_or(i32::MIN),
            IntProvider::ClampedNormal(provider) => provider.max_inclusive,
        }
    }

    pub fn deserialize_ranged<'de, D>(
        deserializer: D,
        min: i32,
        max: i32,
    ) -> Result<IntProvider, D::Error>
    where
        D: Deserializer<'de>,
    {
        let provider = IntProvider::deserialize(deserializer)?;
        if provider.min_value() < min {
            return Err(value_too_small_error(
                Unexpected::Other("int provider out of range"),
                min,
            ));
        }
        if provider.max_value() > max {
            return Err(value_too_big_error(
                Unexpected::Other("int provider out of range"),
                max,
            ));
        }
        Ok(provider)
    }

    pub fn deserialize_non_negative<'de, D>(deserializer: D) -> Result<IntProvider, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::deserialize_ranged(deserializer, 0, i32::MAX)
    }

    pub fn deserialize_positive<'de, D>(deserializer: D) -> Result<IntProvider, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::deserialize_ranged(deserializer, 1, i32::MAX)
    }
}

#[macro_export]
macro_rules! int_provider_deserializer {
    ($name:ident, $min:expr, $max:expr) => {
        fn $name<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
            T: From<IntProvider>,
        {
            Ok(T::from(IntProvider::deserialize_ranged(
                deserializer,
                $min,
                $max,
            )?))
        }
    };
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
    pub mean: f32,
    pub deviation: f32,
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}
