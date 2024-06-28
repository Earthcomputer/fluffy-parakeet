use crate::data::feature::configured_feature::ConfiguredFeature;
use crate::data::feature::placement_modifier::PlacementModifier;
use crate::data::holder::Holder;
use crate::data::{DIMENSION_MAX_Y, DIMENSION_MIN_Y};
use crate::serde_helpers::Ranged;
use ordered_float::NotNan;
use serde::Deserialize;

pub mod configured_feature;
pub mod feature_size;
mod geode;
pub mod ore;
pub mod placement_modifier;
pub mod rule_test;
pub mod tree;

#[derive(Debug, Deserialize)]
pub struct PlacedFeature {
    pub feature: Holder<ConfiguredFeature>,
    pub placement: Vec<PlacementModifier>,
}

#[derive(Debug, Deserialize)]
pub struct WeightedPlacedFeature {
    pub feature: PlacedFeature,
    pub chance: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAnchor {
    Absolute(Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>),
    AboveBottom(Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>),
    BelowTop(Ranged<i32, { DIMENSION_MIN_Y as i64 }, { DIMENSION_MAX_Y as i64 }>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaveSurface {
    Ceiling,
    Floor,
}
