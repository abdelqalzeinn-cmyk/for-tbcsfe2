use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthString, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 65)]
pub struct GV65Block {
    pub h1: HashMapLength<i32, i32, i32>,
    pub h2: HashMapLength<i32, i32, LengthVec<i32, LengthString<i32>>>,
    pub h3: HashMapLength<i32, i32, bool>,
}
