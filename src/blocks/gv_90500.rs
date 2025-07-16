use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_90300::GauntletChapters,
    stream::{Assertable, HashMapLength},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90500Block {
    pub collab_gauntlets: GauntletChapters,
    pub u1: bool,
    pub timestamp1: f64,
    pub timestamp2: f64,
    pub u2: i32,
    #[rw(min_gv = 100300)]
    pub u3: Option<Unknown90500_100300>,
    #[rw(min_gv = 130700)]
    pub u4: Option<Unknown90500_130700>,
    #[rw(min_gv = 140100)]
    pub u5: Option<HashMapLength<i16, i32, f64>>,
    pub _90500: Assertable<90500>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Unknown90500_130700 {
    pub u1: HashMapLength<i16, i32, i8>,
    pub u2: HashMapLength<i16, i32, f64>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Unknown90500_100300 {
    pub u1: i8,
    pub u2: bool,
    pub timestamp1: f64,
    pub timestamp2: f64,
}
