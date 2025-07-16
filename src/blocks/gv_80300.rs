use bcsfe_derive::{Readable, Writable};

use crate::stream::Assertable;

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV80300Block {
    pub filibuster_stage_id: i8,
    pub filibuster_stage_enabled: bool,
    pub _80300: Assertable<80300>,
}
