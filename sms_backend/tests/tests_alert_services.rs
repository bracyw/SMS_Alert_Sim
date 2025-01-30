use std::{sync::Arc, time::Instant};
use rand::Rng;
use sms_backend::{services::alert_services::{create_sim_senders, send_msgs}, structures::progress_monitor::ProgressMonitor, utils::sms_utils::sender_utils::create_sender_service};



#[tokio::test]
async fn test_create_sim_senders_num() {
    let senders = create_sim_senders(100.0, 100.0, 1.0);
    assert_eq!(100, senders.unwrap().len());
}

#[tokio::test]
async fn test_create_sim_senders_failure_rate() {
    let senders = create_sim_senders(100.0, 0.0, 1.0);

    let mut rand = rand::thread_rng();
    let rand_index = rand.gen_range(0..100);
    assert!(senders.unwrap().get(rand_index).unwrap().send("hello there".to_string()).await, "sender service should have created senders that only fail");
}


#[tokio::test]
async fn test_send_msgs_expected_alert() {
    let pm = Arc::new(ProgressMonitor::new());
    let sender_service = create_sender_service(10, 0, 1.0, pm).await.unwrap();
    let result = send_msgs(None, 10, Arc::new(sender_service)).await;
    assert_eq!(result.messages_sent, 10);
    assert_eq!(result.messages_failed, 0);
    assert_eq!(result.send_amount_requested, 10);
}

   