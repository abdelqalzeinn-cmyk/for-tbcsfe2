use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 57)]
pub struct GV57Block {
    pub unknown: bool,
    pub favourite_cats: HashMapLength<i32, i32, bool>,
}
