use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV120400Block {
    pub timestamp1: f64,
    pub timestamp2: f64,
    pub _120400: Assertable<120400>,
}
