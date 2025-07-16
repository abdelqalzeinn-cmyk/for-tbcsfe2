use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_90100::UnknownDict90100,
    stream::{Assertable, HashMapLength, LengthVec},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90000Block {
    pub medals: Medals,
    pub unkown: HashMapLength<i16, i16, bool>,
    pub unknown_2: HashMapLength<i16, i16, HashMapLength<i16, i16, i16>>,
    #[rw(gvcc)]
    pub unknown_3: UnknownDict90100,
    _90000: Assertable<90000>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Medals {
    pub u1: i32,
    pub u2: i32,
    pub u3: i32,
    pub data_1: LengthVec<i16, i16>,
    pub data_2: HashMapLength<i16, i16, i8>,
    pub u4: bool,
}
