use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_90100::UnknownDict90100,
    stream::{Assertable, HashMapLength},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV100700Block {
    pub u1: HashMapLength<i16, i16, bool>,
    pub u2: HashMapLength<i16, i16, HashMapLength<i16, i16, i16>>,
    #[rw(gvcc)]
    pub u3: UnknownDict90100,
    pub _100700: Assertable<100700>,
}
