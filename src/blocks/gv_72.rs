use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthVec};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapResetData {
    pub yearly_end_timestamp: f64,
    pub monthly_end_timestamp: f64,
    pub weekly_end_timestamp: f64,
    pub daily_end_timestamp: f64,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV72Block {
    pub map_resets: HashMapLength<i32, i32, LengthVec<i32, MapResetData>>,
    pub _72: Assertable<72>,
}
