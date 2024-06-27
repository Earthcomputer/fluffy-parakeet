use crate::built_in_registries::Block;
use crate::data::block_state_provider::BlockStateProvider;
use crate::data::feature::feature_size::FeatureSize;
use crate::data::tag::HolderSet;
use crate::data::value_provider::IntProvider;
use crate::int_provider_deserializer;
use crate::serde_helpers::{DefaultOnError, NonEmptyVec, PositiveU32, Ranged};
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer};
use util::direction::Direction;

#[derive(Debug, Deserialize)]
pub struct TreeConfiguration {
    pub trunk_provider: BlockStateProvider,
    pub trunk_placer: TrunkPlacer,
    pub foliage_provider: BlockStateProvider,
    #[serde(default)]
    pub root_placer: Option<RootPlacer>,
    pub dirt_provider: BlockStateProvider,
    pub minimum_size: FeatureSize,
    pub decorators: Vec<TreeDecorator>,
    #[serde(default)]
    pub ignore_vines: DefaultOnError<bool>,
    #[serde(default)]
    pub force_dirt: DefaultOnError<bool>,
}

#[derive(Debug, DispatchDeserialize)]
pub enum TrunkPlacer {
    StraightTrunkPlacer(StraightTrunkPlacer),
    ForkingTrunkPlacer(ForkingTrunkPlacer),
    GiantTrunkPlacer(GiantTrunkPlacer),
    MegaJungleTrunkPlacer(MegaJungleTrunkPlacer),
    DarkOakTrunkPlacer(DarkOakTrunkPlacer),
    FancyTrunkPlacer(FancyTrunkPlacer),
    BendingTrunkPlacer(BendingTrunkPlacer),
    UpwardsBranchingTrunkPlacer(UpwardsBranchingTrunkPlacer),
    CherryTrunkPlacer(CherryTrunkPlacer),
}

#[derive(Debug, Deserialize)]
pub struct TrunkPlacerParts {
    pub base_height: Ranged<u32, 0, 32>,
    pub height_rand_a: Ranged<u32, 0, 24>,
    pub height_rand_b: Ranged<u32, 0, 24>,
}

#[derive(Debug, Deserialize)]
pub struct StraightTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
}

#[derive(Debug, Deserialize)]
pub struct ForkingTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
}

#[derive(Debug, Deserialize)]
pub struct GiantTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
}

#[derive(Debug, Deserialize)]
pub struct MegaJungleTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
}

#[derive(Debug, Deserialize)]
pub struct DarkOakTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
}

#[derive(Debug, Deserialize)]
pub struct FancyTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
}

#[derive(Debug, Deserialize)]
pub struct BendingTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
    #[serde(default = "one")]
    pub min_height_for_leaves: PositiveU32,
    #[serde(deserialize_with = "deserialize_bend_length")]
    pub bend_length: IntProvider,
}

int_provider_deserializer!(deserialize_bend_length, 1, 64);

fn one() -> PositiveU32 {
    PositiveU32::from(1)
}

#[derive(Debug, Deserialize)]
pub struct UpwardsBranchingTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
    pub extra_branch_steps: PositiveU32,
    pub place_branch_per_log_probability: Ranged<NotNan<f32>, 0, 1>,
    #[serde(deserialize_with = "IntProvider::deserialize_non_negative")]
    pub extra_path_length: IntProvider,
    pub can_grow_through: HolderSet<Block>,
}

#[derive(Debug, Deserialize)]
pub struct CherryTrunkPlacer {
    #[serde(flatten)]
    pub parts: TrunkPlacerParts,
    #[serde(deserialize_with = "deserialize_branch_count")]
    pub branch_count: IntProvider,
    #[serde(deserialize_with = "deserialize_branch_horizontal_length")]
    pub branch_horizontal_length: IntProvider,
    #[serde(deserialize_with = "deserialize_branch_start_offset_from_top")]
    pub branch_start_offset_from_top: IntProvider,
    #[serde(deserialize_with = "deserialize_branch_end_offset_from_top")]
    pub branch_end_offset_from_top: IntProvider,
}

int_provider_deserializer!(deserialize_branch_count, 1, 3);
int_provider_deserializer!(deserialize_branch_horizontal_length, 2, 16);
int_provider_deserializer!(deserialize_branch_end_offset_from_top, -16, 16);

fn deserialize_branch_start_offset_from_top<'de, D>(
    deserializer: D,
) -> Result<IntProvider, D::Error>
where
    D: Deserializer<'de>,
{
    let provider = IntProvider::deserialize_ranged(deserializer, -16, 0)?;
    if provider.max_value() - provider.min_value() < 1 {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Other("value provider with too little variation"),
            &"a variation of at least 2",
        ));
    }
    Ok(provider)
}

#[derive(Debug, DispatchDeserialize)]
pub enum RootPlacer {
    MangroveRootPlacer(MangroveRootPlacer),
}

#[derive(Debug, Deserialize)]
pub struct AboveRootPlacement {
    pub above_root_provider: BlockStateProvider,
    pub above_root_placement_chance: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct RootPlacerParts {
    pub trunk_offset_y: IntProvider,
    pub root_provider: BlockStateProvider,
    #[serde(default)]
    pub above_root_placement: Option<AboveRootPlacement>,
}

#[derive(Debug, Deserialize)]
pub struct MangroveRootPlacer {
    #[serde(flatten)]
    pub parts: RootPlacerParts,
    pub mangrove_root_placement: MangroveRootPlacement,
}

#[derive(Debug, Deserialize)]
pub struct MangroveRootPlacement {
    pub can_grow_through: HolderSet<Block>,
    pub muddy_roots_in: HolderSet<Block>,
    pub muddy_roots_provider: BlockStateProvider,
    pub max_root_width: Ranged<u32, 1, 12>,
    pub max_root_length: Ranged<u32, 1, 64>,
    pub random_skew_chance: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, DispatchDeserialize)]
pub enum TreeDecorator {
    TrunkVine(TrunkVineDecorator),
    LeaveVine(LeaveVineDecorator),
    Cocoa(CocoaDecorator),
    Beehive(BeehiveDecorator),
    AlterGround(AlterGroundDecorator),
    AttachedToLeaves(AttachedToLeavesDecorator),
}

#[derive(Debug, Deserialize)]
pub struct TrunkVineDecorator {}

#[derive(Debug, Deserialize)]
pub struct LeaveVineDecorator {
    pub probability: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct CocoaDecorator {
    pub probability: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct BeehiveDecorator {
    pub probability: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct AlterGroundDecorator {
    pub provider: BlockStateProvider,
}

#[derive(Debug, Deserialize)]
pub struct AttachedToLeavesDecorator {
    pub probability: Ranged<NotNan<f32>, 0, 1>,
    pub exclusion_radius_xz: Ranged<u32, 0, 16>,
    pub exclusion_radius_y: Ranged<u32, 0, 16>,
    pub block_provider: BlockStateProvider,
    pub required_empty_blocks: Ranged<u32, 1, 16>,
    pub directions: NonEmptyVec<Direction>,
}
