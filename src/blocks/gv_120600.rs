use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV120600Block {
    pub sfx_volume: i8,
    pub bgm_volume: i8,
    pub _120600: Assertable<120600>,
}
