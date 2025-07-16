use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV63Block {
    pub unlock_popups: HashMapLength<i32, i32, bool>,
    pub _63: Assertable<63>,
}
