use std::collections::HashMap;

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
    #[rw(with = "HashMapLength<i8, i32, LengthVec<i8, i8>>")]
    pub u5: HashMap<i32, Vec<i8>>,
    pub dojo_chapters: NewChapters,
    #[rw(with = "LengthVec<i16, i32>")]
    pub u6: Vec<i32>,
    pub u7: bool,
    pub u8: f64,
    #[rw(with = "HashMapLength<i16, i16, i8>")]
    pub u9: HashMap<i16, i8>,
}
