use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Default, Clone, Copy, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 140700)]
pub struct GV140700Block {
    pub b1: bool,
    pub b2: bool,
    pub d1: f64,
    pub d2: f64,
    pub b3: bool,
}
