use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100300Block {
    pub unknown: [Unknown100300; 6],
    pub _100300: Assertable<100300>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Unknown100300 {
    pub u1: bool,
    pub u2: bool,
    pub u3: i8,
    pub u4: f64,
    pub u5: f64,
}
