use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 120400)]
pub struct GV120400Block {
    pub timestamp1: f64,
    pub timestamp2: f64,
}
