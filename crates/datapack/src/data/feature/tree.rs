use serde::Deserialize;
use datapack_macros::DispatchDeserialize;
use crate::serde_helpers::DefaultOnError;

#[derive(Debug, Deserialize)]
pub struct TreeConfiguration {
    pub trunk_provider: BlockStateProvider,
    pub trunk_placer: TrunkPlacer,
    pub foliage_provider: BlockStateProvider,
    #[serde(default)]
    pub root_placer: Option<RootPlacer>,
    pub dirt_provider: BlockStateProvider,
    pub minimum_size: FeatureSize,
    pub decorators: Vec<TreeDecorator>,
    #[serde(default)]
    pub ignore_vines: DefaultOnError<bool>,
    #[serde(default)]
    pub force_dirt: DefaultOnError<bool>,
}