use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 63)]
pub struct GV63Block {
    #[rw(with = "HashMapLength<i32, i32, bool>")]
    pub unlock_popups: HashMap<i32, bool>,
}
