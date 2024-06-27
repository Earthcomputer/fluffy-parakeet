use crate::serde_helpers::DefaultOnError;
use serde::Deserialize;
use util::identifier::IdentifierBuf;

#[derive(Debug, Deserialize)]
pub struct SoundEvent {
    pub sound_id: IdentifierBuf,
    pub range: DefaultOnError<f32>,
}
