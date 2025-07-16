use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV80700Block {
    pub unknown: HashMapLength<i32, i32, LengthVec<i32, i32>>,
    pub _80700: Assertable<80700>,
}
