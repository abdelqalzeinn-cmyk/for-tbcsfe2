use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV71Block {
    pub gamatoto_collab_flags: HashMapLength<i32, i32, bool>,
    pub gamatoto_collab_durations: HashMapLength<i32, i32, f64>,
    pub _71: Assertable<71>,
}
