use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV60Block {
    #[rw(max_gv = 43)]
    pub old_current_outbreaks: Option<HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>>,
    pub current_outbreaks: HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>,
    pub first_locks: HashMapLength<i32, i32, bool>,
    pub energy_penalty_timestamp: f64,
    pub _60: Assertable<60>,
}
