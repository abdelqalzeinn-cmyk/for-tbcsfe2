use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 120000)]
pub struct GV120000Block {
    pub zero_legends: NewChapters,
    pub unknown: i8,
}
#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewMap {
    pub selected_stage: i8,
    pub clear_progress: i8,
    pub unlock_state: i8,
    #[rw(with = "LengthVec<i16, i16>")]
    pub stages: Vec<i16>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewChapters {
    #[rw(with = "LengthVec<i16, NewChapter>")]
    pub chapters: Vec<NewChapter>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewChapter {
    pub unknown: i8,
    #[rw(with = "LengthVec<i8, NewMap>")]
    pub maps: Vec<NewMap>,
}
