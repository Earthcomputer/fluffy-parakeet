use crate::data::feature::PlacedFeature;
use crate::data::height_provider::HeightProvider;
use crate::data::holder::Holder;
use crate::data::structure::processor::StructureProcessorList;
use crate::data::structure::StructureSettings;
use crate::data::SimpleWeightedListEntry;
use crate::serde_helpers::DefaultOnError;
use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};
use serde::{Deserialize, Deserializer};
use util::heightmap_type::HeightmapType;
use util::identifier::IdentifierBuf;
use util::ranged::{NonNegativeI32, Ranged};

#[derive(Debug, Deserialize)]
pub struct JigsawStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
    pub start_pool: Holder<StructureTemplatePool>,
    #[serde(default)]
    pub start_jigsaw_name: Option<IdentifierBuf>,
    pub size: Ranged<u32, 0, 20>,
    pub start_height: HeightProvider,
    pub use_expansion_pack: bool,
    #[serde(default)]
    pub project_start_to_heightmap: Option<HeightmapType>,
    pub max_distance_from_center: Ranged<u32, 1, 128>,
    #[serde(default)]
    pub pool_aliases: Vec<PoolAliasBinding>,
    #[serde(default)]
    pub dimension_padding: DimensionPadding,
    #[serde(default)]
    pub liquid_settings: LiquidSettings,
}

#[derive(Debug, Deserialize)]
pub struct StructureTemplatePool {
    pub fallback: Box<Holder<StructureTemplatePool>>,
    pub elements: Vec<StructureTemplatePoolElement>,
}

#[derive(Debug, Deserialize)]
pub struct StructureTemplatePoolElement {
    pub element: StructurePoolElement,
    pub weight: Ranged<u32, 1, 150>,
}

#[derive(Debug, DispatchDeserialize)]
#[dispatch(tag_name = "element_type")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum StructurePoolElement {
    SinglePoolElement(SinglePoolElement),
    ListPoolElement(ListPoolElement),
    FeaturePoolElement(FeaturePoolElement),
    EmptyPoolElement(EmptyPoolElement),
    LegacySinglePoolElement(LegacySinglePoolElement),
}

#[derive(Debug, Deserialize)]
pub struct SinglePoolElement {
    pub location: IdentifierBuf,
    pub processors: Holder<StructureProcessorList>,
    pub projection: Projection,
    #[serde(default)]
    pub overridable_liquid_settings: Option<LiquidSettings>,
}

#[derive(Debug, Deserialize)]
pub struct ListPoolElement {
    pub elements: Vec<StructurePoolElement>,
    pub projection: Projection,
}

#[derive(Debug, Deserialize)]
pub struct FeaturePoolElement {
    pub feature: Holder<PlacedFeature>,
    pub projection: Projection,
}

#[derive(Debug, Deserialize)]
pub struct EmptyPoolElement {}

#[derive(Debug, Deserialize)]
pub struct LegacySinglePoolElement {
    pub location: IdentifierBuf,
    pub processors: Holder<StructureProcessorList>,
    pub projection: Projection,
    #[serde(default)]
    pub overridable_liquid_settings: Option<LiquidSettings>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum Projection {
    TerrainMatching,
    Rigid,
}

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum PoolAliasBinding {
    Random(RandomAliasBinding),
    RandomGroup(RandomGroupAliasBinding),
    Direct(DirectAliasBinding),
}

#[derive(Debug, Deserialize)]
pub struct RandomAliasBinding {
    pub alias: IdentifierBuf,
    pub targets: Vec<SimpleWeightedListEntry<IdentifierBuf>>,
}

#[derive(Debug, Deserialize)]
pub struct RandomGroupAliasBinding {
    pub groups: Vec<SimpleWeightedListEntry<Vec<PoolAliasBinding>>>,
}

#[derive(Debug, Deserialize)]
pub struct DirectAliasBinding {
    pub alias: IdentifierBuf,
    pub target: IdentifierBuf,
}

#[derive(Debug, Default)]
pub struct DimensionPadding {
    pub bottom: NonNegativeI32,
    pub top: NonNegativeI32,
}

impl<'de> Deserialize<'de> for DimensionPadding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SurrogateRecord {
            #[serde(default)]
            pub bottom: DefaultOnError<NonNegativeI32>,
            #[serde(default)]
            pub top: DefaultOnError<NonNegativeI32>,
        }
        #[derive(UntaggedDeserialize)]
        enum Surrogate {
            Constant(NonNegativeI32),
            Record(SurrogateRecord),
        }
        match Surrogate::deserialize(deserializer)? {
            Surrogate::Constant(value) => Ok(DimensionPadding {
                bottom: value,
                top: value,
            }),
            Surrogate::Record(SurrogateRecord { bottom, top }) => Ok(DimensionPadding {
                bottom: *bottom,
                top: *top,
            }),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum LiquidSettings {
    IgnoreWaterlogging,
    #[default]
    ApplyWaterlogging,
}
