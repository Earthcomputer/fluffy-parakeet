use std::ops::Range;
use ordered_float::NotNan;
use serde::Deserialize;
use datapack_macros::DispatchDeserialize;
use crate::data::biome::GenerationStepCarving;
use crate::data::height_provider::HeightProvider;
use crate::serde_helpers::{DefaultOnError, Ranged, RangedPositiveU32};

#[derive(Debug, DispatchDeserialize)]
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
    pub step: GenerationStepCarving,
}

#[derive(Debug, Deserialize)]
pub struct CountLikePlacement {
    pub count: Ranged<i32, 0, 256>,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentScanPlacement {
    pub direction_of_search: VerticalDirection,
    pub target_condition: BlockPredicate,
    #[serde(default = "block_predicate::always_true")]
    pub allowed_search_condition: BlockPredicate,
    pub max_steps: Ranged<i32, 1, 32>,
}

#[derive(Debug, Deserialize)]
pub struct FixedPlacement {
    pub positions: Vec<BlockPos>,
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
    pub noise_factor: NotNan<f64>,
    #[serde(default)]
    pub noise_offset: DefaultOnError<NotNan<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseThresholdCountPlacement {
    pub noise_level: NotNan<f64>,
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
    pub chance: RangedPositiveU32,
}

#[derive(Debug, Deserialize)]
pub struct SurfaceRelativeThresholdFilter {
    pub heightmap: HeightmapType,
    #[serde(default = "min_i32")]
    pub min_inclusive: i32,
    #[serde(default = "max_i32")],
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