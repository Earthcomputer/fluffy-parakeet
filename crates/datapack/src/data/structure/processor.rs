use crate::built_in_registries::Block;
use crate::data::block_state::BlockState;
use crate::data::feature::rule_test::{PosRuleTest, RuleTest};
use crate::data::tag::{deserialize_hashed_tag, HolderSet};
use crate::data::value_provider::IntProvider;
use crate::serde_helpers::{DefaultOnError, ValueProvider};
use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};

use serde::{Deserialize, Deserializer};
use serde_json::Value;
use util::heightmap_type::HeightmapType;
use util::identifier::IdentifierBuf;
use util::ranged::Ranged;

#[derive(Debug)]
pub struct StructureProcessorList {
    pub list: Vec<StructureProcessor>,
}

impl<'de> Deserialize<'de> for StructureProcessorList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Nested {
            processors: Vec<StructureProcessor>,
        }

        #[derive(UntaggedDeserialize)]
        enum Surrogate {
            Nested(Nested),
            Inline(Vec<StructureProcessor>),
        }

        let list = match Surrogate::deserialize(deserializer)? {
            Surrogate::Nested(Nested { processors }) => processors,
            Surrogate::Inline(processors) => processors,
        };
        Ok(Self { list })
    }
}

#[derive(Debug, DispatchDeserialize)]
#[dispatch(tag_name = "processor_type")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum StructureProcessor {
    BlockIgnore(BlockIgnoreProcessor),
    BlockRot(BlockRotProcessor),
    Gravity(GravityProcessor),
    JigsawReplacement(JigsawReplacementProcessor),
    Rule(RuleProcessor),
    Nop(NopProcessor),
    BlockAge(BlockAgeProcessor),
    BlackstoneReplace(BlackstoneReplaceProcessor),
    LavaSubmergedBlock(LavaSubmergedProcessor),
    ProtectedBlocks(ProtectedBlockProcessor),
    Capped(CappedProcessor),
}

#[derive(Debug, Deserialize)]
pub struct BlockIgnoreProcessor {
    pub blocks: Vec<BlockState>,
}

#[derive(Debug, Deserialize)]
pub struct BlockRotProcessor {
    #[serde(default)]
    pub rottable_blocks: Option<HolderSet<Block>>,
    pub integrity: Ranged<f32, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct GravityProcessor {
    #[serde(default)]
    pub heightmap: DefaultOnError<HeightmapType, DefaultToWorldSurfaceWg>,
    pub offset: DefaultOnError<i32>,
}

pub struct DefaultToWorldSurfaceWg;
impl ValueProvider<HeightmapType> for DefaultToWorldSurfaceWg {
    fn provide() -> HeightmapType {
        HeightmapType::OceanFloorWg
    }
}

#[derive(Debug, Deserialize)]
pub struct JigsawReplacementProcessor {}

#[derive(Debug, Deserialize)]
pub struct RuleProcessor {
    pub rules: Vec<ProcessorRule>,
}

#[derive(Debug, Deserialize)]
pub struct ProcessorRule {
    pub input_predicate: RuleTest,
    pub location_predicate: RuleTest,
    #[serde(default)]
    pub position_predicate: DefaultOnError<PosRuleTest>,
    pub output_state: BlockState,
    #[serde(default)]
    pub block_entity_modifier: DefaultOnError<RuleBlockEntityModifier, DefaultToPassthrough>,
}

pub struct DefaultToPassthrough;
impl ValueProvider<RuleBlockEntityModifier> for DefaultToPassthrough {
    fn provide() -> RuleBlockEntityModifier {
        RuleBlockEntityModifier::Passthrough(PassthroughModifier {})
    }
}

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum RuleBlockEntityModifier {
    Clear(ClearModifier),
    Passthrough(PassthroughModifier),
    AppendStatic(AppendStaticModifier),
    AppendLoot(AppendLootModifier),
}

#[derive(Debug, Deserialize)]
pub struct ClearModifier {}

#[derive(Debug, Deserialize)]
pub struct PassthroughModifier {}

#[derive(Debug, Deserialize)]
pub struct AppendStaticModifier {
    pub data: serde_json::Map<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct AppendLootModifier {
    pub loot_table: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct NopProcessor {}

#[derive(Debug, Deserialize)]
pub struct BlockAgeProcessor {
    pub mossiness: f32,
}

#[derive(Debug, Deserialize)]
pub struct BlackstoneReplaceProcessor {}

#[derive(Debug, Deserialize)]
pub struct LavaSubmergedProcessor {}

#[derive(Debug, Deserialize)]
pub struct ProtectedBlockProcessor {
    #[serde(deserialize_with = "deserialize_hashed_tag")]
    pub value: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct CappedProcessor {
    pub delegate: Box<StructureProcessor>,
    #[serde(deserialize_with = "IntProvider::deserialize_positive")]
    pub limit: IntProvider,
}
