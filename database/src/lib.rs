use std::time::{SystemTime, UNIX_EPOCH};

pub mod config;
pub mod requests;
pub mod types;

pub fn now_timestamp() -> u32 {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap();

    // Fix this by 2038
    timestamp.as_secs() as u32
}
