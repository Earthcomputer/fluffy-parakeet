use crate::identifier::IdentifierBuf;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct BlockState {
    #[serde(rename = "Name")]
    pub name: IdentifierBuf,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
}
