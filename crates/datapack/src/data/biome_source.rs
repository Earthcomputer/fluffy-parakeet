use crate::data::biome::{Biome, ClimateParameterPoint};
use crate::identifier::IdentifierBuf;
use crate::serde_helpers::{DefaultOnError, InlineVec, MaybeReference, Ranged, ValueProvider};
use datapack_macros::DispatchDeserialize;
use serde::Deserialize;

#[derive(Debug, DispatchDeserialize)]
pub enum BiomeSource {
    Fixed(FixedBiomeSource),
    MultiNoise(MultiNoiseBiomeSource),
    Checkerboard(CheckerboardColumnBiomeSource),
    TheEnd(TheEndBiomeSource),
}

#[derive(Debug, Deserialize)]
pub struct FixedBiomeSource {
    biome: MaybeReference<Biome>,
}

#[derive(Debug, Deserialize)]
pub enum MultiNoiseBiomeSource {
    #[serde(rename = "preset")]
    Preset(IdentifierBuf),
    #[serde(rename = "biomes")]
    Direct(Vec<MultiNoiseBiomeSourceEntry>),
}

#[derive(Debug, Deserialize)]
pub struct MultiNoiseBiomeSourceEntry {
    parameters: ClimateParameterPoint,
    biome: MaybeReference<Biome>,
}

#[derive(Debug, Deserialize)]
pub struct CheckerboardColumnBiomeSource {
    pub biomes: InlineVec<MaybeReference<Biome>>,
    #[serde(default)]
    #[allow(private_interfaces)]
    pub scale: DefaultOnError<Ranged<u32, 0, 62>, DefaultToTwo>,
}

struct DefaultToTwo;
impl ValueProvider<Ranged<u32, 0, 62>> for DefaultToTwo {
    fn provide() -> Ranged<u32, 0, 62> {
        From::from(2)
    }
}

#[derive(Debug, Deserialize)]
pub struct TheEndBiomeSource {}
