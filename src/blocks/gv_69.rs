use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 69)]
pub struct GV69Block {
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub event_completed_one_level_in_chapter: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub event_displayed_cleared_limit_text: HashMap<i32, bool>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub event_start_dates: HashMap<i32, i32>,
    #[rw(with = "LengthVec<i32, i32>")]
    pub stages_reward_claimed: Vec<i32>,
    pub cotc_1_complete: i32,
}
