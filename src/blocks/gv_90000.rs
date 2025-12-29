use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_90100::EventStartTimes90100,
    stream::{HashMapLength, LengthVec},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 90000)]
pub struct GV90000Block {
    pub medals: Medals,
    #[rw(gvcc)]
    pub wildcat_slots: GamblingEvent,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GamblingEvent {
    pub completed: HashMapLength<i16, i16, bool>,
    pub values: HashMapLength<i16, i16, HashMapLength<i16, i16, i16>>,
    #[rw(gvcc)]
    pub start_times: EventStartTimes90100,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Medals {
    pub u1: i32,
    pub u2: i32,
    pub u3: i32,
    pub data_1: LengthVec<i16, i16>,
    pub data_2: HashMapLength<i16, i16, i8>,
    pub u4: bool,
}
