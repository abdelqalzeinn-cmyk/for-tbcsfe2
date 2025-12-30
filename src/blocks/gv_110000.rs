use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 110000)]
pub struct GV110000Block {
    #[rw(with = "HashMapLength<i16, i32, (i8, i8)>")]
    pub u1: HashMap<i32, (i8, i8)>,
}
