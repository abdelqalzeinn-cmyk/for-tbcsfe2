use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 80500)]
pub struct GV80500Block {
    #[rw(with = "LengthVec<i32, i32>")]
    pub stage_ids_10s: Vec<i32>,
}
