use crate::data::biome::Biome;
use crate::data::biome_source::MultiNoiseBiomeSourceParameterList;
use crate::data::carvers::ConfiguredWorldCarver;
use crate::data::density_function::{DensityFunction, NoiseParameters};
use crate::data::feature::configured_feature::ConfiguredFeature;
use crate::data::feature::PlacedFeature;
use crate::data::noise::NoiseGeneratorSettings;
use crate::data::structure::set::StructureSet;
use crate::data::structure::Structure;
use crate::{DataPack, DataPackResult};
use datapack_macros::UntaggedDeserialize;
use serde::Serialize;
use util::add_only_map::AddOnlyMap;
use util::identifier::{Identifier, IdentifierBuf};

mod sealed {
    pub trait Sealed {}
}

pub trait RegistryType: sealed::Sealed + Sized {
    fn load(datapack: &DataPack, id: &Identifier) -> DataPackResult<Self>;
    #[allow(private_interfaces)]
    fn get_loaded_values(loaded_values: &RegistryLoadedValues) -> &AddOnlyMap<IdentifierBuf, Self>;
}

macro_rules! registries {
    ($($id:ident: $type:ty[$folder:literal];)*) => {
        $(
            impl sealed::Sealed for $type {}

            impl RegistryType for $type {
                fn load(datapack: &DataPack, id: &Identifier) -> DataPackResult<Self> {
                    datapack.read_json(id.to_datapack_path($folder, "json"))
                }

                #[allow(private_interfaces)]
                fn get_loaded_values(loaded_values: &RegistryLoadedValues) -> &AddOnlyMap<IdentifierBuf, Self> {
                    &loaded_values.$id
                }
            }
        )*

        #[derive(Debug, Default)]
        pub(crate) struct RegistryLoadedValues {
            $(
                $id: AddOnlyMap<IdentifierBuf, $type>,
            )*
        }
    };
}

registries! {
    biome: Biome["worldgen/biome"];
    configured_carver: ConfiguredWorldCarver["worldgen/configured_carver"];
    configured_feature: ConfiguredFeature["worldgen/configured_feature"];
    density_function: DensityFunction["worldgen/density_function"];
    multi_noise_biome_source_parameter_list: MultiNoiseBiomeSourceParameterList["worldgen/multi_noise_biome_source_parameter_list"];
    noise: NoiseParameters["worldgen/noise"];
    noise_settings: NoiseGeneratorSettings["worldgen/noise_settings"];
    placed_feature: PlacedFeature["worldgen/placed_feature"];
    structure: Structure["worldgen/structure"];
    structure_set: StructureSet["worldgen/structure_set"];
}

#[derive(Debug, UntaggedDeserialize, Serialize)]
#[serde(untagged)]
pub enum Holder<T> {
    Reference(IdentifierBuf),
    Direct(T),
}

impl<T> Holder<T>
where
    T: RegistryType,
{
    pub fn resolve<'a, 'b: 'a>(&'b self, datapack: &'b DataPack) -> DataPackResult<&'a T> {
        match self {
            Holder::Reference(id) => {
                let loaded_values = T::get_loaded_values(&datapack.registry_values);
                if let Some(value) = loaded_values.get(id) {
                    // fast path: value already loaded
                    Ok(value)
                } else {
                    loaded_values.get_or_try_insert(id.clone(), || T::load(datapack, id))
                }
            }
            Holder::Direct(value) => Ok(value),
        }
    }
}
