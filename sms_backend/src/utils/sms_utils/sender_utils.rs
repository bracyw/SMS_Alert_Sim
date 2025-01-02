use std::sync::Arc;

use crate::structures::{sender::SimSender, sender_service::SenderService, progress_monitor::ProgressMonitor};

use super::error_utils::SenderValidationError;


/// Convenience function to create a sender service with the given number of senders, failure percent, average send time, and standard deviation.
/// 
/// Parameters:
/// * `num_senders` - The number of senders to create (no limit)
/// * `failure_percent` - The percentage of messages that will fail to send (0-100)
/// * `avg_send_time` - The average time it takes to send a message in seconds
/// * `std_dev` - The standard deviation of the send time in seconds
/// 
/// Returns: Either a `SenderService` with senders that have the given attributes above or a `SenderValidationError` if the inputs are invalid
pub async fn create_sender_service(num_senders: u64, failure_percent: u64, avg_send_time: f32, std_dev: f32, pm: Arc<ProgressMonitor>) -> Result<SenderService, SenderValidationError> {
    // create the senders, if there is creating the senders return that
    let senders = create_sim_senders(num_senders, failure_percent, avg_send_time, std_dev)?;

    // otherwise create the sender service
    Ok(SenderService::new(senders, pm).await)
}


/// Convenience function to create a sender service with the given number of senders, failure percent, average send time, and standard deviation.
/// 
/// Parameters:
/// * `num_senders` - The number of senders to create (no limit)
/// * `failure_percent` - The percentage of messages that will fail to send (0-100)
/// * `avg_send_time` - The average time it takes to send a message in seconds
/// * `std_dev` - The standard deviation of the send time in seconds
/// 
/// Returns: Either a `SenderService` with senders that have the given attributes above or a `SenderValidationError` if the inputs are invalid
pub fn create_sim_senders(num_senders: u64, failure_percent: u64, avg_send_time: f32, std_dev: f32) -> Result<Vec<SimSender>, SenderValidationError> {
    // convert whole value percent to decminal (for ease of use through api / in general)
    let decimal_percent: f64 = failure_percent as f64 / 100 as f64;

    // ensure all inputs are correct
    if let Err(check_input) = SenderValidationError::validate(decimal_percent, avg_send_time, std_dev) {
        Err(check_input)
    } else {
        // make each sender with the given specifications
        let mut senders = Vec::new();
        
        for _ in 1..=num_senders as u64 {
            senders.push(SimSender::new(avg_send_time, std_dev, decimal_percent as f64));
        }

        Ok(senders)
    }
}

