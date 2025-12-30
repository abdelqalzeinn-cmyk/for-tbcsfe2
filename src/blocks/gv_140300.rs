use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 140300)]
pub struct GV140300Block {
    #[rw(with = "LengthVec<i8, i8>")]
    pub u1: Vec<i8>,
    pub u2: bool,
    #[rw(with = "LengthVec<i8, i32>")]
    pub treasure_chests: Vec<i32>,
    pub u3: i32,
    #[rw(with = "LengthVec<i16, i32>")]
    pub u4: Vec<i32>,
    pub u5: bool,
}
