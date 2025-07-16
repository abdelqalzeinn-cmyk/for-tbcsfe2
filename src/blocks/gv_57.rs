use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV57Block {
    pub unknown: bool,
    pub favourite_cats: HashMapLength<i32, i32, bool>,
    pub _57: Assertable<57>,
}
