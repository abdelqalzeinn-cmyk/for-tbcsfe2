use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 61)]
pub struct GV61Block {
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub new_chara_flags: HashMap<i32, i32>,
    pub shown_maxcollab_msg: bool,
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub displayed_packs: HashMap<i32, bool>,
}
