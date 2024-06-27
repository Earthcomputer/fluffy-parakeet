use crate::serde_helpers::NonNegativeU32;
use datapack_macros::UntaggedDeserialize;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;

const WORLD_BORDER: i32 = 30000000;
const BITS_FOR_XZ: u32 = WORLD_BORDER.ilog2() + 2;
const BITS_FOR_Y: u32 = 64 - BITS_FOR_XZ * 2;
pub const DIMENSION_Y_SIZE: u32 = (1 << BITS_FOR_Y) - 32;
pub const DIMENSION_MAX_Y: i32 = (DIMENSION_Y_SIZE >> 1) as i32 - 1;
pub const DIMENSION_MIN_Y: i32 = DIMENSION_MAX_Y - DIMENSION_Y_SIZE as i32 + 1;

pub mod biome;
pub mod biome_source;
pub mod block_predicate;
pub mod block_state;
pub mod block_state_provider;
pub mod carvers;
pub mod density_function;
pub mod feature;
pub mod flat;
pub mod height_provider;
pub mod holder;
pub mod noise;
pub mod sound_event;
pub mod surface_rules;
pub mod tag;
pub mod value_provider;
pub mod world_preset;

#[derive(Debug)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<'de, T> Deserialize<'de> for Interval<T>
where
    T: Deserialize<'de> + Ord + Clone + Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(UntaggedDeserialize)]
        enum Surrogate<T> {
            Array([T; 2]),
            Named { min: T, max: T },
            Single(T),
        }
        let interval = match Surrogate::<T>::deserialize(deserializer)? {
            Surrogate::Array([min, max]) => Interval { min, max },
            Surrogate::Named { min, max } => Interval { min, max },
            Surrogate::Single(both) => Interval {
                min: both.clone(),
                max: both,
            },
        };
        if interval.min > interval.max {
            return Err(serde::de::Error::custom(format!(
                "cannot construct interval ({:?} > {:?})",
                interval.min, interval.max
            )));
        }
        Ok(interval)
    }
}

#[derive(Debug, Deserialize)]
pub struct SimpleWeightedListEntry<T> {
    pub data: T,
    pub weight: NonNegativeU32,
}
