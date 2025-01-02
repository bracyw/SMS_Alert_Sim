use std::time::Instant;

use futures::future::join_all;
use rand::Rng;
use sms_backend::structures::sender::SimSender;

#[tokio::test]
async fn test_single_sender_send() {
    // notice failure is now 0 ... 1 not 1 ... 100
    let sender = SimSender::new(4.0, 0.0, 1.0);

    let start_time = Instant::now();
    let result = sender.send("Hello world".to_owned()).await;
    assert!(3999 < start_time.elapsed().as_millis() && start_time.elapsed().as_millis() < 4100);
    assert!(!result, "The result of this send has a 100% chance of being false");
}


#[tokio::test]
async fn test_no_multi_send() {
    let sender = SimSender::new(1.0, 0.0, 1.0);
    let mut task_handle = Vec::new();

    let sender_clone = sender.clone();
    let start_time = Instant::now();
    task_handle.push(tokio::spawn(async move {
        sender_clone.send("Hello world".to_owned()).await;
    }));
    task_handle.push(tokio::spawn(async move {
        sender.send("Hello world".to_owned()).await;
    }));

    join_all(task_handle).await;

    assert!(1999 < start_time.elapsed().as_millis() && start_time.elapsed().as_millis() < 2100, "two async sends of 1 sec should take roughly 2 seconds");
}


#[tokio::test]
async fn test_sender_failure_rate() {
    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let expected_fail_rate = rng.gen_range(0.0..1.0);
        let mut all_failures = 0;
        for _ in 0..100 {
            let sender = SimSender::new(0.0, 0.0, expected_fail_rate);
            if !sender.send("bruh".to_owned()).await {
                all_failures += 1;
            }
        }
        let actual_failure_rate = all_failures as f64 / 100 as f64;
        assert!((expected_fail_rate - 0.1) < actual_failure_rate && actual_failure_rate < (expected_fail_rate + 0.1));
    }
}