use serde::Deserialize;
use std::collections::BTreeMap;
use util::identifier::IdentifierBuf;

#[derive(Debug, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    pub name: IdentifierBuf,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
}

pub type FluidState = BlockState;
