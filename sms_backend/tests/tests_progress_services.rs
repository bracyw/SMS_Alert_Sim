use std::sync::Arc;

use sms_backend::{models::progress_models::SendProgressData, services::progress_services::get_progress_data, structures::progress_monitor::ProgressMonitor};

#[tokio::test]
async fn test_get_progress_data() {
    // Mock a ProgressMonitor object (assuming its methods work as described)
    let pm = Arc::new(ProgressMonitor::new()); // Assume this creates a default instance

    // Simulate progress data
    pm.add_message_sent(2); // Assume this increments the messages sent count
    pm.add_message_failed(4); // Assume this increments the messages failed count

    // Call the function under test
    let progress_data: SendProgressData = get_progress_data(pm.clone());

    // Assert the progress data is as expected
    assert_eq!(progress_data.msgs_sent, 1, "Messages sent count is incorrect");
    assert_eq!(progress_data.msgs_failed, 1, "Messages failed count is incorrect");
    assert_eq!(progress_data.avg_wait_time, 3, "Average wait time is incorrect");
}