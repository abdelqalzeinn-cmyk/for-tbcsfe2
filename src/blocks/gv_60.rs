use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 60)]
pub struct GV60Block {
    #[rw(
        max_gv = 43,
        with = "HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>"
    )]
    pub old_current_outbreaks: HashMap<i32, HashMap<i32, bool>>,
    #[rw(with = "HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>")]
    pub current_outbreaks: HashMap<i32, HashMap<i32, bool>>,
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub first_locks: HashMap<i32, bool>,
    pub energy_penalty_timestamp: f64,
}
