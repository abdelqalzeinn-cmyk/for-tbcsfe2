use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 90800)]
pub struct GV90800Block {
    #[rw(with = "LengthVec<i16, i32>")]
    pub u1: Vec<i32>,
    pub u2: [bool; 10],
}
