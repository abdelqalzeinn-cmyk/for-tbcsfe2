use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 71)]
pub struct GV71Block {
    pub gamatoto_collab_flags: HashMapLength<i32, i32, bool>,
    pub gamatoto_collab_durations: HashMapLength<i32, i32, f64>,
}
