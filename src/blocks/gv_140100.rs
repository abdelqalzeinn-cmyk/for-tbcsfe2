use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV140100Block {
    pub unknown: i8,
    pub _140100: Assertable<140100>,
}
