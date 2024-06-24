use crate::data::block_state::BlockState;
use crate::data::{DIMENSION_MAX_Y, DIMENSION_MIN_Y};
use crate::identifier::IdentifierBuf;
use crate::serde_helpers::Ranged;
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug, DispatchDeserialize)]
pub enum SurfaceRuleSource {
    Bandlands(BandlandsRuleSource),
    Block(BlockRuleSource),
    Sequence(SequenceRuleSource),
    Condition(Box<TestRuleSource>),
}

#[derive(Debug, Deserialize)]
pub struct BandlandsRuleSource {}

#[derive(Debug, Deserialize)]
pub struct BlockRuleSource {
    pub result_state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct SequenceRuleSource {
    pub sequence: Vec<SurfaceRuleSource>,
}

#[derive(Debug, Deserialize)]
pub struct TestRuleSource {
    pub if_true: SurfaceRulesConditionSource,
    pub then_run: SurfaceRuleSource,
}

#[derive(Debug, DispatchDeserialize)]
pub enum SurfaceRulesConditionSource {
    Biome(BiomeConditionSource),
    NoiseThreshold(NoiseThresholdConditionSource),
    VerticalGradient(VerticalGradientConditionSource),
    YAbove(YConditionSource),
    Water(WaterConditionSource),
    Temperature(TemperatureConditionSource),
    Steep(SteepConditionSource),
    Not(NotConditionSource),
    Hole(HoleConditionSource),
    AbovePreliminarySurface(AbovePreliminarySurfaceConditionSource),
    StoneDepth(StoneDepthCheckConditionSource),
}

#[derive(Debug, Deserialize)]
pub struct BiomeConditionSource {
    pub biome_is: Vec<IdentifierBuf>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseThresholdConditionSource {
    pub noise: IdentifierBuf,
    pub min_threshold: NotNan<f64>,
    pub max_threshold: NotNan<f64>,
}

#[derive(Debug, Deserialize)]
pub struct VerticalGradientConditionSource {
    pub random_name: IdentifierBuf,
    pub true_at_and_below: VerticalAnchor,
    pub false_at_and_above: VerticalAnchor,
}

#[derive(Debug, Deserialize)]
pub struct YConditionSource {
    pub anchor: VerticalAnchor,
    pub surface_depth_multiplier: Ranged<i32, -20, 20>,
    pub add_stone_depth: bool,
}

#[derive(Debug, Deserialize)]
pub struct WaterConditionSource {
    pub offset: i32,
    pub surface_depth_multiplier: Ranged<i32, -20, 20>,
    pub add_stone_depth: bool,
}

#[derive(Debug, Deserialize)]
pub struct TemperatureConditionSource {}

#[derive(Debug, Deserialize)]
pub struct SteepConditionSource {}

#[derive(Debug, Deserialize)]
pub struct NotConditionSource {
    pub invert: Box<SurfaceRulesConditionSource>,
}

#[derive(Debug, Deserialize)]
pub struct HoleConditionSource {}

#[derive(Debug, Deserialize)]
pub struct AbovePreliminarySurfaceConditionSource {}

#[derive(Debug, Deserialize)]
pub struct StoneDepthCheckConditionSource {
    pub offset: i32,
    pub add_surface_depth: bool,
    pub secondary_depth_range: i32,
    pub surface_type: CaveSurface,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaveSurface {
    Ceiling,
    Floor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAnchor {
    Absolute(Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>),
    AboveBottom(Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>),
    BelowTop(Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>),
}
