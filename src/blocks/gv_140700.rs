use crate::stream::Assertable;

use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable)]
pub struct GV140700Block {
    pub b1: bool,
    pub b2: bool,
    pub d1: f64,
    pub d2: f64,
    pub b3: bool,
    pub _140700: Assertable<140700>,
}
