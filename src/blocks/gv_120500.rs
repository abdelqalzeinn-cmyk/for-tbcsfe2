use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV120500Block {
    pub u1: bool,
    pub u2: bool,
    pub u3: bool,
    pub date: i32,
    pub u5: i8,
    pub _120500: Assertable<120500>,
}
