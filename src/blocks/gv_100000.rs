use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::stream::{HashMapLength, LengthString};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100000)]
pub struct GV100000Block {
    pub legend_tickets: i32,
    #[rw(with = "HashMapLength<i8, i8, i32>")]
    pub u1: HashMap<i8, i32>, // FIXME: may not be a hashmap
    pub u2: bool,
    pub u3: bool,
    #[rw(with = "LengthString<i32>")]
    pub password_refresh_token: String,
    pub u4: bool,
    pub u5: i8,
    pub u6: i8,
    pub u7: f64,
    pub u8: f64,
}
