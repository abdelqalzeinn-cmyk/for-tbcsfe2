use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Copy, Clone, Default, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 51)]
pub struct GV51Block {}
