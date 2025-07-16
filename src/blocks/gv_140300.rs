use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV140300Block {
    pub u1: LengthVec<i8, i8>,
    pub u2: bool,
    pub treasure_chests: LengthVec<i8, i32>,
    pub u3: i32,
    pub u4: LengthVec<i16, i32>,
    pub u5: bool,
    pub _140300: Assertable<140300>,
}
