use crate::data::biome::ClimateParameterPoint;
use crate::data::block_state::BlockState;
use crate::data::density_function::{deserialize_density_function_holder, DensityFunction};
use crate::data::holder::Holder;
use crate::data::surface_rules::SurfaceRuleSource;
use crate::data::{DIMENSION_MAX_Y, DIMENSION_MIN_Y, DIMENSION_Y_SIZE};
use crate::serde_helpers::Ranged;
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
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub barrier: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub fluid_level_floodedness: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub fluid_level_spread: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub lava: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub temperature: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub vegetation: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub continents: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub erosion: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub depth: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub ridges: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub initial_density_without_jaggedness: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub final_density: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub vein_toggle: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub vein_ridged: Holder<DensityFunction>,
    #[serde(deserialize_with = "deserialize_density_function_holder")]
    pub vein_gap: Holder<DensityFunction>,
}
