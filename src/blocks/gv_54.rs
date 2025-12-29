use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthString, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 54)]
pub struct GV54Block1 {
    pub gamatoto_helpers: LengthVec<i32, i32>,
    pub is_ad_present: bool,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 54)]
pub struct GV54Block {
    pub block_1: GV54Block1,
    pub item_pack: HashMapLength<i32, i32, HashMapLength<i32, LengthString<i32>, bool>>,
}
