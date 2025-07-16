use bcsfe_derive::{Readable, Writable};

use crate::{save::Formi16, stream::Assertable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV80200Block {
    pub unknown: bool,
    pub leadership: i16,
    pub officer_cat_id: i16,
    pub officer_cat_form: Formi16,
    pub _80200: Assertable<80200>,
}
