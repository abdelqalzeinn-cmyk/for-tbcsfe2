use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV81000Block {
    pub restart_pack: i8,
    pub _81000: Assertable<81000>,
}
