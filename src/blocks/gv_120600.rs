use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 120600)]
pub struct GV120600Block {
    pub sfx_volume: i8,
    pub bgm_volume: i8,
}
