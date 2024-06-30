use crate::data::block_predicate::BlockPredicate;
use crate::data::height_provider::HeightProvider;
use crate::data::step::CarvingStep;
use crate::serde_helpers::DefaultOnError;
use datapack_macros::DispatchDeserialize;
use glam::IVec3;

use serde::Deserialize;
use util::direction::Direction;
use util::heightmap_type::HeightmapType;
use util::ranged::{PositiveI32, Ranged};

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum PlacementModifier {
    BiomeFilter(BiomeFilter),
    BlockPredicateFilter(BlockPredicateFilter),
    CarvingMaskPlacement(CarvingMaskPlacement),
    CountOnEveryLayerPlacement(CountLikePlacement),
    CountPlacement(CountLikePlacement),
    EnvironmentScanPlacement(EnvironmentScanPlacement),
    FixedPlacement(FixedPlacement),
    HeightmapPlacement(HeightmapPlacement),
    HeightRangePlacement(HeightRangePlacement),
    InSquarePlacement(InSquarePlacement),
    NoiseBasedCountPlacement(NoiseBasedCountPlacement),
    RandomOffsetPlacement(RandomOffsetPlacement),
    RarityFilter(RarityFilter),
    SurfaceRelativeThresholdFilter(SurfaceRelativeThresholdFilter),
    SurfaceWaterDepthFilter(SurfaceWaterDepthFilter),
}

#[derive(Debug, Deserialize)]
pub struct BiomeFilter {}

#[derive(Debug, Deserialize)]
pub struct BlockPredicateFilter {
    pub predicate: BlockPredicate,
}

#[derive(Debug, Deserialize)]
pub struct CarvingMaskPlacement {
    pub step: CarvingStep,
}

#[derive(Debug, Deserialize)]
pub struct CountLikePlacement {
    pub count: Ranged<i32, 0, 256>,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentScanPlacement {
    #[serde(deserialize_with = "Direction::deserialize_horizontal")]
    pub direction_of_search: Direction,
    pub target_condition: BlockPredicate,
    #[serde(default = "BlockPredicate::always_true")]
    pub allowed_search_condition: BlockPredicate,
    pub max_steps: Ranged<i32, 1, 32>,
}

#[derive(Debug, Deserialize)]
pub struct FixedPlacement {
    pub positions: Vec<IVec3>,
}

#[derive(Debug, Deserialize)]
pub struct HeightmapPlacement {
    pub heightmap: HeightmapType,
}

#[derive(Debug, Deserialize)]
pub struct HeightRangePlacement {
    pub height: HeightProvider,
}

#[derive(Debug, Deserialize)]
pub struct InSquarePlacement {}

#[derive(Debug, Deserialize)]
pub struct NoiseBasedCountPlacement {
    pub noise_to_count_ratio: i32,
    pub noise_factor: f64,
    #[serde(default)]
    pub noise_offset: DefaultOnError<f64>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseThresholdCountPlacement {
    pub noise_level: f64,
    pub below_noise: i32,
    pub above_noise: i32,
}

#[derive(Debug, Deserialize)]
pub struct RandomOffsetPlacement {
    pub xz_spread: Ranged<i32, -16, 16>,
    pub y_spread: Ranged<i32, -16, 16>,
}

#[derive(Debug, Deserialize)]
pub struct RarityFilter {
    pub chance: PositiveI32,
}

#[derive(Debug, Deserialize)]
pub struct SurfaceRelativeThresholdFilter {
    pub heightmap: HeightmapType,
    #[serde(default = "min_i32")]
    pub min_inclusive: i32,
    #[serde(default = "max_i32")]
    pub max_inclusive: i32,
}

fn min_i32() -> i32 {
    i32::MIN
}

fn max_i32() -> i32 {
    i32::MAX
}

#[derive(Debug, Deserialize)]
pub struct SurfaceWaterDepthFilter {
    pub max_water_depth: i32,
}
