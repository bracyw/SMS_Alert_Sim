use std::sync::atomic::AtomicU64;

/// Measures the progress (health and performance) of the sms simulator backend. 
/// This monitor is safe to use in a multithreaded environment, through the use of only atomic operations. 
/// Any additions to the monitor should be done in a thread-safe manner and should remain simplistic. 
/// More complex measuremnts should be done through other means, as this monitor is meant to be light weight. 
pub struct ProgressMonitor {
    pub num_messages_sent: AtomicU64,
    pub num_messages_failed: AtomicU64,
    pub total_wait_time: AtomicU64,
}

/// Implementation for the ProgressMonitor struct
impl ProgressMonitor {

    /// Creates a new instance of the ProgressMonitor struct with all values set to 0
    pub fn new() -> Self {
        Self {
            num_messages_sent: AtomicU64::new(0),
            num_messages_failed: AtomicU64::new(0),
            total_wait_time: AtomicU64::new(0),
        }
    }

    /// Increments the sent counter by one and adds the wait time to the total wait time
    /// 
    /// Parmeters:
    /// * `wait_time_secs` - The time it took to send the message in seconds
    pub fn add_message_sent(&self, wait_time_secs: u64) {
        self.num_messages_sent.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.total_wait_time.fetch_add(wait_time_secs, std::sync::atomic::Ordering::SeqCst);
    }

    /// Increments the failed counter by one and adds the wait time to the total wait time
    /// 
    /// Parmeters:
    /// * `wait_time_secs` - The time it took to for the send to fail in seconds
    pub fn add_message_failed(&self, wait_time_secs: u64) {
        self.num_messages_failed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.total_wait_time.fetch_add(wait_time_secs, std::sync::atomic::Ordering::SeqCst);
    }

    /// Returns the number of messages sent
    pub fn get_msgs_sent(&self) -> u64 {
        self.num_messages_sent.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns the number of messages failed
    pub fn get_msgs_failed(&self) -> u64 {
        self.num_messages_failed.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns the average wait time for all messages sent
    pub fn get_avg_wait_time(&self) -> u64 {
        let msgs_sent = self.num_messages_sent.load(std::sync::atomic::Ordering::SeqCst);
        let msgs_failed = self.num_messages_failed.load(std::sync::atomic::Ordering::SeqCst);
        let total_wait_time = self.total_wait_time.load(std::sync::atomic::Ordering::SeqCst);
        let total_msgs = msgs_failed + msgs_sent;
        if total_msgs == 0 {
            return 0;
        }
        total_wait_time / total_msgs
    }

    /// Resets all values in the progress monitor to 0.
    /// 
    /// Again: IT RESETS ALL VALUES IN THE PROGRESS MONITOR TO 0.
    pub fn reset(&self) {
        self.num_messages_sent.store(0, std::sync::atomic::Ordering::SeqCst);
        self.num_messages_failed.store(0, std::sync::atomic::Ordering::SeqCst);
        self.total_wait_time.store(0, std::sync::atomic::Ordering::SeqCst);
    }

}