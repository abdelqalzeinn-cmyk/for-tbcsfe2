use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV90900Block {
    pub cat_shrine: CatShrine,
    pub u1: f64,
    pub u2: f64,
    pub _90900: Assertable<90900>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatShrine {
    pub unknown: bool,
    pub stamp_1: f64,
    pub stamp_2: f64,
    pub shrine_gone: bool,
    pub flags: LengthVec<i8, i8>,
    pub xp_offering: i64,
}
