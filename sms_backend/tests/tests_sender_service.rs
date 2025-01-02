use std::sync::Arc;
use sms_backend::{services::alert_services::create_sim_senders, structures::{progress_monitor::ProgressMonitor, sender_service::SenderService}, utils::sms_utils::sender_utils::create_sender_service};



// test multiple send message request that go over buffer SenderService channel (hard coded to 1000)
#[tokio::test]
async fn test_full_send_channel_multi_task() {
    let pm = Arc::new(ProgressMonitor::new());
    let sender_service = create_sender_service(10, 50, 0.01, 0.0, pm.clone()).await.unwrap();

    let mut handles = Vec::new();
    for _ in 0..1000 {
        let sender_service_clone = sender_service.clone();
        handles.push(tokio::spawn(async move {
            sender_service_clone.send("abacabdaba".to_owned()).await;
        }));
    }

    futures::future::join_all(handles).await;

    // check that the failure percentage is roughly within 1% of the expected value
    assert!(pm.get_msgs_failed() > 450 && pm.get_msgs_failed() < 550);
    assert!(pm.get_msgs_sent() > 450 && pm.get_msgs_sent() < 550);
    // check total messages sent
    assert_eq!(pm.get_msgs_sent() + pm.get_msgs_failed(), 1000);
    assert_eq!(pm.get_avg_wait_time(), 0);
}

// test that send messages correctly adds to pm
#[tokio::test]
async fn test_send_msg_adds_to_pm() {
    let pm = Arc::new(ProgressMonitor::new());
    let senders = create_sim_senders(10.0, 100.0, 1.0, 1.0);
    let sender_service = SenderService::new(senders.unwrap(), pm.clone()).await;
    sender_service.send("abacabdaba".to_owned()).await;
    sender_service.send("abacabdaba".to_owned()).await;
    sender_service.send("abacabdaba".to_owned()).await;
    sender_service.send("abacabdaba".to_owned()).await;
    assert_eq!(pm.get_msgs_sent(), 0);
    assert_eq!(pm.get_msgs_failed(), 4);
    assert_eq!(pm.get_avg_wait_time(), 1);
}

// test that new sender pool produces different results (but a send will still continue)
#[tokio::test]
async fn test_new_sender_pool_sync() {
    let pm = Arc::new(ProgressMonitor::new());
    let senders = create_sim_senders(2.0, 0.0, 5.0, 1.0);
    let sender_service = SenderService::new(senders.unwrap(), pm.clone()).await;
    for _ in 0..5 {
        sender_service.send("abacabdaba".to_owned()).await;
    }
    let new_senders = create_sim_senders(2.0, 100.0, 1.0, 1.0).unwrap();
    let _ = sender_service.new_sender_pool(new_senders).await;
    let send_success = sender_service.send("abacabdaba".to_owned()).await;
    assert!(!send_success, "Failed send should be false");
    assert_eq!(pm.get_msgs_sent(), 5);
    assert_eq!(pm.get_msgs_failed(), 1);
    assert!(pm.get_avg_wait_time() < 5);
}

// test that new sender pool will effect waiting send commands... it is completly normal for there 
// to be a print statement regarding Failed to return sender to pool... there is an new pool
// so that sender should not be returned to the old pool.
#[tokio::test]
async fn test_new_sender_pool_multi_task() {
    let pm = Arc::new(ProgressMonitor::new());
    let senders = create_sim_senders(2.0, 0.0, 1.0, 0.0);
    let sender_service = Arc::new(SenderService::new(senders.unwrap(), pm.clone()).await);
    let mut handles = Vec::new();

    let sender_service_clone = sender_service.clone();
    handles.push(tokio::spawn(async move {
        for _ in 0..5 {
            sender_service_clone.send("abacabdaba".to_owned()).await;
        }
    }));
    
    let sender_service_clone_2 = sender_service.clone();
    handles.push(tokio::spawn(async move {
        for _ in 0..5 {
            sender_service_clone_2.send("abacabdaba".to_owned()).await;
        }
    }));

    let sender_service_clone_3 = sender_service.clone();
    handles.push(tokio::spawn(async move {
        for _ in 0..5 {
            sender_service_clone_3.send("abacabdaba".to_owned()).await;
        }
    }));

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    assert_eq!(pm.get_msgs_failed(), 0);
    assert_eq!(pm.get_avg_wait_time(), 1);
    let _ = sender_service.new_sender_pool(create_sim_senders(2.0, 100.0, 3.0, 0.0).unwrap()).await;

    futures::future::join_all(handles).await;
 
    assert!(pm.get_msgs_failed() > 0);  
    assert!(pm.get_avg_wait_time() > 1);
}


// test shutdown works and allows for requests to finish
#[tokio::test]
async fn test_shutdown_works() {
    let pm = Arc::new(ProgressMonitor::new());
    let senders = create_sim_senders(2.0, 0.0, 1.0, 0.0);
    let sender_service = Arc::new(SenderService::new(senders.unwrap(), pm.clone()).await);
        for _ in 0..5 {
            let sender_service_clone_2 = sender_service.clone();
            tokio::spawn(async move {
                sender_service_clone_2.send("abacabdaba".to_owned()).await;
            });
        }


    // wait a tiny second so that the tasks can all send there send requests
    tokio::time::sleep(tokio::time::Duration::from_nanos(1)).await;
    sender_service.shut_down().await;
    assert_eq!(pm.get_msgs_sent(), 5);  
    assert_eq!(pm.get_avg_wait_time(), 1);

    

    // THE SAME TEST ABOVE WILL FAIL IF THE SHUTDOWN FUNCTION IS MISSING
    let pm = Arc::new(ProgressMonitor::new());
    let senders = create_sim_senders(2.0, 0.0, 1.0, 0.0);
    let sender_service = Arc::new(SenderService::new(senders.unwrap(), pm.clone()).await);
        for _ in 0..5 {
            let sender_service_clone_2 = sender_service.clone();
            tokio::spawn(async move {
                sender_service_clone_2.send("abacabdaba".to_owned()).await;
            });
        }
    // wait a tiny second so that the tasks can all send there send requests
    tokio::time::sleep(tokio::time::Duration::from_nanos(1)).await;
    assert_eq!(pm.get_msgs_sent(), 0);  
    assert_eq!(pm.get_avg_wait_time(), 0);

}