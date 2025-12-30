use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 80700)]
pub struct GV80700Block {
    #[rw(with = "HashMapLength<i32, i32, LengthVec<i32, i32>>")]
    pub unknown: HashMap<i32, Vec<i32>>,
}
