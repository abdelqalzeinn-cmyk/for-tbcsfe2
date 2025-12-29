use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_70000::UncannyChapters,
    stream::{Assertable, LengthVec},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV70100Block {
    pub catamin_stages: UncannyChapters,
    pub lucky_tickets: LengthVec<i32, i32>,
    pub unknown: bool,
    pub _77: Assertable<77>,
}
