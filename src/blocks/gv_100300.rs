use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100300)]
pub struct GV100300Block {
    pub endless_items: [EndlessBattleItem; 6],
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndlessBattleItem {
    pub active: bool,
    pub unknown: bool,
    pub amount: i8,
    pub start: f64,
    pub end: f64,
}
