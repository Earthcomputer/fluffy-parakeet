use crate::data::biome_source::BiomeSource;
use crate::data::flat::FlatLevelGeneratorSettings;
use crate::data::holder::Holder;
use crate::data::noise::NoiseGeneratorSettings;
use ahash::AHashMap;
use datapack_macros::DispatchDeserialize;
use serde::{Deserialize, Deserializer};
use util::identifier::{Identifier, IdentifierBuf};

const OVERWORLD: &Identifier = Identifier::new_const("overworld");

#[derive(Debug, Deserialize)]
pub struct WorldPreset {
    #[serde(deserialize_with = "require_overworld")]
    pub dimensions: AHashMap<IdentifierBuf, LevelStem>,
}

fn require_overworld<'de, D>(
    deserializer: D,
) -> Result<AHashMap<IdentifierBuf, LevelStem>, D::Error>
where
    D: Deserializer<'de>,
{
    let dimensions: AHashMap<IdentifierBuf, LevelStem> = Deserialize::deserialize(deserializer)?;
    if !dimensions.contains_key(OVERWORLD) {
        return Err(serde::de::Error::missing_field("minecraft:overworld"));
    }
    Ok(dimensions)
}

#[derive(Debug, Deserialize)]
pub struct LevelStem {
    #[serde(rename = "type")]
    pub ty: IdentifierBuf,
    pub generator: ChunkGenerator,
}

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum ChunkGenerator {
    Noise(NoiseBasedChunkGenerator),
    Flat(FlatLevelSource),
    Debug(DebugLevelSource),
}

#[derive(Debug, Deserialize)]
pub struct NoiseBasedChunkGenerator {
    pub biome_source: BiomeSource,
    pub settings: Holder<NoiseGeneratorSettings>,
}

#[derive(Debug, Deserialize)]
pub struct FlatLevelSource {
    pub settings: FlatLevelGeneratorSettings,
}

#[derive(Debug, Deserialize)]
pub struct DebugLevelSource {}
