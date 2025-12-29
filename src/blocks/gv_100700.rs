use bcsfe_derive::{Readable, Writable};

use crate::{blocks::gv_90000::GamblingEvent, stream::Assertable};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV100700Block {
    #[rw(gvcc)]
    pub cat_scratcher: GamblingEvent,
    pub _100700: Assertable<100700>,
}
