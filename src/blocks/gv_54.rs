use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthString, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV54Block {
    pub gamatoto_helpers: LengthVec<i32, i32>,
    pub is_ad_present: bool,
    pub _54: Assertable<54>,
    pub item_pack: HashMapLength<i32, i32, HashMapLength<i32, LengthString<i32>, bool>>,
    pub _54_2: Assertable<54>,
}
