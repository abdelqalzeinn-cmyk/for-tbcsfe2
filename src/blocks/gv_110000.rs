use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV110000Block {
    pub u1: HashMapLength<i16, i32, (i8, i8)>,
    pub _110000: Assertable<110000>,
}
