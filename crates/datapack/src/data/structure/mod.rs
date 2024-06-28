use datapack_macros::DispatchDeserialize;

pub mod placement;
pub mod processor;
pub mod set;

#[derive(Debug, DispatchDeserialize)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum Structure {
    // TODO
}
