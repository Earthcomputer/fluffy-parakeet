use crate::built_in_registries::Block;
use crate::data::block_state::BlockState;
use crate::data::feature::VerticalAnchor;
use crate::data::height_provider::HeightProvider;
use crate::data::tag::HolderSet;
use crate::data::value_provider::FloatProvider;
use crate::float_provider_deserializer;
use crate::serde_helpers::{NonNegativeU32, Ranged};
use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};
use ordered_float::NotNan;
use serde::{Deserialize};
use std::collections::BTreeMap;
use util::identifier::IdentifierBuf;
use crate::data::feature::configured_feature::ProbabilityFeatureConfiguration;

#[derive(Debug, DispatchDeserialize)]
pub enum ConfiguredWorldCarver {
    Cave(CaveCarverConfiguration),
    NetherCave(CaveCarverConfiguration),
    Canyon(CanyonCarverConfiguration),
}

#[derive(Debug, Deserialize)]
pub struct CarverConfiguration {
    #[serde(flatten)]
    pub probability: ProbabilityFeatureConfiguration,
    pub y: AnchorOrHeightProvider,
    #[serde(rename = "yScale")]
    pub y_scale: FloatProvider,
    pub lava_level: VerticalAnchor,
    pub debug_settings: Option<CarverDebugSettings>,
    pub replaceable: HolderSet<Block>,
}

#[derive(Debug, Deserialize)]
pub struct CarverDebugSettings {
    #[serde(default)]
    pub debug_mode: bool,
    #[serde(default = "debug_air_state")]
    pub air_state: BlockState,
    #[serde(default = "debug_air_state")]
    pub water_state: BlockState,
    #[serde(default = "debug_air_state")]
    pub lava_state: BlockState,
    #[serde(default = "debug_air_state")]
    pub barrier_state: BlockState,
}

fn debug_air_state() -> BlockState {
    BlockState {
        name: IdentifierBuf::new("minecraft:acacia_button").unwrap(),
        properties: BTreeMap::new(),
    }
}

#[derive(Debug, UntaggedDeserialize)]
pub enum AnchorOrHeightProvider {
    Anchor(VerticalAnchor),
    HeightProvider(HeightProvider),
}

#[derive(Debug, Deserialize)]
pub struct CaveCarverConfiguration {
    #[serde(flatten)]
    pub base: CarverConfiguration,
    pub horizontal_radius_multiplier: FloatProvider,
    pub vertical_radius_multiplier: FloatProvider,

    #[serde(deserialize_with = "deserialize_floor_level")]
    pub floor_level: FloatProvider,
}

float_provider_deserializer!(deserialize_floor_level, -1.0, 1.0);

#[derive(Debug, Deserialize)]
pub struct CanyonCarverConfiguration {
    #[serde(flatten)]
    pub base: CarverConfiguration,
    pub vertical_rotation: FloatProvider,
    pub shape: CanyonShapeConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct CanyonShapeConfiguration {
    pub distance_factor: NotNan<f32>,
    pub thickness: NotNan<f32>,
    pub width_smoothness: NonNegativeU32,
    pub horizontal_radius_factor: FloatProvider,
    pub vertical_radius_default_factor: NotNan<f32>,
    pub vertical_radius_center_factor: NotNan<f32>,
}
