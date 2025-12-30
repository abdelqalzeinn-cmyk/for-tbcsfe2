use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV59Block {
    pub last_checked_zombie_time: f64,
    #[rw(with = "HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>")]
    pub outbreaks: HashMap<i32, HashMap<i32, bool>>,
    pub zombie_event_remaining_time: f64,
    #[rw(with = "LengthVec<i32, i32>")]
    pub scheme_items_to_obtain: Vec<i32>,
    #[rw(with = "LengthVec<i32, i32>")]
    pub scheme_items_received: Vec<i32>,
}
