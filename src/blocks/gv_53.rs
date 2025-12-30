use bcsfe_derive::{Readable, Writable};

use crate::stream::LengthVec;

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 53)]
pub struct GV53Block {
    pub time_since_last_check_cumulative: f64,
    pub server_timestamp: f64,
    pub last_checked_energy_recovery_time: f64,
    pub time_since_last_check: f64,
    pub last_checked_expedition_time: f64,
    #[rw(with = "LengthVec<i32,i32>")]
    pub catfruit: Vec<i32>,
    #[rw(with = "LengthVec<i32,i32>")]
    pub cat_fourth_forms: Vec<i32>,
    #[rw(with = "LengthVec<i32,i32>")]
    pub cat_catseyes_used: Vec<i32>,
    #[rw(with = "LengthVec<i32,i32>")]
    pub catseyes: Vec<i32>,
    #[rw(with = "LengthVec<i32,i32>")]
    pub catamins: Vec<i32>,
    pub gamatoto: Gamatoto,
    #[rw(with = "LengthVec<i32,bool>")]
    pub unlock_popups: Vec<bool>,
    #[rw(with = "LengthVec<i32,[i32;12]>")]
    pub ex_stages: Vec<[i32; 12]>,
}

impl GV53Block {
    /// returns the previous value if the index is valid
    pub fn set_catseye(&mut self, index: usize, val: i32) -> Option<i32> {
        if let Some(prev) = self.catseyes.get_mut(index) {
            let tmp = *prev;

            *prev = val;

            Some(tmp)
        } else {
            None
        }
    }

    pub fn get_catseye(&self, index: usize) -> Option<i32> {
        self.catseyes.get(index).map(|v| *v)
    }
    /// returns the previous value if the index is valid
    pub fn set_catfruit(&mut self, index: usize, val: i32) -> Option<i32> {
        if let Some(prev) = self.catfruit.get_mut(index) {
            let tmp = *prev;

            *prev = val;

            Some(tmp)
        } else {
            None
        }
    }

    pub fn get_catfruit(&self, index: usize) -> Option<i32> {
        self.catfruit.get(index).map(|v| *v)
    }
    /// returns the previous value if the index is valid
    pub fn set_catamin(&mut self, index: usize, val: i32) -> Option<i32> {
        if let Some(prev) = self.catamins.get_mut(index) {
            let tmp = *prev;

            *prev = val;

            Some(tmp)
        } else {
            None
        }
    }

    pub fn get_catamin(&self, index: usize) -> Option<i32> {
        self.catamins.get(index).map(|v| *v)
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gamatoto {
    pub remaining_seconds: f64,
    pub return_flag: bool,
    pub xp: i32,
    pub dest_id: i32,
    pub recon_length: i32,
    pub unknown: i32,
    pub notif_value: i32,
}
