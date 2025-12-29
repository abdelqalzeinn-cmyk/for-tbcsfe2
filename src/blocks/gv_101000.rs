use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 101000)]
pub struct GV101000Block {
    pub uknown: i8,
}
