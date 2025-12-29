use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV130400Block {
    pub u1: f64,
    pub u2: f64,
    pub _130400: Assertable<130400>,
}
