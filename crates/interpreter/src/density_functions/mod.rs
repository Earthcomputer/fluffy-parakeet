use crate::sealed::Sealed;
use datapack::data::density_function::{
    AbsFunction, AddFunction, BeardifierFunction, BlendAlphaFunction, BlendDensityFunction,
    BlendOffsetFunction, BlendedNoiseFunction, Cache2dFunction, CacheAllInCellFunction,
    CacheOnceFunction, ClampFunction, ConstantFunction, CubeFunction, DensityFunction,
    EndIslandsFunction, FlatCacheFunction, HalfNegativeFunction, InterpolatedFunction, MaxFunction,
    MinFunction, MulFunction, NoiseFunction, QuarterNegativeFunction, RangeChoiceFunction,
    ShiftAFunction, ShiftBFunction, ShiftFunction, ShiftedNoiseFunction, SplineFunction,
    SquareFunction, SqueezeFunction, WeirdScaledSamplerFunction, YClampedGradientFunction,
};
use datapack::DataPackResult;

pub trait DensityFunctionExt: Sealed {
    fn compute<I>(&self, interpreter: &I) -> DataPackResult<f64>
    where
        I: Interpreter;
    fn min_value(&self) -> DataPackResult<f64>;
    fn max_value(&self) -> DataPackResult<f64>;
}

macro_rules! define_marker_ext {
    (
        $($ty:ident $inter_fn:ident $value:literal);*$(;)?
    ) => {
        $(
            impl Sealed for $ty {}

            impl DensityFunctionExt for $ty {
                fn compute<I>(&self, interpreter: &I) -> DataPackResult<f64>
                where
                    I: Interpreter
                {
                    interpreter.$inter_fn(self)
                }

                fn min_value(&self) -> DataPackResult<f64> {
                    Ok($value)
                }

                fn max_value(&self) -> DataPackResult<f64> {
                    Ok($value)
                }
            }
        )*
    };
}

define_marker_ext! {
    BlendAlphaFunction handle_blend_alpha 1.0;
    BlendOffsetFunction handle_blend_offset 0.0;
    BeardifierFunction handle_beardifier 0.0;
}

macro_rules! define_wrapper_ext {
    (
        $($ty:ident $inter_fn:ident);*$(;)?
    ) => {
        $(
            impl Sealed for $ty {}

            impl DensityFunctionExt for $ty {
                fn compute<I>(&self, interpreter: &I) -> DataPackResult<f64>
                where
                    I: Interpreter
                {
                    interpreter.$inter_fn(self)
                }

                fn min_value(&self) -> DataPackResult<f64> {
                    todo!()
                }
            }
        )*
    };
}

// BlendedNoise

pub trait Interpreter {
    fn handle_blend_alpha(&self, function: &BlendAlphaFunction) -> DataPackResult<f64>;
    fn handle_blend_offset(&self, function: &BlendOffsetFunction) -> DataPackResult<f64>;
    fn handle_beardifier(&self, function: &BeardifierFunction) -> DataPackResult<f64>;
    fn handle_old_blended_noise(&self, function: &BlendedNoiseFunction) -> DataPackResult<f64>;
    fn handle_interpolated(&self, function: &InterpolatedFunction) -> DataPackResult<f64>;
    fn handle_flat_cache(&self, function: &FlatCacheFunction) -> DataPackResult<f64>;
    fn handle_cache_2d(&self, function: &Cache2dFunction) -> DataPackResult<f64>;
    fn handle_cache_once(&self, function: &CacheOnceFunction) -> DataPackResult<f64>;
    fn handle_cache_all_in_cell(&self, function: &CacheAllInCellFunction) -> DataPackResult<f64>;
    fn handle_noise(&self, function: &NoiseFunction) -> DataPackResult<f64>;
    fn handle_end_islands(&self, function: &EndIslandsFunction) -> DataPackResult<f64>;
    fn handle_weird_scaled_sampler(
        &self,
        function: &WeirdScaledSamplerFunction,
    ) -> DataPackResult<f64>;
    fn handle_shifted_noise(&self, function: &ShiftedNoiseFunction) -> DataPackResult<f64>;
    fn handle_range_choice(&self, function: &RangeChoiceFunction) -> DataPackResult<f64>;
    fn handle_shift_a(&self, function: &ShiftAFunction) -> DataPackResult<f64>;
    fn handle_shift_b(&self, function: &ShiftBFunction) -> DataPackResult<f64>;
    fn handle_shift(&self, function: &ShiftFunction) -> DataPackResult<f64>;
    fn handle_blend_density(&self, function: &BlendDensityFunction) -> DataPackResult<f64>;
    fn handle_clamp(&self, function: &ClampFunction) -> DataPackResult<f64>;
    fn handle_abs(&self, function: &AbsFunction) -> DataPackResult<f64>;
    fn handle_square(&self, function: &SquareFunction) -> DataPackResult<f64>;
    fn handle_cube(&self, function: &CubeFunction) -> DataPackResult<f64>;
    fn handle_half_negative(&self, function: &HalfNegativeFunction) -> DataPackResult<f64>;
    fn handle_quarter_negative(&self, function: &QuarterNegativeFunction) -> DataPackResult<f64>;
    fn handle_squeeze(&self, function: &SqueezeFunction) -> DataPackResult<f64>;
    fn handle_add(&self, function: &AddFunction) -> DataPackResult<f64>;
    fn handle_mul(&self, function: &MulFunction) -> DataPackResult<f64>;
    fn handle_min(&self, function: &MinFunction) -> DataPackResult<f64>;
    fn handle_max(&self, function: &MaxFunction) -> DataPackResult<f64>;
    fn handle_spline(&self, function: &SplineFunction) -> DataPackResult<f64>;
    fn handle_constant(&self, function: &ConstantFunction) -> DataPackResult<f64>;
    fn handle_y_clamped_gradient(&self, function: &YClampedGradientFunction)
        -> DataPackResult<f64>;
}
