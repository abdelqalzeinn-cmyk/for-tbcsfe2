use bcsfe_derive::{Readable, Writable};

use crate::blocks::gv_120000::NewChapters;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 130500)]
pub struct GV130500Block {
    pub unknown_chapters: NewChapters,
}
