use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 58)]
pub struct GV58Block {
    pub dojo_chapters: HashMapLength<i32, i32, HashMapLength<i32, i32, i32>>,
    pub dojo_item_lock_flag: bool,
    pub dojo_item_locks: [bool; TOTAL_BATTLE_ITEMS],
}
pub const TOTAL_BATTLE_ITEMS: usize = 6;
