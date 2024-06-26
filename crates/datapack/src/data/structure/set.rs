use crate::data::holder::Holder;
use crate::data::structure::placement::StructurePlacement;
use crate::data::structure::Structure;
use serde::Deserialize;
use util::ranged::PositiveI32;

#[derive(Debug, Deserialize)]
pub struct StructureSet {
    pub structures: Vec<StructureSelectionEntry>,
    pub placement: StructurePlacement,
}

#[derive(Debug, Deserialize)]
pub struct StructureSelectionEntry {
    pub structure: Holder<Structure>,
    pub weight: PositiveI32,
}
