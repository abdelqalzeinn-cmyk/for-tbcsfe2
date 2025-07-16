use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130100Block {
    pub unknown: HashMapLength<i32, i32, i64>, // FIXME: may not be a hashmap
    pub _130100: Assertable<130100>,
}
