use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 130100)]
pub struct GV130100Block {
    #[rw(with = "HashMapLength<i32, i32, i64>")]
    pub unknown: HashMap<i32, i64>, // FIXME: may not be a hashmap
}
