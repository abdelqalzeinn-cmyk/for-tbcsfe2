use bcsfe_derive::{Readable, Writable};

use crate::blocks::gv_90300::GauntletChapters;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 110500)]
pub struct GV110500Block {
    pub behemoth_culling: GauntletChapters,
    pub unknown: bool,
}
