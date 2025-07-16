use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV56Block {
    pub uknown: bool,
    pub item_reward_item_obtains: HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>,
    pub item_reward_unobtained_sets: HashMapLength<i32, i32, bool>,
    pub stepup_gatya_stages: HashMapLength<i32, i32, i32>,
    pub stepup_gatya_durations: HashMapLength<i32, i32, f64>,
    pub backup_frame: i32,
    pub _56: Assertable<56>,
}
