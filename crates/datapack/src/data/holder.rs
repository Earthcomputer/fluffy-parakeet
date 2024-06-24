use crate::data::biome::Biome;
use crate::data::biome_source::MultiNoiseBiomeSourceParameterList;
use crate::data::carvers::ConfiguredWorldCarver;
use crate::data::density_function::{DensityFunction, NoiseParameters};
use crate::data::noise::NoiseGeneratorSettings;
use crate::identifier::{Identifier, IdentifierBuf};
use crate::{DataPack, DataPackResult};
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use datapack_macros::UntaggedDeserialize;
use serde::Serialize;
use std::fmt::{Debug, Formatter};
use std::mem;

/// # Invariants
/// The following invariants hold until after the start of the Drop function:
/// 1. Values in the map are pointers to a value on the heap created by `Box::leak(Box::new())`
/// 2. The preconditions for pointer dereference hold for all values in the map
/// 3. There are no mutable references to values in the map
struct RegistryMap<T>(DashMap<IdentifierBuf, *mut T, ahash::RandomState>);

impl<T> RegistryMap<T> {
    fn get<'a>(&self, key: &Identifier) -> Option<&T> {
        // SAFETY: pointer can be dereferenced due to struct invariant 2
        self.0.get(key).map(|value| unsafe { &**value })
    }

    fn get_or_try_insert<E>(
        &self,
        key: IdentifierBuf,
        new_value: impl FnOnce() -> Result<T, E>,
    ) -> Result<&T, E> {
        match self.0.entry(key) {
            Entry::Occupied(entry) => {
                // SAFETY: pointer can be dereferenced due to struct invariant 2
                Ok(unsafe { &**entry.get() })
            }
            Entry::Vacant(entry) => {
                // create a value according to struct invariant 1
                let value = Box::leak(Box::new(new_value()?)) as *mut T;
                entry.insert(value);
                // SAFETY: we just created this value from a box, so it's valid to dereference.
                // Creating an immutable reference in accordance with struct invariant 3
                Ok(unsafe { &*value })
            }
        }
    }
}

impl<T> Drop for RegistryMap<T> {
    fn drop(&mut self) {
        for (_, value) in mem::take(&mut self.0) {
            // SAFETY: value is still a valid allocation that was created from Box::leak(Box::new()) (struct invariant 1)
            let value = unsafe { Box::from_raw(value) };
            drop(value);
        }
    }
}

impl<T> Debug for RegistryMap<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_map = f.debug_map();
        for entry in self.0.iter() {
            // SAFETY: pointer can be dereferenced due to struct invariant 2
            debug_map.entry(entry.key(), unsafe { &**entry.value() });
        }
        debug_map.finish()
    }
}

impl<T> Default for RegistryMap<T> {
    fn default() -> Self {
        Self(DashMap::default())
    }
}

trait RegistryType: Sized {
    fn load(datapack: &DataPack, id: &Identifier) -> DataPackResult<Self>;
    fn get_loaded_values(loaded_values: &RegistryLoadedValues) -> &RegistryMap<Self>;
}

macro_rules! registries {
    ($($id:ident: $type:ty[$folder:literal];)*) => {
        $(
            impl RegistryType for $type {
                fn load(datapack: &DataPack, id: &Identifier) -> DataPackResult<Self> {
                    datapack.read_json(id.to_datapack_path($folder, "json"))
                }

                fn get_loaded_values(loaded_values: &RegistryLoadedValues) -> &RegistryMap<Self> {
                    &loaded_values.$id
                }
            }
        )*

        #[derive(Debug, Default)]
        pub(crate) struct RegistryLoadedValues {
            $(
                $id: RegistryMap<$type>,
            )*
        }
    };
}

registries! {
    biome: Biome["worldgen/biome"];
    configured_carver: ConfiguredWorldCarver["worldgen/configured_carver"];
    density_function: DensityFunction["worldgen/density_function"];
    multi_noise_biome_source_parameter_list: MultiNoiseBiomeSourceParameterList["worldgen/multi_noise_biome_source_parameter_list"];
    noise: NoiseParameters["worldgen/noise"];
    noise_settings: NoiseGeneratorSettings["worldgen/noise_settings"];
}

#[derive(Debug, UntaggedDeserialize, Serialize)]
#[serde(untagged)]
pub enum Holder<T> {
    Reference(IdentifierBuf),
    Direct(T),
}

#[allow(private_bounds)]
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
