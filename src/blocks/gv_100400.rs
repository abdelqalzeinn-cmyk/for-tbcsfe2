use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100400)]
pub struct GV100400Block {
    #[rw(with = "LengthVec<i8, i32>")]
    pub event_capsules_2: Vec<i32>,
    pub two_battle_lines: bool,
}
