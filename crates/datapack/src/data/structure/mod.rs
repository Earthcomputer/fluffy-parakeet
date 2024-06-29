use crate::data::biome::{Biome, MobCategory, SpawnerData};
use crate::data::height_provider::HeightProvider;
use crate::data::step::DecorationStep;
use crate::data::structure::jigsaw::JigsawStructure;
use crate::data::tag::HolderSet;
use crate::serde_helpers::{NonEmptyVec, Ranged};
use ahash::AHashMap;
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::Deserialize;

pub mod jigsaw;
pub mod placement;
pub mod processor;
pub mod set;

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum Structure {
    BuriedTreasure(BuriedTreasureStructure),
    DesertPyramid(DesertPyramidStructure),
    EndCity(EndCityStructure),
    Fortress(NetherFortressStructure),
    Igloo(IglooStructure),
    Jigsaw(JigsawStructure),
    JungleTemple(JungleTempleStructure),
    Mineshaft(MineshaftStructure),
    NetherFossil(NetherFossilStructure),
    OceanMonument(OceanMonumentStructure),
    OceanRuin(OceanRuinStructure),
    RuinedPortal(RuinedPortalStructure),
    Shipwreck(ShipwreckStructure),
    Stronghold(StrongholdStructure),
    SwampHut(SwampHutStructure),
    WoodlandMansion(WoodlandMansionStructure),
}

#[derive(Debug, Deserialize)]
pub struct StructureSettings {
    pub biomes: HolderSet<Biome>,
    pub spawn_overrides: AHashMap<MobCategory, StructureSpawnOverride>,
    pub step: DecorationStep,
    #[serde(default)]
    pub terrain_adaptation: TerrainAdjustment,
}

impl Default for StructureSettings {
    fn default() -> Self {
        StructureSettings {
            biomes: HolderSet::default(),
            spawn_overrides: AHashMap::default(),
            step: DecorationStep::SurfaceStructures,
            terrain_adaptation: TerrainAdjustment::None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StructureSpawnOverride {
    pub bounding_box: BoundingBoxType,
    pub spawns: Vec<SpawnerData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum BoundingBoxType {
    Piece,
    Structure,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum TerrainAdjustment {
    #[default]
    None,
    Bury,
    BeardThin,
    BeardBox,
    Encapsulate,
}

#[derive(Debug, Deserialize)]
pub struct BuriedTreasureStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct DesertPyramidStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct EndCityStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct NetherFortressStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct IglooStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct JungleTempleStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct MineshaftStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
    pub mineshaft_type: MineshaftType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum MineshaftType {
    Normal,
    Mesa,
}

#[derive(Debug, Deserialize)]
pub struct NetherFossilStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
    pub height: HeightProvider,
}

#[derive(Debug, Deserialize)]
pub struct OceanMonumentStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct OceanRuinStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
    pub biome_temp: OceanRuinType,
    pub large_probability: Ranged<NotNan<f32>, 0, 1>,
    pub cluster_probability: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum OceanRuinType {
    Warm,
    Cold,
}

#[derive(Debug, Deserialize)]
pub struct RuinedPortalStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
    pub setups: NonEmptyVec<RuinedPortalSetup>,
}

#[derive(Debug, Deserialize)]
pub struct RuinedPortalSetup {
    pub placement: RuinedPortalVerticalPlacement,
    pub air_pocket_probability: Ranged<NotNan<f32>, 0, 1>,
    pub mossiness: Ranged<NotNan<f32>, 0, 1>,
    pub overgrown: bool,
    pub vines: bool,
    pub can_be_cold: bool,
    pub replace_with_blackstone: bool,
    #[serde(with = "crate::serde_helpers::PositiveF32")]
    pub weight: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuinedPortalVerticalPlacement {
    OnLandSurface,
    PartlyBuried,
    OnOceanFloor,
    InMountain,
    Underground,
    InNether,
}

#[derive(Debug, Deserialize)]
pub struct ShipwreckStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
    pub is_beached: bool,
}

#[derive(Debug, Deserialize)]
pub struct StrongholdStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct SwampHutStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}

#[derive(Debug, Deserialize)]
pub struct WoodlandMansionStructure {
    #[serde(flatten)]
    pub settings: StructureSettings,
}
