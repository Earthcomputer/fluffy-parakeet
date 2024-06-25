use serde::Deserialize;
use crate::data::feature::configured_feature::ConfiguredFeature;
use crate::data::feature::placement_modifier::PlacementModifier;
use crate::data::holder::Holder;

pub mod configured_feature;
pub mod placement_modifier;
mod tree;

#[derive(Debug, Deserialize)]
pub struct PlacedFeature {
    pub feature: Holder<ConfiguredFeature>,
    pub placement: Vec<PlacementModifier>,
}