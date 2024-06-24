use crate::sealed::Sealed;
use datapack::data::density_function::{
    BeardifierFunction, BlendAlphaFunction, BlendOffsetFunction, DensityFunction,
};
use datapack::{DataPack, DataPackResult};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub pos: BlockPos,
    pub datapack: &'a DataPack,
    pub blender: Option<Blender>,
}

pub trait DensityFunctionVisitor {
    fn apply<T>(&mut self, function: DensityFunction) -> DensityFunction;
    fn visit_noise(&mut self, noise: NoiseHolder);
}

trait DensityFunctionImpl: Sealed {
    fn compute(&self, context: &Context<'_>) -> DataPackResult<f64>;
    fn fill_slice(
        &self,
        slice: &mut [f64],
        contexts: impl Fn(usize) -> Context<'_>,
    ) -> DataPackResult<()>;
    fn min_value(&self) -> DataPackResult<f64>;
    fn max_value(&self) -> DataPackResult<f64>;
}

pub trait DensityFunctionEnumImpl: Sealed + DensityFunctionImpl {
    fn map_all<V>(&mut self, visitor: &mut V)
    where
        V: DensityFunctionVisitor;
}

macro_rules! impl_marker {
    (
        $($name:ident = $value:literal;)*
    ) => {
        $(
            impl DensityFunctionImpl for $name {
                fn compute(&self, context: &Context<'_>) -> DataPackResult<f64> {
                    panic!(concat!($name, " is a marker function and should not be called"));
                }

                fn fill_slice(&self, slice: &mut [f64], contexts: impl Fn(usize) -> Context<'_>) -> DataPackResult<()> {
                    slice.fill($value);
                    Ok(())
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

impl_marker! {
    BlendAlphaFunction = 1.0;
    BlendOffsetFunction = 0.0;
    BeardifierFunction = 0.0;
}

macro_rules! impl_wrapper {
    (
        $($name:ident),*$(,)?
    ) => {
        $(
            impl DensityFunctionImpl for $name {
                fn compute(&self, context: &Context<'_>) -> DataPackResult<f64> {
                    let argument = self.argument.resolve(context.datapack)?;
                    argument.compute(context)
                }

                fn fill_slice(&self, slice: &mut [f64], contexts: impl Fn(usize) -> Context<'_>) -> DataPackResult<()> {
                    let argument = self.argument.resolve(context.datapack)?;
                    argument.fill_slice(slice, contexts)
                }

                fn min_value(&self) -> DataPackResult<f64> {
                    let argument = self.argument.resolve(context.datapack)?;
                    argument.min_value()
                }

                fn max_value(&self) -> DataPackResult<f64> {
                    let argument = self.argument.resolve(context.datapack)?;
                    argument.max_value()
                }
            }
        )*
    };
}
