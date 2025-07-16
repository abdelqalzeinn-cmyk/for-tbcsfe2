use bcsfe_derive::{Readable, Writable};

use crate::{blocks::gv_120000::NewChapters, stream::Assertable};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130500Block {
    pub unknown_chapters: NewChapters,
    pub _130500: Assertable<130500>,
}
