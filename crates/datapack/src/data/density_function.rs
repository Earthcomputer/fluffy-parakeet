use crate::data::holder::Holder;
use crate::data::{DIMENSION_MAX_Y, DIMENSION_MIN_Y};
use crate::identifier::IdentifierBuf;
use crate::serde_helpers::{NonEmptyVec, Ranged};
use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};
use ordered_float::NotNan;
use serde::{Deserialize, Deserializer};

#[derive(Debug, DispatchDeserialize)]
pub enum DensityFunction {
    BlendAlpha(BlendAlphaFunction),
    BlendOffset(BlendOffsetFunction),
    Beardifier(BeardifierFunction),
    OldBlendedNoise(BlendedNoiseFunction),
    Interpolated(InterpolatedFunction),
    FlatCache(FlatCacheFunction),
    #[allow(non_camel_case_types)]
    Cache_2d(Cache2dFunction),
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
    Constant(ConstantFunction),
    YClampedGradient(YClampedGradientFunction),
}

pub fn deserialize_density_function_boxed<'de, D>(
    deserializer: D,
) -> Result<Box<DensityFunction>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(UntaggedDeserialize)]
    enum Surrogate {
        Constant(NoiseValue),
        Function(DensityFunction),
    }
    match Surrogate::deserialize(deserializer)? {
        Surrogate::Constant(constant) => {
            Ok(Box::new(DensityFunction::Constant(ConstantFunction {
                argument: constant,
            })))
        }
        Surrogate::Function(function) => Ok(Box::new(function)),
    }
}

pub fn deserialize_density_function_holder<'de, D>(
    deserializer: D,
) -> Result<Holder<DensityFunction>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(UntaggedDeserialize)]
    enum Surrogate {
        Reference(IdentifierBuf),
        Constant(NoiseValue),
        Function(DensityFunction),
    }
    match Surrogate::deserialize(deserializer)? {
        Surrogate::Reference(id) => Ok(Holder::Reference(id)),
        Surrogate::Constant(constant) => Ok(Holder::Direct(DensityFunction::Constant(
            ConstantFunction { argument: constant },
        ))),
        Surrogate::Function(function) => Ok(Holder::Direct(function)),
    }
}

pub fn deserialize_density_function_holder_boxed<'de, D>(
    deserializer: D,
) -> Result<Box<Holder<DensityFunction>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(UntaggedDeserialize)]
    enum Surrogate {
        Reference(IdentifierBuf),
        Constant(NoiseValue),
        Function(DensityFunction),
    }
    match Surrogate::deserialize(deserializer)? {
        Surrogate::Reference(id) => Ok(Box::new(Holder::Reference(id))),
        Surrogate::Constant(constant) => Ok(Box::new(Holder::Direct(DensityFunction::Constant(
            ConstantFunction { argument: constant },
        )))),
        Surrogate::Function(function) => Ok(Box::new(Holder::Direct(function))),
    }
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
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct FlatCacheFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct Cache2dFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct CacheOnceFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct CacheAllInCellFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
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
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub input: Box<Holder<DensityFunction>>,
    pub noise: Holder<NoiseParameters>,
    pub rarity_value_mapper: RarityValueMapper,
}

#[derive(Debug, Deserialize)]
pub enum RarityValueMapper {
    #[serde(rename = "type_1")]
    Type1,
    #[serde(rename = "type_2")]
    Type2,
}

#[derive(Debug, Deserialize)]
pub struct ShiftedNoiseFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub shift_x: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub shift_y: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub shift_z: Box<Holder<DensityFunction>>,
    pub xz_scale: NotNan<f64>,
    pub y_scale: NotNan<f64>,
    pub noise: Holder<NoiseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct RangeChoiceFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub input: Box<Holder<DensityFunction>>,
    pub min_inclusive: NoiseValue,
    pub max_exclusive: NoiseValue,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub when_in_range: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
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
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct ClampFunction {
    #[serde(deserialize_with = "deserialize_density_function_boxed")]
    pub input: Box<DensityFunction>,
    pub min: NoiseValue,
    pub max: NoiseValue,
}

#[derive(Debug, Deserialize)]
pub struct AbsFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct SquareFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct CubeFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct HalfNegativeFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct QuarterNegativeFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct SqueezeFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct AddFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument1: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct MulFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument1: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct MinFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument1: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument2: Box<Holder<DensityFunction>>,
}

#[derive(Debug, Deserialize)]
pub struct MaxFunction {
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
    pub argument1: Box<Holder<DensityFunction>>,
    #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
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
        #[serde(deserialize_with = "deserialize_density_function_holder_boxed")]
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
