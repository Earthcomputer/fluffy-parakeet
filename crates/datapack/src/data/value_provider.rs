use crate::data::SimpleWeightedListEntry;
use crate::serde_helpers::{value_too_big_error, value_too_small_error, NonEmptyVec};
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;

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
    pub fn min_value(&self) -> NotNan<f32> {
        match self {
            FloatProvider::Constant(provider) => provider.value,
            FloatProvider::Uniform(provider) => provider.min_inclusive,
            FloatProvider::ClampedNormal(provider) => provider.min,
            FloatProvider::Trapezoid(provider) => provider.min,
        }
    }

    pub fn max_value(&self) -> NotNan<f32> {
        match self {
            FloatProvider::Constant(provider) => provider.value,
            FloatProvider::Uniform(provider) => provider.max_exclusive,
            FloatProvider::ClampedNormal(provider) => provider.max,
            FloatProvider::Trapezoid(provider) => provider.max,
        }
    }

    pub fn deserialize_ranged<'de, D, A, B>(
        deserializer: D,
        min: A,
        max: B,
    ) -> Result<FloatProvider, D::Error>
    where
        D: Deserializer<'de>,
        A: TryInto<NotNan<f32>>,
        B: TryInto<NotNan<f32>>,
        <A as TryInto<NotNan<f32>>>::Error: Debug,
        <B as TryInto<NotNan<f32>>>::Error: Debug,
    {
        let min = min.try_into().unwrap();
        let max = max.try_into().unwrap();
        let provider = FloatProvider::deserialize(deserializer)?;
        if provider.min_value() < min {
            return Err(value_too_small_error(min));
        }
        if provider.max_value() > max {
            return Err(value_too_big_error(max));
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
            return Err(value_too_small_error(min));
        }
        if provider.max_value() > max {
            return Err(value_too_big_error(max));
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
    pub mean: NotNan<f32>,
    pub deviation: NotNan<f32>,
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}
