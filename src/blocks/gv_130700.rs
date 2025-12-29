use bcsfe_derive::{Readable, Writable};

use crate::stream::HashMapLength;

type InnerValue = (i16, i32, HashMapLength<i16, i16, i16>);

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 130700)]
pub struct GV130700Block {
    #[rw(en = false, kr = false, tw = false)]
    pub u1: Option<i16>,
    pub u2: f64,
    pub u3: i8,
    pub u4: i8,
    pub u5: i16,
    pub u6: i8,
    pub u7: i8,
    pub u8: i8,
    pub u9: f64,
    pub u10: HashMapLength<i16, i16, InnerValue>,
}
