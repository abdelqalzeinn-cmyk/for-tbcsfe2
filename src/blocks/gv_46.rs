use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV46Block {
    pub _46: Assertable<46>,
}
