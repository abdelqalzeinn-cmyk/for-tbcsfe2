use bcsfe_derive::{Readable, Writable};

use crate::{blocks::gv_90300::GauntletChapters, stream::Assertable};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV110500Block {
    pub behemoth_culling: GauntletChapters,
    pub unknown: bool,
    pub _110500: Assertable<110500>,
}
