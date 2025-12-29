use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 68)]
pub struct GV68Block {
    pub weekly_missions: HashMapLength<i32, i32, bool>,
    pub dojo_ranking_did_win_rewards: bool,
    pub event_update: bool,
}
