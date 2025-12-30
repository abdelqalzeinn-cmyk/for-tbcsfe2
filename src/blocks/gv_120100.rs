use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 120100)]
pub struct GV120100Block {
    #[rw(with = "LengthVec<i16, i16>")]
    pub unknown: Vec<i16>,
}
