use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100100Block {
    pub date: i32,
    pub _100100: Assertable<100100>,
}
