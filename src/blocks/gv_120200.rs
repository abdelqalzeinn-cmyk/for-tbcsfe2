use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120200Block {
    pub u1: bool,
    pub u2: i16,
    pub u3: HashMapLength<i8, i16, i16>,
    pub _120200: Assertable<120200>,
}
