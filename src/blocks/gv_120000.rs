use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV120000Block {
    pub zero_legends: NewChapters,
    pub unknown: i8,
    pub _120000: Assertable<120000>,
}
#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewChapter {
    pub selected_stage: i8,
    pub clear_progress: i8,
    pub unlock_state: i8,
    pub stages: LengthVec<i16, i16>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewChapters {
    pub chapters: LengthVec<i16, (i8, LengthVec<i8, NewChapter>)>,
}
