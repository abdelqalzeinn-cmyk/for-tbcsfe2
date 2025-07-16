use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Copy, Clone, Default, Readable, Writable)]
pub struct GV48Block {
    pub _48: Assertable<48>,
}
