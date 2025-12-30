use std::time::{SystemTime, UNIX_EPOCH};

pub mod account_info;
pub mod password;
pub mod signature;
pub mod stored_string;
pub mod transfer;

pub fn unix_timestamp() -> u64 {
    let start = SystemTime::now();
    let duration = start
        .duration_since(UNIX_EPOCH)
        .expect("time went forwards");

    duration.as_secs()
}
