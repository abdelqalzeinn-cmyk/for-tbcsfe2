use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV130600Block {
    pub u1: i8,
    #[rw(jp = false)]
    pub u2: Option<i16>,
    pub _130600: Assertable<130600>,
}
