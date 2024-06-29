use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum CarvingStep {
    Air,
    Liquid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum DecorationStep {
    RawGeneration,
    Lakes,
    LocalModifications,
    UndergroundStructures,
    SurfaceStructures,
    Strongholds,
    UndergroundOres,
    UndergroundDecoration,
    FluidSprings,
    VegetalDecoration,
    TopLayerModification,
}
