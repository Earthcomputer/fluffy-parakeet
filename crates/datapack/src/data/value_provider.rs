use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug, DispatchDeserialize)]
pub enum FloatProvider {
    #[dispatch(inlinable)]
    Constant(NotNan<f32>),
    Uniform(UniformFloatProvider),
    ClampedNormal(ClampedNormalProvider),
    Trapezoid(TrapezoidFloatProvider),
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
