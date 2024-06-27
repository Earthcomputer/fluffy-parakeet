use crate::data::block_predicate::BlockPredicate;
use crate::data::block_state::BlockState;
use crate::data::density_function::NoiseParameters;
use crate::data::value_provider::IntProvider;
use crate::data::SimpleWeightedListEntry;
use crate::serde_helpers::Ranged;
use datapack_macros::DispatchDeserialize;
use ordered_float::NotNan;
use serde::Deserialize;

#[derive(Debug, DispatchDeserialize)]
pub enum BlockStateProvider {
    SimpleStateProvider(SimpleStateProvider),
    WeightedStateProvider(WeightedStateProvider),
    NoiseThresholdProvider(NoiseThresholdStateProvider),
    NoiseProvider(NoiseStateProvider),
    DualNoiseProvider(DualNoiseStateProvider),
    RotatedBlockProvider(RotatedStateProvider),
    RandomizedIntStateProvider(RandomizedIntStateProvider),
}

#[derive(Debug, Deserialize)]
pub struct SimpleStateProvider {
    pub state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct WeightedStateProvider {
    pub entries: Vec<SimpleWeightedListEntry<BlockState>>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseBasedStateProvider {
    pub seed: i64,
    pub noise: NoiseParameters,
    #[serde(with = "crate::serde_helpers::PositiveF32")]
    pub scale: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseThresholdStateProvider {
    #[serde(flatten)]
    pub noise: NoiseBasedStateProvider,
    pub threshold: Ranged<NotNan<f32>, -1, 1>,
    pub high_chance: Ranged<NotNan<f32>, 0, 1>,
    pub default_state: BlockState,
    pub low_states: Vec<BlockState>,
    pub high_states: Vec<BlockState>,
}

#[derive(Debug, Deserialize)]
pub struct NoiseStateProvider {
    #[serde(flatten)]
    pub noise: NoiseBasedStateProvider,
    pub states: Vec<BlockState>,
}

#[derive(Debug, Deserialize)]
pub struct DualNoiseStateProvider {
    #[serde(flatten)]
    pub noise: NoiseStateProvider,
    pub variety: Ranged<i32, 1, 64>,
    pub slow_noise: NoiseParameters,
    #[serde(with = "crate::serde_helpers::PositiveF32")]
    pub slow_scale: NotNan<f32>,
}

#[derive(Debug, Deserialize)]
pub struct RotatedStateProvider {
    pub state: BlockState,
}

#[derive(Debug, Deserialize)]
pub struct RandomizedIntStateProvider {
    pub source: Box<BlockStateProvider>,
    pub property: String,
    pub values: IntProvider,
}

#[derive(Debug, Deserialize)]
pub struct RuleBasedBlockStateProvider {
    pub fallback: BlockStateProvider,
    pub rules: Vec<BlockStateProviderRule>,
}

#[derive(Debug, Deserialize)]
pub struct BlockStateProviderRule {
    pub if_true: BlockPredicate,
    pub then: BlockStateProvider,
}
