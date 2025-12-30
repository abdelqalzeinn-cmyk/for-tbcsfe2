use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthString, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 65)]
pub struct GV65Block {
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub h1: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, LengthVec<i32, LengthString<i32>>>")]
    pub h2: HashMap<i32, Vec<String>>,
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub h3: HashMap<i32, bool>,
}
