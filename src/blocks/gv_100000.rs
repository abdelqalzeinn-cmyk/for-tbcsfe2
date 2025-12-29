use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthString};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV100000Block {
    pub legend_tickets: i32,
    pub u1: HashMapLength<i8, i8, i32>, // FIXME: may not be a hashmap
    pub u2: bool,
    pub u3: bool,
    pub password_refresh_token: LengthString<i32>,
    pub u4: bool,
    pub u5: i8,
    pub u6: i8,
    pub u7: f64,
    pub u8: f64,
    pub _100000: Assertable<100000>,
}
