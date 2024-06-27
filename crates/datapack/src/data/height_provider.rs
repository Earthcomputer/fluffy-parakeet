use crate::data::feature::VerticalAnchor;
use crate::data::SimpleWeightedListEntry;
use crate::serde_helpers::{NonNegativeU32, Ranged};
use datapack_macros::DispatchDeserialize;
use serde::Deserialize;

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum HeightProvider {
    BasedToBottomHeight(BiasedOrVeryBiasedToBottomHeight),
    ConstantHeight(ConstantHeight),
    TrapezoidHeight(TrapezoidHeight),
    UniformHeight(UniformHeight),
    VeryBiasedToBottomHeight(BiasedOrVeryBiasedToBottomHeight),
    WeightedListHeight(WeightedListHeight),
}

#[derive(Debug, Deserialize)]
pub struct BiasedOrVeryBiasedToBottomHeight {
    pub min_inclusive: VerticalAnchor,
    pub max_inclusive: VerticalAnchor,
    #[serde(default = "one")]
    pub inner: NonNegativeU32,
}

fn one() -> NonNegativeU32 {
    Ranged::from(1)
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ConstantHeight(pub VerticalAnchor);

#[derive(Debug, Deserialize)]
pub struct TrapezoidHeight {
    pub min_inclusive: VerticalAnchor,
    pub max_inclusive: VerticalAnchor,
    #[serde(default)]
    pub plateau: i32,
}

#[derive(Debug, Deserialize)]
pub struct UniformHeight {
    pub min_inclusive: VerticalAnchor,
    pub max_inclusive: VerticalAnchor,
}

#[derive(Debug, Deserialize)]
pub struct WeightedListHeight {
    pub distribution: Vec<SimpleWeightedListEntry<HeightProvider>>,
}
