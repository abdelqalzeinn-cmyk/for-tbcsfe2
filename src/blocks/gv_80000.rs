use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, HashMapLength, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV80000Block {
    #[rw(gvcc)]
    pub gold_pass: GoldPass,
    pub cat_talents: HashMapLength<i32, i32, LengthVec<i32, Talent>>,
    pub np: i32,
    pub unknown: bool,
    pub _80000: Assertable<80000>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GoldPass {
    pub officer_id: i32,
    pub total_renewal_times: i32,
    pub start_date_now: f64,
    pub end_date_now: f64,
    pub start_date_next: f64,
    pub end_date_next: f64,
    pub start_date_total: f64,
    pub end_date_total: f64,
    pub time_error_end: f64,
    pub total_state_updates: i32,
    pub login_bonus_date: f64,
    pub claimed_rewards: HashMapLength<i32, i32, i32>,
    pub remaining_days_popup: f64,
    pub first_popup_flag: bool,
    #[rw(min_gv = 80100)]
    pub badge_flag: Option<bool>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Talent {
    pub id: i32,
    pub level: i32,
}
