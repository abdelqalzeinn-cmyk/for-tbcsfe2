use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV100400Block {
    pub event_capsules_2: LengthVec<i8, i32>,
    pub two_battle_lines: bool,
    pub _100400: Assertable<100400>,
}
