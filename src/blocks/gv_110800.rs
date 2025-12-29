use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV110800Block {
    pub cat_shrine_dialogs: i32,
    pub u1: bool,
    pub dojo_3x_speed: bool,
    pub u2: bool,
    pub u3: bool,
    pub _110800: Assertable<110800>,
}
