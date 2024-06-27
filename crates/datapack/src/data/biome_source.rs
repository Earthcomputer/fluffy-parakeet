use crate::data::biome::{Biome, ClimateParameterPoint};
use crate::data::holder::Holder;
use crate::data::tag::HolderValueSet;
use crate::serde_helpers::{DefaultOnError, Ranged, ValueProvider};
use datapack_macros::DispatchDeserialize;
use serde::Deserialize;
use util::identifier::IdentifierBuf;

#[derive(Debug, DispatchDeserialize)]
pub enum BiomeSource {
    Fixed(FixedBiomeSource),
    MultiNoise(MultiNoiseBiomeSource),
    Checkerboard(CheckerboardColumnBiomeSource),
    TheEnd(TheEndBiomeSource),
}

#[derive(Debug, Deserialize)]
pub struct FixedBiomeSource {
    pub biome: Holder<Biome>,
}

#[derive(Debug, Deserialize)]
pub enum MultiNoiseBiomeSource {
    #[serde(rename = "preset")]
    Preset(Holder<MultiNoiseBiomeSourceParameterList>),
    #[serde(rename = "biomes")]
    Direct(Vec<MultiNoiseBiomeSourceEntry>),
}

#[derive(Debug, Deserialize)]
pub struct MultiNoiseBiomeSourceParameterList {
    // See MultiNoiseBiomeSourceParameterList.Preset for implementations.
    pub preset: IdentifierBuf,
}

#[derive(Debug, Deserialize)]
pub struct MultiNoiseBiomeSourceEntry {
    pub parameters: ClimateParameterPoint,
    pub biome: Holder<Biome>,
}

#[derive(Debug, Deserialize)]
pub struct CheckerboardColumnBiomeSource {
    pub biomes: HolderValueSet<Biome>,
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
