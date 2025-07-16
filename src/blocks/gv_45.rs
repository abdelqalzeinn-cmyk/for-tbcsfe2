use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable)]
pub struct GV45Block {
    pub itf1_complete: i32,
    pub itf_timed_scores: [[i32; 51]; 3],
    pub title_chapter_bg: i32,
    #[rw(min_gv = 27)]
    pub combo_unlocks: Option<LengthVec<i32, i32>>,
    pub combo_unlocked_10k_ur: bool,
    pub _45: Assertable<45>,
}

impl Default for GV45Block {
    fn default() -> Self {
        Self {
            itf_timed_scores: [[0; 51]; 3],
            itf1_complete: 0,
            title_chapter_bg: 0,
            combo_unlocks: None,
            combo_unlocked_10k_ur: false,
            _45: Assertable,
        }
    }
}
