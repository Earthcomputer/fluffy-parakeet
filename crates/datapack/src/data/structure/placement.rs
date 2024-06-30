use crate::data::biome::Biome;
use crate::data::tag::HolderSet;
use crate::serde_helpers::RangedIVec3;
use datapack_macros::DispatchDeserialize;

use serde::Deserialize;
use util::identifier::IdentifierBuf;
use util::ranged::{NonNegativeI32, Ranged};

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum StructurePlacement {
    RandomSpread(RandomSpreadStructurePlacement),
    ConcentricRings(ConcentricRingsStructurePlacement),
}

#[derive(Debug, Deserialize)]
pub struct CommonStructurePlacement {
    #[serde(default)]
    pub locate_offset: RangedIVec3<-16, 16, -16, 16>,
    #[serde(default)]
    pub frequency_reduction_method: FrequencyReductionMethod,
    #[serde(default = "one")]
    pub frequency: Ranged<f32, 0, 1>,
    pub salt: NonNegativeI32,
    #[serde(default)]
    pub exclusion_zone: Option<ExclusionZone>,
}

fn one() -> Ranged<f32, 0, 1> {
    Ranged::new(1.0).unwrap()
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum FrequencyReductionMethod {
    #[default]
    Default,
    LegacyType1,
    LegacyType2,
    LegacyType3,
}

#[derive(Debug, Deserialize)]
pub struct ExclusionZone {
    pub other_set: IdentifierBuf,
    pub chunk_count: Ranged<u32, 1, 16>,
}

#[derive(Debug, Deserialize)]
pub struct RandomSpreadStructurePlacement {
    #[serde(flatten)]
    pub common: CommonStructurePlacement,
    pub spacing: Ranged<u32, 0, 4096>,
    pub separation: Ranged<u32, 0, 4096>,
    #[serde(default)]
    pub spread_type: RandomSpreadType,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum RandomSpreadType {
    #[default]
    Linear,
    Triangular,
}

#[derive(Debug, Deserialize)]
pub struct ConcentricRingsStructurePlacement {
    #[serde(flatten)]
    pub common: CommonStructurePlacement,
    pub distance: Ranged<u32, 0, 1023>,
    pub spread: Ranged<u32, 0, 1023>,
    pub count: Ranged<u32, 0, 4095>,
    pub preferred_biomes: HolderSet<Biome>,
}
