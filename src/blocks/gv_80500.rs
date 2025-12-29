use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV80500Block {
    pub unknown: LengthVec<i32, i32>,
    pub _80500: Assertable<80500>,
}
