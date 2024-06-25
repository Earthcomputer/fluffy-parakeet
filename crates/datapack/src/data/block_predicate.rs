use serde::Deserialize;
use datapack_macros::DispatchDeserialize;
use util::direction::Direction;
use util::identifier::IdentifierBuf;
use crate::data::block_state::BlockState;
use crate::serde_helpers::{InlineVec, RangedIVec3};

#[derive(Debug, DispatchDeserialize)]
pub enum BlockPredicate {
    MatchingBlocks(MatchingBlocksPredicate),
    MatchingBlocksTag(MatchingBlockTagPredicate),
    MatchingFluids(MatchingFluidsPredicate),
    HasSturdyFace(HasSturdyFacePredicate),
    Solid(SolidPredicate),
    Replaceable(ReplaceablePredicate),
    WouldSurvive(WouldSurvivePredicate),
    InsideWorldBounds(InsideWorldBoundsPredicate),
    AnyOf(AnyOfPredicate),
    AllOf(AllOfPredicate),
    Not(NotPredicate),
    True(TruePredicate),
    Unobstructed(UnobstructedPredicate),
}

impl BlockPredicate {
    pub fn always_true() -> BlockPredicate {
        BlockPredicate::True(TruePredicate {})
    }
}

#[derive(Debug, Deserialize)]
pub struct MatchingBlocksPredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
    pub blocks: InlineVec<IdentifierBuf>,
}

#[derive(Debug, Deserialize)]
pub struct MatchingBlockTagPredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
    pub tag: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct MatchingFluidsPredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
    pub fluids: InlineVec<IdentifierBuf>,
}

#[derive(Debug, Deserialize)]
pub struct HasSturdyFacePredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
    pub direction: Direction,
}

#[derive(Debug, Deserialize)]
pub struct SolidPredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceablePredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
}

#[derive(Debug, Deserialize)]
pub struct WouldSurvivePredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
    pub state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct InsideWorldBoundsPredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
}

#[derive(Debug, Deserialize)]
pub struct AnyOfPredicate {
    pub predicates: Vec<BlockPredicate>,
}

#[derive(Debug, Deserialize)]
pub struct AllOfPredicate {
    pub predicates: Vec<BlockPredicate>,
}

#[derive(Debug, Deserialize)]
pub struct NotPredicate {
    pub predicate: Box<BlockPredicate>,
}

#[derive(Debug, Deserialize)]
pub struct TruePredicate {}

#[derive(Debug, Deserialize)]
pub struct UnobstructedPredicate {
    #[serde(default)]
    pub offset: RangedIVec3<-16, 16, -16, 16>,
}
