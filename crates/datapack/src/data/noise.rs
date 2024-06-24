use crate::data::biome::ClimateParameterPoint;
use crate::data::block_state::BlockState;
use crate::data::density_function::DensityFunction;
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
    pub barrier: Holder<DensityFunction>,
    pub fluid_level_floodedness: Holder<DensityFunction>,
    pub fluid_level_spread: Holder<DensityFunction>,
    pub lava: Holder<DensityFunction>,
    pub temperature: Holder<DensityFunction>,
    pub vegetation: Holder<DensityFunction>,
    pub continents: Holder<DensityFunction>,
    pub erosion: Holder<DensityFunction>,
    pub depth: Holder<DensityFunction>,
    pub ridges: Holder<DensityFunction>,
    pub initial_density_without_jaggedness: Holder<DensityFunction>,
    pub final_density: Holder<DensityFunction>,
    pub vein_toggle: Holder<DensityFunction>,
    pub vein_ridged: Holder<DensityFunction>,
    pub vein_gap: Holder<DensityFunction>,
}
