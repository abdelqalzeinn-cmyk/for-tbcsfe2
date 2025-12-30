use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 52)]
pub struct GV52Block {
    #[rw(with = "LengthVec<i32, bool>")]
    pub catguide_collected: Vec<bool>,
}
