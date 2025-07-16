use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV59Block {
    pub last_checked_zombie_time: f64,
    pub outbreaks: HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>,
    pub zombie_event_remaining_time: f64,
    pub scheme_items_to_obtain: LengthVec<i32, i32>,
    pub scheme_items_received: LengthVec<i32, i32>,
}
