use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 120500)]
pub struct GV120500Block {
    pub u1: bool,
    pub u2: bool,
    pub u3: bool,
    pub date: i32,
    pub u5: i8,
}
