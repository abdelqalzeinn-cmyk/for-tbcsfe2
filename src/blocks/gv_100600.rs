use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100600BlockEn {
    pub uknown: i8,
    pub _100600: Assertable<100600>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100600Block {
    pub timestamp: f64,
    pub platinum_shards: i32,
    pub u2: bool,
    pub _100600: Assertable<100600>,
}
