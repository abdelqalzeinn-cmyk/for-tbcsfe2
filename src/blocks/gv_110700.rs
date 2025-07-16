use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV110700Block {
    pub u1: HashMapLength<i32, i32, (f64, f64)>,
    #[rw(jp = false)]
    pub u2: Option<bool>,
    pub _110700: Assertable<110700>,
}
