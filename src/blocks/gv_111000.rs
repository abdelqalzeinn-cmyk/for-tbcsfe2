use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 111000)]
pub struct GV111000Block {
    pub u1: i32,
    pub u2: i16,
    pub u3: i8,
    pub u4: i8,
    pub u5: bool,
    pub u6: i8,
    #[rw(with = "LengthVec<i8, i16>")]
    pub u7: Vec<i16>,
    #[rw(with = "LengthVec<i16, i16>")]
    pub u8: Vec<i16>,
    #[rw(with = "LengthVec<i16, i16>")]
    pub u9: Vec<i16>,
    pub u10: i32,
    pub u11: i32,
    pub date1: i32,
    pub date2: i16,
    pub u14: i16,
    pub u15: i16,
    pub u16: i16,
    pub u17: i8,
    pub u18: bool,
    pub u19: bool,
    pub u20: bool,
    pub u21: bool,
    pub u22: bool,
    pub u23: bool,
    pub u24: i8,
    #[rw(with = "LengthVec<i16, i16>")]
    pub u25: Vec<i16>,
    pub u26: [bool; 14],
    #[rw(with = "LengthVec<i8, i16>")]
    pub labyrinth_medals: Vec<i16>,
}
