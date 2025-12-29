use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 61)]
pub struct GV61Block {
    pub new_chara_flags: HashMapLength<i32, i32, i32>,
    pub shown_maxcollab_msg: bool,
    pub displayed_packs: HashMapLength<i32, i32, bool>,
}
