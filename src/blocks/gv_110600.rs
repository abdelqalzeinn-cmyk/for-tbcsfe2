use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV110600Block {
    pub unknown: bool,
    pub _110600: Assertable<110600>,
}
