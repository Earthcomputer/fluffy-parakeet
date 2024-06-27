use crate::data::block_state::BlockState;
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::Deserialize;
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
