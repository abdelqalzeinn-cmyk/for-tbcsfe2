use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Copy, Clone, Default, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV51Block {
    _51: Assertable<51>,
}
