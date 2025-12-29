use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 50)]
pub struct GV50Block {}
