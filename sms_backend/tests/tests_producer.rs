use std::{sync::Arc, time::Instant};

use futures::{stream::FuturesUnordered, StreamExt};
use sms_backend::{structures::{producer::SmsProducer, progress_monitor::ProgressMonitor}, utils::sms_utils::sender_utils::create_sender_service};



// ensure the producer caries out a basic send as intented
#[tokio::test]
async fn test_producer_basic_send() {
    let pm = Arc::new(ProgressMonitor::new());
    let sender_service = create_sender_service(1, 100, 1.0, 0.0, pm).await.unwrap();
    
    let producer = SmsProducer::new(Arc::new(sender_service));
    let alert = producer.send_msgs(None, 1).await;

    assert_eq!(alert.messages_sent, 0);
    assert_eq!(alert.messages_failed, 1);
    // normally this would round down but because the entire send is encapsulated in a single future it will round up
    assert_eq!(alert.total_time_to_send, 1); 
    
    // ensure the total time to send is correct on sends where we have to wait for senders to be available
    let alert = producer.send_msgs(Some("HELLO WORLD".to_owned()), 3).await;
    assert_eq!(alert.messages_sent, 0);
    assert_eq!(alert.messages_failed, 3);
    assert_eq!(alert.total_time_to_send, 3, "total time to send is off");
}


// test producer effecieny with concurrent sends
#[tokio::test]
async fn multi_producer_concurrency_test() {
    let pm = Arc::new(ProgressMonitor::new());
    let sender_service = create_sender_service(80, 100, 1.0, 0.0, pm).await.unwrap();
    
    let producer1 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer2 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer3 = SmsProducer::new(Arc::new(sender_service.clone()));
    let mut task_handle = Vec::new();
    let start_time = Instant::now();
    task_handle.push(tokio::spawn(async move {
        producer1.send_msgs(None, 30).await;
    }));
    task_handle.push(tokio::spawn(async move {
        producer2.send_msgs(None, 20).await;
    }));
    task_handle.push(tokio::spawn(async move {
        producer3.send_msgs(None, 30).await;
    }));

    futures::future::join_all(task_handle).await;

    // there exactly enough senders to send all the messages
    // unless someones pc is really slow, this should always take less than 2 seconds
    assert!(start_time.elapsed().as_secs() < 2);
}


// test multiple producers on the same sender service, produce correct results
#[tokio::test]
async fn multi_producer_same_service() {
    let pm = Arc::new(ProgressMonitor::new());
    let sender_service = create_sender_service(100, 100, 1.0, 0.0, pm).await.unwrap();
    
    let producer1 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer2 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer3 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer4 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer5 = SmsProducer::new(Arc::new(sender_service.clone()));
    let producer6 = SmsProducer::new(Arc::new(sender_service.clone()));
    let mut futures_unordered = FuturesUnordered::new();
    futures_unordered.push(tokio::spawn(async move {
        producer1.send_msgs(None, 10).await
    }));
    futures_unordered.push(tokio::spawn(async move {
        producer2.send_msgs(None, 20).await
    }));
    futures_unordered.push(tokio::spawn(async move {
        producer3.send_msgs(None, 30).await
    }));
    futures_unordered.push(tokio::spawn(async move {
        producer4.send_msgs(None, 10).await
    }));
    futures_unordered.push(tokio::spawn(async move {
        producer5.send_msgs(None, 20).await
    }));
    futures_unordered.push(tokio::spawn(async move {
        producer6.send_msgs(None, 30).await
    }));

    while let Some(result) = futures_unordered.next().await {
        let alert = result.unwrap();
        assert_eq!(alert.messages_sent + alert.messages_failed, alert.send_amount_requested);
    }
}
