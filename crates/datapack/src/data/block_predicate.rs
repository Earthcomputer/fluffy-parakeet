use crate::built_in_registries::{Block, Fluid};
use crate::data::block_state::BlockState;
use crate::data::tag::HolderSet;
use crate::serde_helpers::RangedIVec3;
use datapack_macros::DispatchDeserialize;
use serde::Deserialize;
use util::direction::Direction;
use util::identifier::IdentifierBuf;

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
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
    pub blocks: HolderSet<Block>,
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
    pub fluids: HolderSet<Fluid>,
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
