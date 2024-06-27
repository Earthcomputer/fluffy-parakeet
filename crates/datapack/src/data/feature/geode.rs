use ordered_float::NotNan;
use serde::Deserialize;
use util::identifier::IdentifierBuf;
use crate::data::tag::deserialize_hashed_tag;
use crate::data::block_state::BlockState;
use crate::data::block_state_provider::BlockStateProvider;
use crate::data::value_provider::{IntProvider, UniformIntProvider};
use crate::int_provider_deserializer;
use crate::serde_helpers::{DefaultOnError, DefaultToNum, DefaultToTrue, NonEmptyVec, Ranged, ValueProvider};

#[derive(Debug, Deserialize)]
pub struct GeodeConfiguration {
    pub blocks: GeodeBlockSettings,
    pub layers: GeodeLayerSettings,
    pub crack: GeodeCrackSettings,
    #[serde(default)]
    pub use_potential_placements_chance: DefaultOnError<Ranged<NotNan<f64>, 0, 1>, DefaultToNum<35, 100>>,
    #[serde(default)]
    pub use_alternate_layer0_chance: DefaultOnError<Ranged<NotNan<f64>, 0, 1>>,
    #[serde(default)]
    pub placements_require_layer0_alternate: DefaultOnError<bool, DefaultToTrue>,
    #[serde(deserialize_with = "one_twenty_provider")]
    #[serde(default)]
    pub outer_wall_distance: DefaultOnError<IntProvider, OuterWallDistanceDefault>,
    #[serde(deserialize_with = "one_twenty_provider")]
    #[serde(default)]
    pub distribution_points: DefaultOnError<IntProvider, DistributionPointsDefault>,
    #[serde(deserialize_with = "zero_ten_provider")]
    #[serde(default)]
    pub point_offset: DefaultOnError<IntProvider, PointOffsetDefault>,
    #[serde(default)]
    pub min_gen_offset: DefaultOnError<i32, DefaultToNum<-16>>,
    #[serde(default)]
    pub max_gen_offset: DefaultOnError<i32, DefaultToNum<16>>,
    #[serde(default)]
    pub noise_multiplier: DefaultOnError<Ranged<NotNan<f64>, 0, 1>, DefaultToNum<5, 100>>,
    pub invalid_blocks_threshold: i32,
}

int_provider_deserializer!(one_twenty_provider, 1, 20);
int_provider_deserializer!(zero_ten_provider, 0, 10);

pub struct OuterWallDistanceDefault;
impl ValueProvider<IntProvider> for OuterWallDistanceDefault {
    fn provide() -> IntProvider {
        IntProvider::Uniform(UniformIntProvider {
            min_inclusive: 4,
            max_inclusive: 5,
        })
    }
}

pub struct DistributionPointsDefault;
impl ValueProvider<IntProvider> for DistributionPointsDefault {
    fn provide() -> IntProvider {
        IntProvider::Uniform(UniformIntProvider {
            min_inclusive: 3,
            max_inclusive: 4,
        })
    }
}

pub struct PointOffsetDefault;
impl ValueProvider<IntProvider> for PointOffsetDefault {
    fn provide() -> IntProvider {
        IntProvider::Uniform(UniformIntProvider {
            min_inclusive: 1,
            max_inclusive: 2,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct GeodeBlockSettings {
    pub filling_provider: BlockStateProvider,
    pub inner_layer_provider: BlockStateProvider,
    pub alternate_inner_layer_provider: BlockStateProvider,
    pub middle_layer_provider: BlockStateProvider,
    pub outer_layer_provider: BlockStateProvider,
    pub inner_placements: NonEmptyVec<BlockState>,
    #[serde(deserialize_with = "deserialize_hashed_tag")]
    pub cannot_replace: IdentifierBuf,
    #[serde(deserialize_with = "deserialize_hashed_tag")]
    pub invalid_blocks: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct GeodeLayerSettings {
    #[serde(default)]
    pub filling: DefaultOnError<Ranged<NotNan<f64>, 1, 5000, 100>, DefaultToNum<17, 10>>,
    #[serde(default)]
    pub inner_layer: DefaultOnError<Ranged<NotNan<f64>, 1, 5000, 100>, DefaultToNum<22, 10>>,
    #[serde(default)]
    pub middle_layer: DefaultOnError<Ranged<NotNan<f64>, 1, 5000, 100>, DefaultToNum<32, 10>>,
    #[serde(default)]
    pub outer_layer: DefaultOnError<Ranged<NotNan<f64>, 1, 5000, 100>, DefaultToNum<42, 10>>,
}

#[derive(Debug, Deserialize)]
pub struct GeodeCrackSettings {
    #[serde(default)]
    pub generate_crack_chance: DefaultOnError<Ranged<NotNan<f64>, 0, 1>, DefaultToNum<1>>,
    #[serde(default)]
    pub base_crack_size: DefaultOnError<Ranged<NotNan<f64>, 0, 5>, DefaultToNum<2>>,
    #[serde(default)]
    pub crack_point_offset: DefaultOnError<Ranged<u32, 0, 10>, DefaultToNum<2>>,
}
