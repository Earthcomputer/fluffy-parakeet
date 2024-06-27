use crate::data::block_state::BlockState;
use crate::data::feature::rule_test::RuleTest;
use crate::serde_helpers::Ranged;
use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OreConfiguration {
    pub targets: Vec<TargetBlockState>,
    pub size: Ranged<u32, 0, 64>,
    pub discard_chance_on_air_exposure: Ranged<NotNan<f32>, 0, 1>,
}

#[derive(Debug, Deserialize)]
pub struct TargetBlockState {
    pub target: RuleTest,
    pub state: BlockState,
}
