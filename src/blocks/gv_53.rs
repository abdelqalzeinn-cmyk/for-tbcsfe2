use bcsfe_derive::{Readable, Writable};

use crate::stream::{Assertable, LengthVec};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV53Block {
    pub time_since_last_check_cumulative: f64,
    pub server_timestamp: f64,
    pub last_checked_energy_recovery_time: f64,
    pub time_since_last_check: f64,
    pub last_checked_expedition_time: f64,
    pub catfruit: LengthVec<i32, i32>,
    pub cat_fourth_forms: LengthVec<i32, i32>,
    pub cat_catseyes_used: LengthVec<i32, i32>,
    pub catseyes: LengthVec<i32, i32>,
    pub catamins: LengthVec<i32, i32>,
    pub gamatoto: Gamatoto,
    pub unlock_popups: LengthVec<i32, bool>,
    pub ex_stages: LengthVec<i32, [i32; 12]>,
    pub _53: Assertable<53>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Gamatoto {
    pub remaining_seconds: f64,
    pub return_flag: bool,
    pub xp: i32,
    pub dest_id: i32,
    pub recon_length: i32,
    pub unknown: i32,
    pub notif_value: i32,
}
