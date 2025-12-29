use bcsfe_derive::{Readable, Writable};

use crate::save::Formi16;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 80200)]
pub struct GV80200Block {
    pub unknown: bool,
    pub leadership: i16,
    pub officer_cat_id: i16,
    pub officer_cat_form: Formi16,
}
