use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV100300Block {
    pub endless_items: [EndlessBattleItem; 6],
    pub _100300: Assertable<100300>,
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
