use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120100Block {
    pub unknown: LengthVec<i16, i16>,
    pub _120100: Assertable<120100>,
}
