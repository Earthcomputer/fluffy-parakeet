use crate::data::carvers::ConfiguredWorldCarver;
use crate::data::feature::PlacedFeature;
use crate::data::holder::Holder;
use crate::data::sound_event::SoundEvent;
use crate::data::step::CarvingStep;
use crate::data::tag::HolderValueSet;
use crate::data::Interval;
use ahash::AHashMap;

use serde::Deserialize;
use util::identifier::IdentifierBuf;
use util::ranged::{PositiveI32, Ranged};

#[derive(Debug, Deserialize)]
pub struct Biome {
    #[serde(flatten)]
    pub climate_settings: ClimateSettings,
    pub effects: BiomeSpecialEffects,

    #[serde(flatten)]
    pub generation_settings: BiomeGenerationSettings,

    #[serde(flatten)]
    pub mob_settings: MobSpawnSettings,
}

#[derive(Debug, Deserialize)]
pub struct ClimateSettings {
    pub has_precipitation: bool,
    pub temperature: f32,
    #[serde(default)]
    pub temperature_modifier: TemperatureModifier,
    pub downfall: f32,
}

#[derive(Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum TemperatureModifier {
    #[default]
    None,
    Frozen,
}

#[derive(Debug, Deserialize)]
pub struct BiomeSpecialEffects {
    pub fog_color: i32,
    pub water_color: i32,
    pub water_fog_color: i32,
    pub sky_color: i32,
    #[serde(default)]
    pub foliage_color: Option<i32>,
    #[serde(default)]
    pub grass_color: Option<i32>,
    #[serde(default)]
    pub grass_color_modifier: Option<GrassColorModifier>,
    #[serde(default)]
    pub particle: Option<AmbientParticleSettings>,
    #[serde(default)]
    pub ambient_sound: Option<Holder<SoundEvent>>,
    #[serde(default)]
    pub mood_sound: Option<AmbientMoodSettings>,
    #[serde(default)]
    pub additions_sound: Option<AmbientAdditionsSettings>,
    #[serde(default)]
    pub music: Option<Music>,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum GrassColorModifier {
    None,
    DarkForest,
    Swamp,
}

#[derive(Debug, Deserialize)]
pub struct AmbientParticleSettings {
    // TODO(feat/particles)
    // pub options: ParticleTypes,
    pub probability: f32,
}

#[derive(Debug, Deserialize)]
pub struct AmbientMoodSettings {
    pub sound: Holder<SoundEvent>,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

#[derive(Debug, Deserialize)]
pub struct AmbientAdditionsSettings {
    pub sound: Holder<SoundEvent>,
    pub tick_chance: f64,
}

#[derive(Debug, Deserialize)]
pub struct Music {
    pub sound: Holder<SoundEvent>,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}

#[derive(Debug, Deserialize)]
pub struct BiomeGenerationSettings {
    pub carvers: AHashMap<CarvingStep, HolderValueSet<ConfiguredWorldCarver>>,
    pub features: Vec<Vec<Holder<PlacedFeature>>>,
}

#[derive(Debug, Deserialize)]
pub struct MobSpawnSettings {
    #[serde(default = "default_creature_spawn_probability")]
    pub creature_spawn_probability: Ranged<f32, 0, 9999999, 10000000>,
    pub spawners: AHashMap<MobCategory, Vec<SpawnerData>>,
    pub spawn_costs: AHashMap<IdentifierBuf, MobSpawnCost>,
}

fn default_creature_spawn_probability() -> Ranged<f32, 0, 9999999, 10000000> {
    Ranged::new(0.1).unwrap()
}

#[derive(Debug, Deserialize)]
pub struct SpawnerData {
    // TODO this is an entity type
    #[serde(rename = "type")]
    pub ty: IdentifierBuf,
    pub weight: u32,
    #[serde(rename = "minCount")]
    pub min_count: PositiveI32,
    #[serde(rename = "maxCount")]
    pub max_count: PositiveI32,
}

#[derive(Debug, Deserialize)]
pub struct MobSpawnCost {
    pub energy_budget: f64,
    pub charge: f64,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum MobCategory {
    Monster,
    Creature,
    Ambient,
    Axolotls,
    UndergroundWaterCreature,
    WaterCreature,
    WaterAmbient,
    Misc,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ClimateParameter {
    pub interval: Interval<Ranged<f32, -2, 2>>,
}

#[derive(Debug, Deserialize)]
pub struct ClimateParameterPoint {
    pub temperature: ClimateParameter,
    pub humidity: ClimateParameter,
    pub continentalness: ClimateParameter,
    pub erosion: ClimateParameter,
    pub depth: ClimateParameter,
    pub weirdness: ClimateParameter,
    pub offset: Ranged<f32, 0, 1>,
}
