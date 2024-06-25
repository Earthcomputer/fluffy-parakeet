use util::identifier::IdentifierBuf;
use crate::serde_helpers::DefaultOnError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SoundEvent {
    pub sound_id: IdentifierBuf,
    pub range: DefaultOnError<f32>,
}
