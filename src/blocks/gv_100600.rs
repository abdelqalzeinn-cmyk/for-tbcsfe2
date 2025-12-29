use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100600)]
pub struct GV100600BlockEn {
    pub uknown: i8,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100600)]
pub struct GV100600Block {
    pub timestamp: f64,
    pub platinum_shards: i32,
    pub u2: bool,
}
