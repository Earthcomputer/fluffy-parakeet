use crate::data::biome::Biome;
use crate::data::feature::structure_set::StructureSet;
use crate::data::holder::Holder;
use crate::data::tag::HolderSet;
use crate::data::DIMENSION_Y_SIZE;
use crate::serde_helpers::{DefaultOnError, DefaultToAir, DefaultToPlains, Ranged};
use serde::Deserialize;
use util::identifier::IdentifierBuf;

#[derive(Debug, Deserialize)]
pub struct FlatLevelGeneratorSettings {
    #[serde(default)]
    pub structure_overrides: DefaultOnError<HolderSet<StructureSet>>,
    pub layers: Vec<FlatLayerInfo>,
    #[serde(default)]
    pub lakes: bool,
    #[serde(default)]
    pub features: bool,
    #[serde(default)]
    pub biome: DefaultOnError<Holder<Biome>, DefaultToPlains>,
}

#[derive(Debug, Deserialize)]
pub struct FlatLayerInfo {
    pub height: Ranged<u32, 0, { DIMENSION_Y_SIZE as i64 }>,
    #[serde(default)]
    pub block: DefaultOnError<IdentifierBuf, DefaultToAir>,
}
