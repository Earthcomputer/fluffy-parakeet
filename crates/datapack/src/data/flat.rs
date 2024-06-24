use crate::data::biome::Biome;
use crate::data::holder::Holder;
use crate::data::DIMENSION_Y_SIZE;
use crate::identifier::IdentifierBuf;
use crate::serde_helpers::{DefaultOnError, DefaultToAir, DefaultToPlains, InlineVec, Ranged};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FlatLevelGeneratorSettings {
    #[serde(default)]
    pub structure_overrides: DefaultOnError<InlineVec<IdentifierBuf>>,
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
