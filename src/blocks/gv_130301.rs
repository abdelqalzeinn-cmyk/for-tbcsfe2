use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthString};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130301Block {
    pub unknown: HashMapLength<i32, LengthString<i32>, (i32, f64)>, // uuid, ?, timestamp
    pub _130301: Assertable<130301>,
}
