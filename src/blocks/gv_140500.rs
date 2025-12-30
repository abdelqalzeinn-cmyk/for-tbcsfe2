use std::collections::HashMap;

use crate::stream::{HashMapLength, LengthVec};
use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Default, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 140500)]
pub struct GV140500Block {
    #[rw(with = "HashMapLength<i8, i32, bool>")]
    pub unknown: HashMap<i32, bool>,
    #[rw(with = "HashMapLength<i8, i8, LengthVec<i8, i16>>")]
    pub unknown2: HashMap<i8, Vec<i16>>,
    #[rw(with = "HashMapLength<i32, i32, f64>")]
    pub unknown3: HashMap<i32, f64>,
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub unknown4: HashMap<i32, bool>,
}
