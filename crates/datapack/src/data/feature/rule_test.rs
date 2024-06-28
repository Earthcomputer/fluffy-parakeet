use crate::data::block_state::BlockState;
use crate::serde_helpers::{DefaultOnError, ValueProvider};
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::Deserialize;
use util::direction::Axis;
use util::identifier::IdentifierBuf;

#[derive(Debug, DispatchDeserialize)]
#[dispatch(tag_name = "predicate_type")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum RuleTest {
    AlwaysTrue(AlwaysTrueTest),
    BlockMatch(BlockMatchTest),
    BlockstateMatch(BlockStateMatchTest),
    TagMatch(TagMatchTest),
    RandomBlockMatch(RandomBlockMatchTest),
    RandomBlockstateMatch(RandomBlockStateMatchTest),
}

impl Default for RuleTest {
    fn default() -> Self {
        RuleTest::AlwaysTrue(AlwaysTrueTest {})
    }
}

#[derive(Debug, Deserialize)]
pub struct AlwaysTrueTest {}

#[derive(Debug, Deserialize)]
pub struct BlockMatchTest {
    pub block: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct BlockStateMatchTest {
    pub block_state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct TagMatchTest {
    pub tag: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct RandomBlockMatchTest {
    pub block: IdentifierBuf,
    pub probability: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct RandomBlockStateMatchTest {
    pub block_state: BlockState,
    pub probability: NotNan<f32>,
}

#[derive(Debug, DispatchDeserialize)]
#[dispatch(tag_name = "predicate_type")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum PosRuleTest {
    AlwaysTrue(PosAlwaysTrueTest),
    LinearPos(LinearPosTest),
    AxisAlignedLinearPos(AxisAlignedLinearPosTest),
}

impl Default for PosRuleTest {
    fn default() -> Self {
        PosRuleTest::AlwaysTrue(PosAlwaysTrueTest {})
    }
}

#[derive(Debug, Deserialize)]
pub struct PosAlwaysTrueTest {}

#[derive(Debug, Deserialize)]
pub struct LinearPosTest {
    #[serde(default)]
    pub min_chance: DefaultOnError<NotNan<f32>>,
    #[serde(default)]
    pub max_chance: DefaultOnError<NotNan<f32>>,
    #[serde(default)]
    pub min_dist: DefaultOnError<i32>,
    #[serde(default)]
    pub max_dist: DefaultOnError<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AxisAlignedLinearPosTest {
    #[serde(default)]
    pub min_chance: DefaultOnError<NotNan<f32>>,
    #[serde(default)]
    pub max_chance: DefaultOnError<NotNan<f32>>,
    #[serde(default)]
    pub min_dist: DefaultOnError<i32>,
    #[serde(default)]
    pub max_dist: DefaultOnError<i32>,
    #[serde(default)]
    pub axis: DefaultOnError<Axis, DefaultToY>,
}

pub struct DefaultToY;
impl ValueProvider<Axis> for DefaultToY {
    fn provide() -> Axis {
        Axis::Y
    }
}
