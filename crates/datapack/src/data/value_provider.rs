use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};
use ordered_float::NotNan;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum FloatProvider {
    Constant(NotNan<f32>),
    Uniform(UniformFloatProvider),
    ClampedNormal(ClampedNormalProvider),
    Trapezoid(TrapezoidFloatProvider),
}

impl<'de> Deserialize<'de> for FloatProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(DispatchDeserialize)]
        enum DispatchSurrogate {
            Constant(NotNan<f32>),
            Uniform(UniformFloatProvider),
            ClampedNormal(ClampedNormalProvider),
            Trapezoid(TrapezoidFloatProvider),
        }

        #[derive(UntaggedDeserialize)]
        enum Surrogate {
            Constant(NotNan<f32>),
            Provider(DispatchSurrogate),
        }

        Ok(match Surrogate::deserialize(deserializer)? {
            Surrogate::Constant(value) => FloatProvider::Constant(value),
            Surrogate::Provider(DispatchSurrogate::Constant(value)) => {
                FloatProvider::Constant(value)
            }
            Surrogate::Provider(DispatchSurrogate::Uniform(uniform)) => {
                FloatProvider::Uniform(uniform)
            }
            Surrogate::Provider(DispatchSurrogate::ClampedNormal(clamped_normal)) => {
                FloatProvider::ClampedNormal(clamped_normal)
            }
            Surrogate::Provider(DispatchSurrogate::Trapezoid(trapezoid)) => {
                FloatProvider::Trapezoid(trapezoid)
            }
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UniformFloatProvider {
    pub min_inclusive: NotNan<f32>,
    pub max_exclusive: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ClampedNormalProvider {
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
