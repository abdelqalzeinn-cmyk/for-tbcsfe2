use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Copy, Clone, Default, Readable, Writable)]
pub struct GV51Block {
    _51: Assertable<51>,
}
