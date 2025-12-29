use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV111000Block {
    pub u1: i32,
    pub u2: i16,
    pub u3: i8,
    pub u4: i8,
    pub u5: bool,
    pub u6: i8,
    pub u7: LengthVec<i8, i16>,
    pub u8: LengthVec<i16, i16>,
    pub u9: LengthVec<i16, i16>,
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
    pub u25: LengthVec<i16, i16>,
    pub u26: [bool; 14],
    pub labyrinth_medals: LengthVec<i8, i16>,
    pub _111000: Assertable<111000>,
}
