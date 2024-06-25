use crate::data::carvers::ConfiguredWorldCarver;
use crate::data::holder::Holder;
use crate::data::sound_event::SoundEvent;
use crate::data::Interval;
use util::identifier::IdentifierBuf;
use crate::serde_helpers::{InlineVec, Ranged, RangedPositiveU32};
use ahash::AHashMap;
use ordered_float::NotNan;
use serde::Deserialize;
use crate::data::feature::PlacedFeature;

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
    pub temperature: NotNan<f32>,
    #[serde(default)]
    pub temperature_modifier: TemperatureModifier,
    pub downfall: NotNan<f32>,
}

#[derive(Debug, Default, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
pub enum GrassColorModifier {
    None,
    DarkForest,
    Swamp,
}

#[derive(Debug, Deserialize)]
pub struct AmbientParticleSettings {
    // TODO(feat/particles)
    // pub options: ParticleTypes,
    pub probability: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct AmbientMoodSettings {
    pub sound: Holder<SoundEvent>,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: NotNan<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AmbientAdditionsSettings {
    pub sound: Holder<SoundEvent>,
    pub tick_chance: NotNan<f64>,
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
    pub carvers: AHashMap<GenerationStepCarving, InlineVec<Holder<ConfiguredWorldCarver>>>,
    // TODO(feat/features)
    pub features: Vec<Vec<Holder<PlacedFeature>>>,
}

#[derive(Debug, Deserialize)]
pub struct MobSpawnSettings {
    #[serde(default = "default_creature_spawn_probability")]
    pub creature_spawn_probability: Ranged<NotNan<f32>, 0, 9999999, 10000000>,
    pub spawners: AHashMap<MobCategory, Vec<SpawnerData>>,
    pub spawn_costs: AHashMap<IdentifierBuf, MobSpawnCost>,
}

fn default_creature_spawn_probability() -> Ranged<NotNan<f32>, 0, 9999999, 10000000> {
    From::from(NotNan::new(0.1).unwrap())
}

#[derive(Debug, Deserialize)]
pub struct SpawnerData {
    // TODO this is an entity type
    #[serde(rename = "type")]
    pub ty: IdentifierBuf,
    pub weight: u32,
    #[serde(rename = "minCount")]
    pub min_count: RangedPositiveU32,
    #[serde(rename = "maxCount")]
    pub max_count: RangedPositiveU32,
}

#[derive(Debug, Deserialize)]
pub struct MobSpawnCost {
    pub energy_budget: NotNan<f64>,
    pub charge: NotNan<f64>,
}

// TODO(joe): move to mod.rs
#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GenerationStepCarving {
    Air,
    Liquid,
}

#[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
    pub interval: Interval<Ranged<NotNan<f32>, -2, 2>>,
}

#[derive(Debug, Deserialize)]
pub struct ClimateParameterPoint {
    pub temperature: ClimateParameter,
    pub humidity: ClimateParameter,
    pub continentalness: ClimateParameter,
    pub erosion: ClimateParameter,
    pub depth: ClimateParameter,
    pub weirdness: ClimateParameter,
    pub offset: Ranged<NotNan<f32>, 0, 1>,
}
