use crate::stream::{HashMapLength, LengthVec};
use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Default, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 140500)]
pub struct GV140500Block {
    pub unknown: HashMapLength<i8, i32, bool>,
    pub unknown2: HashMapLength<i8, i8, LengthVec<i8, i16>>,
    pub unknown3: HashMapLength<i32, i32, f64>,
    pub unknown4: HashMapLength<i32, i32, bool>,
}
