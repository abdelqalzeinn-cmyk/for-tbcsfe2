use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_120000::NewChapters,
    stream::{HashMapLength, LengthVec},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 140000)]
pub struct GV140000Block {
    pub u1: i32,
    pub u2: f64,
    pub u3: i8,
    pub u5: HashMapLength<i8, i32, LengthVec<i8, i8>>,
    pub dojo_chapters: NewChapters,
    pub u6: LengthVec<i16, i32>,
    pub u7: bool,
    pub u8: f64,
    pub u9: HashMapLength<i16, i16, i8>,
}
