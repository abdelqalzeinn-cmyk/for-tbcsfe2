use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 56)]
pub struct GV56Block {
    pub uknown: bool,
    #[rw(with = "HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>")]
    pub item_reward_item_obtains: HashMap<i32, HashMap<i32, bool>>,
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub item_reward_unobtained_sets: HashMap<i32, bool>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub stepup_gatya_stages: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, f64>")]
    pub stepup_gatya_durations: HashMap<i32, f64>,
    pub backup_frame: i32,
}
