use crate::data::holder::Holder;
use crate::data::{DIMENSION_MAX_Y, DIMENSION_MIN_Y};
use crate::serde_helpers::{NonEmptyVec, Ranged};
use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};
use ordered_float::NotNan;
use serde::{Deserialize, Deserializer};

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum DensityFunction {
    BlendAlpha(BlendAlphaFunction),
    BlendOffset(BlendOffsetFunction),
    Beardifier(BeardifierFunction),
    OldBlendedNoise(BlendedNoiseFunction),
    Interpolated(InterpolatedFunction),
    FlatCache(FlatCacheFunction),
    #[dispatch(rename = "cache_2d")]
    Cache2d(Cache2dFunction),
    CacheOnce(CacheOnceFunction),
    CacheAllInCell(CacheAllInCellFunction),
    Noise(NoiseFunction),
    EndIslands(EndIslandsFunction),
    WeirdScaledSampler(WeirdScaledSamplerFunction),
    ShiftedNoise(ShiftedNoiseFunction),
    RangeChoice(RangeChoiceFunction),
    ShiftA(ShiftAFunction),
    ShiftB(ShiftBFunction),
    Shift(ShiftFunction),
    BlendDensity(BlendDensityFunction),
    Clamp(ClampFunction),
    Abs(AbsFunction),
    Square(SquareFunction),
    Cube(CubeFunction),
    HalfNegative(HalfNegativeFunction),
    QuarterNegative(QuarterNegativeFunction),
    Squeeze(SqueezeFunction),
    Add(AddFunction),
    Mul(MulFunction),
    Min(MinFunction),
    Max(MaxFunction),
    Spline(SplineFunction),
    #[dispatch(inlinable = "deserialize_constant")]
    Constant(ConstantFunction),
    YClampedGradient(YClampedGradientFunction),
}

fn deserialize_constant<'de, D>(deserializer: D) -> Result<ConstantFunction, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ConstantFunction {
        argument: Deserialize::deserialize(deserializer)?,
    })
}

pub type NoiseValue = Ranged<NotNan<f64>, -1000000, 1000000>;

#[derive(Debug, Deserialize)]
pub struct BlendAlphaFunction {}

#[derive(Debug, Deserialize)]
pub struct BlendOffsetFunction {}

#[derive(Debug, Deserialize)]
pub struct BeardifierFunction {}

#[derive(Debug, Deserialize)]
pub struct BlendedNoiseFunction {
    pub xz_scale: Ranged<NotNan<f64>, 1, 1000000, 1000>,
    pub y_scale: Ranged<NotNan<f64>, 1, 1000000, 1000>,
    pub xz_factor: Ranged<NotNan<f64>, 1, 1000000, 1000>,
    pub y_factor: Ranged<NotNan<f64>, 1, 1000000, 1000>,
    pub smear_scale_multiplier: Ranged<NotNan<f64>, 1, 8>,
}

#[derive(Debug, Deserialize)]
pub struct InterpolatedFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct FlatCacheFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct Cache2dFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct CacheOnceFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct CacheAllInCellFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseFunction {
    pub noise: Holder<NoiseParameters>,
    pub xz_scale: NotNan<f64>,
    pub y_scale: NotNan<f64>,
}

#[derive(Debug, Deserialize)]
pub struct EndIslandsFunction {}

#[derive(Debug, Deserialize)]
pub struct WeirdScaledSamplerFunction {
    pub input: Box<Holder<DensityFunction>>,
    pub noise: Holder<NoiseParameters>,
    pub rarity_value_mapper: RarityValueMapper,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum RarityValueMapper {
    #[serde(rename = "type_1")]
    Type1,
    #[serde(rename = "type_2")]
    Type2,
}

#[derive(Debug, Deserialize)]
pub struct ShiftedNoiseFunction {
    pub shift_x: Box<Holder<DensityFunction>>,
    pub shift_y: Box<Holder<DensityFunction>>,
    pub shift_z: Box<Holder<DensityFunction>>,
    pub xz_scale: NotNan<f64>,
    pub y_scale: NotNan<f64>,
    pub noise: Holder<NoiseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct RangeChoiceFunction {
    pub input: Box<Holder<DensityFunction>>,
    pub min_inclusive: NoiseValue,
    pub max_exclusive: NoiseValue,
    pub when_in_range: Box<Holder<DensityFunction>>,
    pub when_out_of_range: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct ShiftAFunction {
    pub argument: Holder<NoiseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct ShiftBFunction {
    pub argument: Holder<NoiseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct ShiftFunction {
    pub argument: Holder<NoiseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct BlendDensityFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct ClampFunction {
    pub input: Box<DensityFunction>,
    pub min: NoiseValue,
    pub max: NoiseValue,
}

#[derive(Debug, Deserialize)]
pub struct AbsFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct SquareFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct CubeFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct HalfNegativeFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct QuarterNegativeFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct SqueezeFunction {
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct AddFunction {
    pub argument1: Box<Holder<DensityFunction>>,
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct MulFunction {
    pub argument1: Box<Holder<DensityFunction>>,
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct MinFunction {
    pub argument1: Box<Holder<DensityFunction>>,
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct MaxFunction {
    pub argument1: Box<Holder<DensityFunction>>,
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct SplineFunction {
    pub spline: CubicSpline,
}

#[derive(Debug, UntaggedDeserialize)]
pub enum CubicSpline {
    Constant(NotNan<f32>),
    Multipoint {
        coordinate: Box<Holder<DensityFunction>>,
        points: NonEmptyVec<SplinePoint>,
    },
}

#[derive(Debug, Deserialize)]
pub struct SplinePoint {
    pub location: NotNan<f32>,
    pub value: CubicSpline,
    pub derivative: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ConstantFunction {
    argument: NoiseValue,
}

#[derive(Debug, Deserialize)]
pub struct YClampedGradientFunction {
    pub from_y: Ranged<i32, { (DIMENSION_MIN_Y * 2) as i64 }, { (DIMENSION_MAX_Y * 2) as i64 }>,
    pub to_y: Ranged<i32, { (DIMENSION_MIN_Y * 2) as i64 }, { (DIMENSION_MAX_Y * 2) as i64 }>,
    pub from_value: NoiseValue,
    pub to_value: NoiseValue,
}

#[derive(Debug, Deserialize)]
pub struct NoiseParameters {
    #[serde(rename = "firstOctave")]
    pub first_octave: i32,
    pub amplitudes: Vec<NotNan<f64>>,
}
