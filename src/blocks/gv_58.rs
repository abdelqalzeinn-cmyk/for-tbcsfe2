use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV58Block {
    pub dojo_chapters: HashMapLength<i32, i32, HashMapLength<i32, i32, i32>>,
    pub dojo_item_lock_flag: bool,
    pub dojo_item_locks: [bool; TOTAL_BATTLE_ITEMS],
    pub _58: Assertable<58>,
}
pub const TOTAL_BATTLE_ITEMS: usize = 6;
