use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV52Block {
    pub catguide_collected: LengthVec<i32, bool>,
    pub _52: Assertable<52>,
}
