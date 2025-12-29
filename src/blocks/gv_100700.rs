use bcsfe_derive::{Readable, Writable};

use crate::blocks::gv_90000::GamblingEvent;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100700)]
pub struct GV100700Block {
    #[rw(gvcc)]
    pub cat_scratcher: GamblingEvent,
}
