use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90800Block {
    pub u1: LengthVec<i16, i32>,
    pub u2: [bool; 10],
    pub _90800: Assertable<90800>,
}
