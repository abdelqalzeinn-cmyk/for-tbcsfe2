use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 80300)]
pub struct GV80300Block {
    pub filibuster_stage_id: i8,
    pub filibuster_stage_enabled: bool,
}
