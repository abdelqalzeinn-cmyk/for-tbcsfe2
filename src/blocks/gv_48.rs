use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Copy, Clone, Default, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV48Block {
    pub _48: Assertable<48>,
}
