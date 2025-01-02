use serde::Serialize;

/// Struct to represent the current progress data for the client
#[derive(Serialize)]
pub struct SendProgressData {
    pub msgs_sent: u64,
    pub msgs_failed: u64,
    pub avg_wait_time: u64
}