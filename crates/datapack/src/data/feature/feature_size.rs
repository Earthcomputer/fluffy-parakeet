use crate::serde_helpers::{DefaultOnError, DefaultToRanged};
use datapack_macros::DispatchDeserialize;
use serde::Deserialize;
use util::ranged::Ranged;

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum FeatureSize {
    TwoLayersFeatureSize(TwoLayersFeatureSize),
    ThreeLayersFeatureSize(ThreeLayersFeatureSize),
}

#[derive(Debug, Deserialize)]
pub struct TwoLayersFeatureSize {
    pub limit: Ranged<u32, 0, 81>,
    pub lower_size: Ranged<u32, 0, 16>,
    pub upper_size: Ranged<u32, 0, 16>,
    #[serde(default)]
    pub min_clipped_height: Option<Ranged<u32, 0, 80>>,
}

#[derive(Debug, Deserialize)]
pub struct ThreeLayersFeatureSize {
    #[serde(default)]
    pub limit: DefaultOnError<Ranged<u32, 0, 80>, DefaultToRanged<1>>,
    #[serde(default)]
    pub upper_limit: DefaultOnError<Ranged<u32, 0, 80>, DefaultToRanged<1>>,
    #[serde(default)]
    pub lower_size: DefaultOnError<Ranged<u32, 0, 16>>,
    #[serde(default)]
    pub middle_size: DefaultOnError<Ranged<u32, 0, 16>, DefaultToRanged<1>>,
    #[serde(default)]
    pub upper_size: DefaultOnError<Ranged<u32, 0, 16>, DefaultToRanged<1>>,
    #[serde(default)]
    pub min_clipped_height: Option<Ranged<u32, 0, 80>>,
}
