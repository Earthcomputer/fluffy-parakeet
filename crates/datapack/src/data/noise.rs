use crate::data::biome::ClimateParameterPoint;
use crate::data::block_state::BlockState;
use crate::data::density_function::{deserialize_maybe_density_function, DensityFunction};
use crate::data::surface_rules::SurfaceRuleSource;
use crate::data::{DIMENSION_MAX_Y, DIMENSION_MIN_Y, DIMENSION_Y_SIZE};
use crate::serde_helpers::{MaybeReference, Ranged};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NoiseGeneratorSettings {
    pub noise: NoiseSettings,
    pub default_block: BlockState,
    pub default_fluid: BlockState,
    pub noise_router: NoiseRouter,
    pub surface_rule: SurfaceRuleSource,
    pub spawn_target: Vec<ClimateParameterPoint>,
    pub sea_level: i32,
    pub disable_mob_generation: bool,
    pub aquifers_enabled: bool,
    pub ore_veins_enabled: bool,
    pub legacy_random_source: bool,
}

#[derive(Debug, Deserialize)]
pub struct NoiseSettings {
    pub min_y: Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>,
    pub height: Ranged<u32, 0, { DIMENSION_Y_SIZE as i64 }>,
    pub size_horizontal: Ranged<u32, 1, 4>,
    pub size_vertical: Ranged<u32, 1, 4>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseRouter {
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub barrier: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub fluid_level_floodedness: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub fluid_level_spread: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub lava: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub temperature: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub vegetation: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub continents: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub erosion: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub depth: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub ridges: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub initial_density_without_jaggedness: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub final_density: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub vein_toggle: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub vein_ridged: MaybeReference<DensityFunction>,
    #[serde(deserialize_with = "deserialize_maybe_density_function")]
    pub vein_gap: MaybeReference<DensityFunction>,
}
