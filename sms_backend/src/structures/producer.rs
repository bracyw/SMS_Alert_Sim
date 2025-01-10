use std::{sync::Arc, time::Instant};
use futures::{stream::FuturesUnordered, StreamExt};
use tracing::debug;
use crate::structures::sender_service::SenderService;
use crate::models::alert_models::Alert;
use crate::utils::sms_utils::rand_utils::create_random_string;

/// A producer that sends messages using the given sender service
pub struct SmsProducer {
    sender_service: Arc<SenderService>
}


impl SmsProducer {
    /// Creates a new SmsProducer that will use the given shared sender service to send messages
    /// 
    /// Parameters:
    /// * `sender_service` - The sender service to use to send messages
    /// 
    /// Returns: A new SmsProducer
    pub fn new(sender_service: Arc<SenderService>) -> SmsProducer {
        SmsProducer {
            sender_service
        }

    }

    /// Sends the given amount of messages with the given message (or random strings if Null) using the sender service in this producer.
    /// 
    /// Parameters:
    /// * `msg` - The message to send (if this is None then the Alert will send random messages)
    /// * `num_to_send` - The number of messages to send
    /// 
    /// Returns: an `Alert` struct with the results of the sending operation
    pub async fn send_msgs(&self, msg: Option<String>, num_to_send: u64) -> Alert {
        let mut futures_unordered = FuturesUnordered::new();
        let mut messages_sent = 0;
        let mut messages_failed = 0;
    
        // Start timing the operation
        let start_time = Instant::now();
    
        for _ in 1..=num_to_send {
            let sender_service = self.sender_service.clone();
    
            // Generate a random message if none is provided
            let msg = msg.clone().unwrap_or_else(|| {
                create_random_string(100)
            });
    
            // create a future to send the given message and track it
            futures_unordered.push(async move {
                sender_service.send(msg).await
            });
        }
    
        // Process all futures
        while let Some(result) = futures_unordered.next().await {
            if result {
                messages_sent += 1;
            } else {
                messages_failed += 1;
            }
        }
    
        // Calculate the total time taken
        let total_time_to_send = start_time.elapsed().as_secs();

        debug!("Sending {} messages", num_to_send);
    
        // Return an Alert struct with the results of this send
        Alert {
            message: msg,
            send_amount_requested: num_to_send as i32,
            messages_sent: messages_sent as i32,
            messages_failed: messages_failed as i32,
            total_time_to_send: total_time_to_send as i32,
        }
    }
}