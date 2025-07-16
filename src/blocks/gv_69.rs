use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV69Block {
    pub event_completed_one_level_in_chapter: HashMapLength<i32, i32, i32>,
    pub event_displayed_cleared_limit_text: HashMapLength<i32, i32, bool>,
    pub event_start_dates: HashMapLength<i32, i32, i32>,
    pub stages_reward_claimed: LengthVec<i32, i32>,
    pub cotc_1_complete: i32,
    pub _69: Assertable<69>,
}
