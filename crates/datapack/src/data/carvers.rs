use crate::data::block_state::BlockState;
use crate::data::height_provider::HeightProvider;
use crate::data::surface_rules::VerticalAnchor;
use crate::data::value_provider::FloatProvider;
use crate::identifier::IdentifierBuf;
use crate::serde_helpers::{InlineVec, Ranged, RangedNonNegativeU32};
use datapack_macros::{DispatchDeserialize, UntaggedDeserialize};
use ordered_float::NotNan;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, DispatchDeserialize)]
pub enum ConfiguredWorldCarver {
    Cave(CaveCarverConfiguration),
    NetherCave(CaveCarverConfiguration),
    Canyon(CanyonCarverConfiguration),
}

#[derive(Debug, Deserialize)]
pub struct CarverConfiguration {
    // TODO(feat/features): move to a ProbabilityFeatureConfiguration base once that exists.
    pub probability: Ranged<NotNan<f32>, 0, 1>,
    pub y: AnchorOrHeightProvider,
    #[serde(rename = "yScale")]
    pub y_scale: FloatProvider,
    pub lava_level: VerticalAnchor,
    pub debug_settings: Option<CarverDebugSettings>,
    pub replaceable: InlineVec<IdentifierBuf>,
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

    // TODO: this is ranged -1 to 1
    pub floor_level: FloatProvider,
}

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
    pub width_smoothness: RangedNonNegativeU32,
    pub horizontal_radius_factor: FloatProvider,
    pub vertical_radius_default_factor: NotNan<f32>,
    pub vertical_radius_center_factor: NotNan<f32>,
}
