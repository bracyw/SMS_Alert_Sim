use std::{sync::{atomic::AtomicBool, Arc}, time::Duration};
use uuid::Uuid;
use rand::Rng;

use crate::utils::sms_utils::rand_utils::create_random_number;

/// A simulated sms message sender with a configurable average wait time to send a message, and sending failure percentage. 
/// 
/// Only one message can be sent at a time to simulate a limited supply of a sending resource.
/// This also makes future implementations of a legitmate sender become less complicated as the 
/// sender will only have to resolve callback response at a time from a perspective reciever. 
/// 
/// To send multiple messages conncurrently then you can simply use multiple `SimSender`
#[derive(Clone)]
pub struct SimSender {
    id: String, // id's are unique 
    send_time_mean: f32, // zero can be used if implementing actual sending
    send_time_std: f32,
    failure_percentage: f64, // percentage as a decimal
    currently_sending: Arc<AtomicBool>
}


impl SimSender {

    /// Creates a new `SimSender` with the given send time mean and failure percent rate.
    /// 
    /// Parameters:
    /// 
    /// * `send_time_mean` - the average time in SECONDS it will take to simulate a message, with a standard deviation of 5 seconds.
    /// * `failure_percent` - the percentage chance as a DECIMAL, that a message will fail to send.
    pub fn new(send_time_mean: f32, send_time_std: f32, failure_percentage: f64) -> SimSender {
        assert!(send_time_mean >= 0 as f32);
        assert!(failure_percentage >= 0 as f64, "percentage for a `Sender` was negative");
        assert!(failure_percentage <= 100 as f64, "percentage for a `Sender` was above 100%, must be represented as a decimal btwn 0 and 1.0");

        SimSender {
            id: Uuid::new_v4().to_string(),
            send_time_mean,
            send_time_std,
            failure_percentage,
            currently_sending: Arc::new(AtomicBool::new(false))
        }
    }

    /// Simulates sending a `msg` by waiting random amount of time centered around the `send_time_mean`.
    /// The bool returned is simulated based on the `failure_percent` of this `SimSender`
    /// 
    /// IMPORTANT: you cannot call send on the same sender in multiple tasks
    /// 
    /// Parameters:
    /// 
    /// * `msg` - the msg to send
    /// 
    /// Returns a bool, true on a successful send, false if the msg fails to send
    pub async fn send(&self, _msg: String) -> bool {
        let mut state = self.currently_sending.load(std::sync::atomic::Ordering::Relaxed);
        // simulate a limited sending resource
        while state {
            // do nothing just wait
            tokio::time::sleep(Duration::from_micros(10)).await;
            state = self.currently_sending.load(std::sync::atomic::Ordering::Relaxed);
        }

        self.currently_sending.fetch_or(true, std::sync::atomic::Ordering::Relaxed);

        // set up and calc wait time
        let wait_time = create_random_number(self.send_time_mean, self.send_time_std);

        // sleep the given amount of time
        tokio::time::sleep(Duration::from_secs_f32(wait_time)).await;

        self.currently_sending.fetch_and(false, std::sync::atomic::Ordering::Relaxed);
        
        // sim the chance of failure for this send 
        let mut rng = rand::thread_rng();
        // gen bool takes a percent chance of a true boolean... but we want percent of failure: false
        !rng.gen_bool(self.failure_percentage)
    }

    /// Gets the id of this `SimSender`
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}