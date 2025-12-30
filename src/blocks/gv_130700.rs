use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 130700)]
pub struct GV130700Block {
    #[rw(en = false, kr = false, tw = false)]
    pub u1: i16,
    pub u2: f64,
    pub u3: i8,
    pub u4: i8,
    pub u5: i16,
    pub u6: i8,
    pub u7: i8,
    pub u8: i8,
    pub u9: f64,
    #[rw(with = "HashMapLength<i16, i16, GV130700UnknownInner>")]
    pub u10: HashMap<i16, GV130700UnknownInner>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV130700UnknownInner {
    pub u1: i16,
    pub u2: i32,
    #[rw(with = "HashMapLength<i16, i16, i16>")]
    pub u3: HashMap<i16, i16>,
}
