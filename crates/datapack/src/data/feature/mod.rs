use serde::Deserialize;
use crate::data::feature::placement_modifier::PlacementModifier;

pub mod placement_modifier;
mod configured_feature;
mod tree;

#[derive(Debug, Deserialize)]
pub struct PlacedFeature {
    pub feature: ConfiguredFeature,
    pub placement: Vec<PlacementModifier>,
}