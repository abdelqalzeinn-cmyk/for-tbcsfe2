use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 90900)]
pub struct GV90900Block {
    pub cat_shrine: CatShrine,
    pub u1: f64,
    pub u2: f64,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatShrine {
    pub unknown: bool,
    pub stamp_1: f64,
    pub stamp_2: f64,
    pub shrine_gone: bool,
    #[rw(with = "LengthVec<i8,i8>")]
    pub flags: Vec<i8>,
    pub xp_offering: i64,
}
