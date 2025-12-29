use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 110700)]
pub struct GV110700Block {
    pub u1: HashMapLength<i32, i32, (f64, f64)>,
    #[rw(jp = false)]
    pub u2: Option<bool>,
}
