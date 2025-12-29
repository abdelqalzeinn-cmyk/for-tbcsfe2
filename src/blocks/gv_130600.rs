use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 130600)]
pub struct GV130600Block {
    pub u1: i8,
    #[rw(jp = false)]
    pub u2: Option<i16>,
}
