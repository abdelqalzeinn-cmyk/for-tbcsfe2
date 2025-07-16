use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
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
    pub u10: HashMapLength<i16, i16, (i16, i32, HashMapLength<i16, i16, i16>)>,
    pub _130700: Assertable<130700>,
}
