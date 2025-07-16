use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_76::UncannyChapters,
    stream::{Assertable, LengthVec},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV77Block {
    pub uncanny_chapters2: UncannyChapters,
    pub lucky_tickets: LengthVec<i32, i32>,
    pub unkown: bool,
    pub _77: Assertable<77>,
}
