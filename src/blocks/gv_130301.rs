use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthString};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 130301)]
pub struct GV130301Block {
    #[rw(with = "HashMapLength<i32, LengthString<i32>, (i32, f64)>")]
    pub unknown: HashMap<String, (i32, f64)>, // uuid, ?, timestamp
}
