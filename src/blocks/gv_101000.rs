use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
pub struct GV101000Block {
    pub uknown: i8,
    pub _101000: Assertable<101000>,
}
