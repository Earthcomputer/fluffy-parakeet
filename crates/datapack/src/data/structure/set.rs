use crate::data::holder::Holder;
use crate::data::structure::placement::StructurePlacement;
use crate::data::structure::Structure;
use crate::serde_helpers::PositiveU32;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StructureSet {
    pub structures: Vec<StructureSelectionEntry>,
    pub placement: StructurePlacement,
}

#[derive(Debug, Deserialize)]
pub struct StructureSelectionEntry {
    pub structure: Holder<Structure>,
    pub weight: PositiveU32,
}
