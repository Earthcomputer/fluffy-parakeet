use crate::built_in_registries::Block;
use crate::data::block_predicate::BlockPredicate;
use crate::data::block_state::{BlockState, FluidState};
use crate::data::block_state_provider::{BlockStateProvider, RuleBasedBlockStateProvider};
use crate::data::feature::geode::GeodeConfiguration;
use crate::data::feature::ore::{OreConfiguration, TargetBlockState};
use crate::data::feature::tree::TreeConfiguration;
use crate::data::feature::{CaveSurface, PlacedFeature, WeightedPlacedFeature};
use crate::data::holder::Holder;
use crate::data::structure::processor::StructureProcessorList;
use crate::data::tag::{deserialize_hashed_tag, HolderSet, HolderValueSet};
use crate::data::value_provider::{FloatProvider, IntProvider};
use crate::data::DIMENSION_Y_SIZE;
use crate::serde_helpers::{
    DefaultOnError, DefaultToNum, DefaultToTrue, NonNegativeU32, PositiveU32, Ranged, ValueProvider,
};
use crate::{float_provider_deserializer, int_provider_deserializer};
use datapack_macros::DispatchDeserialize;
use glam::IVec3;
use ordered_float::NotNan;
use serde::Deserialize;
use util::direction::Direction;
use util::identifier::IdentifierBuf;

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum ConfiguredFeature {
    NoOp(NoneFeatureConfiguration),
    Tree(TreeConfiguration),
    Flower(RandomPatchConfiguration),
    NoBonemealFlower(RandomPatchConfiguration),
    RandomPatch(RandomPatchConfiguration),
    BlockPile(BlockPileConfiguration),
    Spring(SpringConfiguration),
    ChorusPlant(NoneFeatureConfiguration),
    ReplaceSingleBlock(ReplaceBlockConfiguration),
    VoidStartPlatform(NoneFeatureConfiguration),
    DesertWell(NoneFeatureConfiguration),
    Fossil(FossilFeatureConfiguration),
    HugeRedMushroom(HugeMushroomFeatureConfiguration),
    HugeBrownMushroom(HugeMushroomFeatureConfiguration),
    IceSpike(NoneFeatureConfiguration),
    GlowstoneBlob(NoneFeatureConfiguration),
    FreezeTopLayer(NoneFeatureConfiguration),
    Vines(NoneFeatureConfiguration),
    BlockColumn(BlockColumnConfiguration),
    VegetationPatch(VegetationPatchConfiguration),
    WaterloggedVegetationPatch(VegetationPatchConfiguration),
    RootSystem(RootSystemConfiguration),
    MultifaceGrowth(MultifaceGrowthConfiguration),
    UnderwaterMagma(UnderwaterMagmaConfiguration),
    MonsterRoom(NoneFeatureConfiguration),
    BlueIce(NoneFeatureConfiguration),
    Iceberg(BlockStateConfiguration),
    ForestRock(BlockStateConfiguration),
    Disk(DiskConfiguration),
    Lake(LakeConfiguration),
    Ore(OreConfiguration),
    EndPlatform(NoneFeatureConfiguration),
    EndSpike(SpikeConfiguration),
    EndIsland(NoneFeatureConfiguration),
    EndGateway(EndGatewayConfiguration),
    Seagrass(ProbabilityFeatureConfiguration),
    Kelp(NoneFeatureConfiguration),
    CoralTree(NoneFeatureConfiguration),
    CoralMushroom(NoneFeatureConfiguration),
    CoralClaw(NoneFeatureConfiguration),
    SeaPickle(CountConfiguration),
    SimpleBlock(SimpleBlockConfiguration),
    Bamboo(ProbabilityFeatureConfiguration),
    HugeFungus(HugeFungusConfiguration),
    NetherForestVegetation(NetherForestVegetationConfiguration),
    WeepingVines(NoneFeatureConfiguration),
    TwistingVines(TwistingVinesConfiguration),
    BasaltColumns(ColumnFeatureConfiguration),
    DeltaFeature(DeltaFeatureConfiguration),
    ReplaceBlobs(ReplaceSphereConfiguration),
    FillLayer(LayerConfiguration),
    BonusChest(NoneFeatureConfiguration),
    BasaltPiller(NoneFeatureConfiguration),
    ScatteredOre(OreConfiguration),
    RandomSelector(RandomFeatureConfiguration),
    SimpleRandomSelector(SimpleRandomFeatureConfiguration),
    RandomBooleanSelector(RandomBooleanFeatureConfiguration),
    Geode(GeodeConfiguration),
    DripstoneCluster(DripstoneClusterConfiguration),
    LargeDripstone(LargeDripstoneConfiguration),
    PointedDripstone(PointedDripstoneConfiguration),
    SculkPatch(SculkPatchConfiguration),
}

