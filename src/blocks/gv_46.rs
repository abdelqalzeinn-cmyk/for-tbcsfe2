use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 46)]
pub struct GV46Block {}
