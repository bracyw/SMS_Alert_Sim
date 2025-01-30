use std::sync::Arc;
use crate::{models::alert_models::Alert, structures::{producer::SmsProducer, sender::SimSender, sender_service::SenderService}, utils::sms_utils};
use crate::utils::sms_utils::error_utils::SenderValidationError;


/// Convenience function to create a sender service with the given number of senders, failure percent, average send time, and standard deviation.
/// 
/// Parameters:
/// * `num_senders` - The number of senders to create (no limit)
/// * `failure_percent` - The percentage of messages that will fail to send (0-100)
/// * `avg_send_time` - The average time it takes to send a message in seconds
/// 
/// Returns: Either a `SenderService` with senders that have the given attributes above or a `SenderValidationError` if the inputs are invalid
pub fn create_sim_senders(num_senders: f64, failure_percent: f64, avg_send_time: f32) -> Result<Vec<SimSender>, SenderValidationError> {
    sms_utils::sender_utils::create_sim_senders(num_senders as u64, failure_percent as u64, avg_send_time)
}


/// Sends the given amount of messages with the given message (or random strings if Null) and sender service.
/// 
/// Parameters:
/// * `msg` - The message to send (if this is None then the Alert will send random messages)
/// * `num_to_send` - The number of messages to send
/// * `sender_service` - The sender service to use to send the messages
/// 
/// Returns: an `Alert` struct with the results of the sending operation
pub async fn send_msgs(msg: Option<String>, num_to_send: u64, sender_service: Arc<SenderService>) -> Alert {
    // create a producer with the given sender service and send the messages
    let producer = SmsProducer::new(sender_service);
    producer.send_msgs(msg, num_to_send).await
}