#[derive(Debug, Deserialize)]
pub struct NoneFeatureConfiguration {}

#[derive(Debug, Deserialize)]
pub struct RandomPatchConfiguration {
    #[serde(default)]
    pub tries: DefaultOnError<PositiveU32, DefaultToNum<128>>,
    #[serde(default)]
    pub xz_spread: DefaultOnError<NonNegativeU32, DefaultToNum<7>>,
    #[serde(default)]
    pub y_spread: DefaultOnError<NonNegativeU32, DefaultToNum<3>>,
    pub feature: Box<Holder<PlacedFeature>>,
}

#[derive(Debug, Deserialize)]
pub struct BlockPileConfiguration {
    pub state_provider: BlockStateProvider,
}

#[derive(Debug, Deserialize)]
pub struct SpringConfiguration {
    pub state: FluidState,
    #[serde(default)]
    pub requires_block_below: DefaultOnError<bool, DefaultToTrue>,
    #[serde(default)]
    pub rock_count: DefaultOnError<i32, DefaultToNum<4>>,
    #[serde(default)]
    pub hole_count: DefaultOnError<i32, DefaultToNum<1>>,
    pub valid_blocks: HolderSet<Block>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceBlockConfiguration {
    pub targets: Vec<TargetBlockState>,
}

#[derive(Debug, Deserialize)]
pub struct FossilFeatureConfiguration {
    pub fossil_structures: Vec<IdentifierBuf>,
    pub overlay_structures: Vec<IdentifierBuf>,
    pub fossil_processors: Holder<StructureProcessorList>,
    pub overlay_processors: Holder<StructureProcessorList>,
    pub max_empty_corners_allowed: Ranged<u32, 0, 7>,
}

#[derive(Debug, Deserialize)]
pub struct HugeMushroomFeatureConfiguration {
    pub cap_provider: BlockStateProvider,
    pub stem_provider: BlockStateProvider,
    #[serde(default)]
    pub foliage_radius: DefaultOnError<i32, DefaultToNum<2>>,
}

#[derive(Debug, Deserialize)]
pub struct BlockColumnConfiguration {
    pub layers: Vec<BlockColumnLayer>,
    pub direction: Direction,
    pub allowed_placement: BlockPredicate,
    pub prioritize_tip: bool,
}

#[derive(Debug, Deserialize)]
pub struct BlockColumnLayer {
    pub height: NonNegativeU32,
    pub provider: BlockStateProvider,
}

#[derive(Debug, Deserialize)]
pub struct VegetationPatchConfiguration {
    #[serde(deserialize_with = "deserialize_hashed_tag")]
    pub replaceable: IdentifierBuf,
    pub ground_state: BlockStateProvider,
    pub vegetation_feature: Box<Holder<PlacedFeature>>,
    pub surface: CaveSurface,
    #[serde(deserialize_with = "one_one_twenty_eight_provider")]
    pub depth: IntProvider,
    pub extra_bottom_block_chance: Ranged<NotNan<f32>, 0, 1>,
    pub vertical_range: Ranged<u32, 1, 256>,
    pub vegetation_chance: Ranged<NotNan<f32>, 0, 1>,
    pub xz_radius: IntProvider,
    pub extra_edge_column_chance: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct RootSystemConfiguration {
    pub feature: Box<Holder<PlacedFeature>>,
    pub required_vertical_space_for_tree: Ranged<u32, 1, 64>,
    pub root_radius: Ranged<u32, 1, 64>,
    #[serde(deserialize_with = "deserialize_hashed_tag")]
    pub root_replaceable: IdentifierBuf,
    pub root_state_provider: BlockStateProvider,
    pub root_placement_attempts: Ranged<u32, 1, 256>,
    pub root_column_max_height: Ranged<u32, 1, 4096>,
    pub hanging_root_radius: Ranged<u32, 1, 64>,
    pub hanging_roots_vertical_span: Ranged<u32, 0, 16>,
    pub hanging_root_state_provider: BlockStateProvider,
    pub hanging_root_placement_attempts: Ranged<u32, 1, 256>,
    pub allowed_vertical_water_for_tree: Ranged<u32, 1, 64>,
    pub allowed_tree_predicate: BlockPredicate,
}

#[derive(Debug, Deserialize)]
pub struct MultifaceGrowthConfiguration {
    #[serde(default)]
    pub block: DefaultOnError<IdentifierBuf, DefaultToGlowLichen>,
    #[serde(default)]
    pub search_range: DefaultOnError<Ranged<u32, 1, 64>, DefaultToNum<10>>,
    #[serde(default)]
    pub can_place_on_floor: DefaultOnError<bool>,
    #[serde(default)]
    pub can_place_on_ceiling: DefaultOnError<bool>,
    #[serde(default)]
    pub can_place_on_wall: DefaultOnError<bool>,
    #[serde(default)]
    pub chance_of_spreading: DefaultOnError<Ranged<NotNan<f32>, 0, 1>, DefaultToNum<1, 2>>,
    pub can_be_placed_on: HolderSet<Block>,
}

pub struct DefaultToGlowLichen;
impl ValueProvider<IdentifierBuf> for DefaultToGlowLichen {
    fn provide() -> IdentifierBuf {
        IdentifierBuf::new("glow_lichen").unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct UnderwaterMagmaConfiguration {
    pub floor_search_range: Ranged<u32, 0, 512>,
    pub placement_radius_around_floor: Ranged<u32, 0, 64>,
    pub placement_probability_per_valid_position: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct BlockStateConfiguration {
    pub state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct DiskConfiguration {
    pub state_provider: RuleBasedBlockStateProvider,
    pub target: BlockPredicate,
    #[serde(deserialize_with = "zero_eight_provider")]
    pub radius: IntProvider,
    pub half_height: Ranged<u32, 0, 4>,
}

#[derive(Debug, Deserialize)]
pub struct LakeConfiguration {
    pub fluid: BlockStateProvider,
    pub barrier: BlockStateProvider,
}

#[derive(Debug, Deserialize)]
pub struct SpikeConfiguration {
    #[serde(default)]
    pub crystal_invulnerable: DefaultOnError<bool>,
    pub spikes: Vec<EndSpike>,
    #[serde(default)]
    pub crystal_beam_target: Option<IVec3>,
}

#[derive(Debug, Deserialize)]
pub struct EndSpike {
    #[serde(rename = "centerX")]
    #[serde(default)]
    pub center_x: DefaultOnError<i32>,
    #[serde(rename = "centerZ")]
    #[serde(default)]
    pub center_z: DefaultOnError<i32>,
    #[serde(default)]
    pub radius: DefaultOnError<i32>,
    #[serde(default)]
    pub height: DefaultOnError<i32>,
    #[serde(default)]
    pub guarded: DefaultOnError<bool>,
}

#[derive(Debug, Deserialize)]
pub struct EndGatewayConfiguration {
    #[serde(default)]
    pub exit: Option<IVec3>,
    pub exact: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProbabilityFeatureConfiguration {
    pub probability: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct CountConfiguration {
    pub count: Ranged<u32, 0, 256>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleBlockConfiguration {
    pub to_place: BlockStateProvider,
}

#[derive(Debug, Deserialize)]
pub struct HugeFungusConfiguration {
    pub valid_base_block: BlockState,
    pub stem_state: BlockState,
    pub hat_state: BlockState,
    pub decor_state: BlockState,
    pub replaceable_blocks: BlockPredicate,
    #[serde(default)]
    pub planted: DefaultOnError<bool>,
}

#[derive(Debug, Deserialize)]
pub struct NetherForestVegetationConfiguration {
    pub state_provider: BlockStateProvider,
    pub spread_width: PositiveU32,
    pub spread_height: PositiveU32,
}

#[derive(Debug, Deserialize)]
pub struct TwistingVinesConfiguration {
    pub spread_width: PositiveU32,
    pub spread_height: PositiveU32,
    pub max_height: PositiveU32,
}

#[derive(Debug, Deserialize)]
pub struct ColumnFeatureConfiguration {
    #[serde(deserialize_with = "zero_three_provider")]
    pub reach: IntProvider,
    #[serde(deserialize_with = "one_ten_provider")]
    pub height: IntProvider,
}

#[derive(Debug, Deserialize)]
pub struct DeltaFeatureConfiguration {
    pub contents: BlockState,
    pub rim: BlockState,
    #[serde(deserialize_with = "zero_sixteen_provider")]
    pub size: IntProvider,
    #[serde(deserialize_with = "zero_sixteen_provider")]
    pub rim_size: IntProvider,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceSphereConfiguration {
    pub target: BlockState,
    pub state: BlockState,
    #[serde(deserialize_with = "zero_twelve_provider")]
    pub radius: IntProvider,
}

#[derive(Debug, Deserialize)]
pub struct LayerConfiguration {
    pub height: Ranged<u32, 0, { DIMENSION_Y_SIZE as i64 }>,
    pub state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct RandomFeatureConfiguration {
    pub features: Vec<WeightedPlacedFeature>,
    pub placed_feature: Box<PlacedFeature>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleRandomFeatureConfiguration {
    pub features: HolderValueSet<PlacedFeature>,
}

#[derive(Debug, Deserialize)]
pub struct RandomBooleanFeatureConfiguration {
    pub feature_true: Box<PlacedFeature>,
    pub feature_false: Box<PlacedFeature>,
}

#[derive(Debug, Deserialize)]
pub struct DripstoneClusterConfiguration {
    pub floor_to_ceiling_search_range: Ranged<u32, 1, 512>,
    #[serde(deserialize_with = "one_one_twenty_eight_provider")]
    pub height: IntProvider,
    #[serde(deserialize_with = "one_one_twenty_eight_provider")]
    pub radius: IntProvider,
    pub max_stalagmite_stalactite_height_diff: Ranged<u32, 0, 64>,
    pub height_deviation: Ranged<u32, 1, 64>,
    #[serde(deserialize_with = "zero_one_twenty_eight_provider")]
    pub dripstone_block_layer_thickness: IntProvider,
    #[serde(deserialize_with = "zero_two_float_provider")]
    pub density: FloatProvider,
    #[serde(deserialize_with = "zero_two_float_provider")]
    pub wetness: FloatProvider,
    pub chance_of_dripstone_column_at_max_distance_from_center: Ranged<NotNan<f32>, 0, 1>,
    pub max_distance_from_edge_affecting_chance_of_dripstone_column: Ranged<u32, 1, 64>,
    pub max_distance_from_center_affecting_height_bias: Ranged<u32, 1, 64>,
}

#[derive(Debug, Deserialize)]
pub struct LargeDripstoneConfiguration {
    #[serde(default)]
    pub floor_to_ceiling_search_range: DefaultOnError<Ranged<u32, 1, 512>, DefaultToNum<30>>,
    #[serde(deserialize_with = "one_sixty_provider")]
    pub column_radius: IntProvider,
    #[serde(deserialize_with = "zero_twenty_float_provider")]
    pub height_scale: FloatProvider,
    pub max_column_radius_to_cave_height_ratio: Ranged<NotNan<f32>, 1, 10, 10>,
    #[serde(deserialize_with = "point_one_ten_float_provider")]
    pub stalactite_bluntness: FloatProvider,
    #[serde(deserialize_with = "point_one_ten_float_provider")]
    pub stalagmite_bluntness: FloatProvider,
    #[serde(deserialize_with = "zero_two_float_provider")]
    pub wind_speed: FloatProvider,
    pub min_radius_for_wind: Ranged<u32, 0, 100>,
    pub min_bluntness_for_wind: Ranged<NotNan<f32>, 0, 5>,
}

#[derive(Debug, Deserialize)]
pub struct PointedDripstoneConfiguration {
    #[serde(default)]
    pub chance_of_taller_dripstone: DefaultOnError<Ranged<NotNan<f32>, 0, 1>, DefaultToNum<1, 5>>,
    #[serde(default)]
    pub chance_of_directional_spread:
        DefaultOnError<Ranged<NotNan<f32>, 0, 1>, DefaultToNum<7, 10>>,
    #[serde(default)]
    pub chance_of_spread_radius2: DefaultOnError<Ranged<NotNan<f32>, 0, 1>, DefaultToNum<1, 2>>,
    #[serde(default)]
    pub chance_of_spread_radius3: DefaultOnError<Ranged<NotNan<f32>, 0, 1>, DefaultToNum<1, 2>>,
}

#[derive(Debug, Deserialize)]
pub struct SculkPatchConfiguration {
    pub charge_count: Ranged<u32, 1, 32>,
    pub amount_per_charge: Ranged<u32, 1, 500>,
    pub spread_amounts: Ranged<u32, 1, 64>,
    pub growth_rounds: Ranged<u32, 0, 8>,
    pub spread_rounds: Ranged<u32, 0, 8>,
    pub extra_rare_growths: IntProvider,
    pub catalyst_chance: Ranged<NotNan<f32>, 0, 1>,
}

int_provider_deserializer!(zero_three_provider, 0, 3);
int_provider_deserializer!(zero_eight_provider, 0, 8);
int_provider_deserializer!(zero_twelve_provider, 0, 12);
int_provider_deserializer!(zero_sixteen_provider, 0, 16);
int_provider_deserializer!(zero_one_twenty_eight_provider, 0, 128);
int_provider_deserializer!(one_ten_provider, 1, 10);
int_provider_deserializer!(one_sixty_provider, 1, 60);
int_provider_deserializer!(one_one_twenty_eight_provider, 1, 128);

float_provider_deserializer!(zero_two_float_provider, 0.0, 2.0);
float_provider_deserializer!(zero_twenty_float_provider, 0.0, 20.0);
float_provider_deserializer!(point_one_ten_float_provider, 0.1, 10.0);
