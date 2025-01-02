use std::sync::Arc;

use futures::{stream::futures_unordered, StreamExt};
use sms_backend::structures::progress_monitor::ProgressMonitor;
use tokio::task;


// ASYNC FUNCTIONALITY TESTS
#[tokio::test]
async fn test_reset_works_async() {
    // test that reset is respected with high thread competition
    let pm = Arc::new(ProgressMonitor::new());
    let futures_unordered = futures_unordered::FuturesUnordered::new();
    for _ in 0..100 {
        let pm_clone_1 = pm.clone();

        futures_unordered.push(task::spawn(async move {
            for _ in 0..1000000 {
                // will add 1000 sent and 1000 failed messages
                pm_clone_1.add_message_sent(10);
                pm_clone_1.add_message_failed(10);
                pm_clone_1.get_avg_wait_time();
                pm_clone_1.get_msgs_failed();
                pm_clone_1.get_msgs_sent();
                pm_clone_1.reset();
            }
        }));
        
        let pm_clone_2 = pm.clone();
        futures_unordered.push (task::spawn(
        async move {
            for _ in 0..1000000 {
                pm_clone_2.get_avg_wait_time();
                // will add 1000 sent and 1000 failed messages
                pm_clone_2.add_message_sent(10);
                pm_clone_2.add_message_failed(10);
                pm_clone_2.get_msgs_failed();
                pm_clone_2.get_msgs_sent();
                pm_clone_2.reset();
            }
        }));

        let pm_clone_3 = pm.clone();
        futures_unordered.push (task::spawn(async move {
            for _ in 0..1000000 {
                pm_clone_3.get_msgs_failed();
                pm_clone_3.get_msgs_sent();
                // will add 1000 sent and 1000 failed messages
                pm_clone_3.add_message_sent(10);
                pm_clone_3.add_message_failed(10);
                pm_clone_3.get_avg_wait_time();
                pm_clone_3.reset();

            }
        }));
    }

    futures_unordered.for_each(|_| async {}).await;
    assert_eq!(pm.get_msgs_sent(), 0);
    assert_eq!(pm.get_msgs_failed(), 0);
    assert_eq!(pm.get_avg_wait_time(), 0);

}

#[tokio::test]
async fn test_get_avg_time_works_async() {
    let pm = Arc::new(ProgressMonitor::new());
    let futures_unordered = futures_unordered::FuturesUnordered::new();
    for _ in 0..10 {
        let pm_clone_1 = pm.clone();
        futures_unordered.push(task::spawn(async move {
            for _ in 0..1000000 {
                // will add 1000 sent
                pm_clone_1.add_message_sent(9);
                pm_clone_1.add_message_sent(11);
            }
        }));

        let pm_clone_2 = pm.clone();
        futures_unordered.push(task::spawn(async move {
            for _ in 0..1000000 {
                pm_clone_2.add_message_sent(11);
                pm_clone_2.add_message_sent(9);
            }
        }));

        let pm_clone_3 = pm.clone();
        futures_unordered.push(task::spawn(async move {
            for _ in 0..1000000 {
                assert_eq!(pm_clone_3.get_avg_wait_time(), 10, "Average wait time is not 10, it is {}", pm_clone_3.get_avg_wait_time());
            }
        }));
    }

    futures_unordered.for_each(|_| async {}).await;
    assert_eq!(pm.get_msgs_sent(), 40000000);
    assert_eq!(pm.get_msgs_failed(), 0);
    assert_eq!(pm.get_avg_wait_time(), 10);
}

#[tokio::test]
async fn stress_test_add_and_get() {
    // test that the basic functionality works in a sequential manner
    let pm = Arc::new(ProgressMonitor::new());
    let futures_unordered = futures_unordered::FuturesUnordered::new();
    for _ in 0..100 {
        let pm_clone_1 = pm.clone();
        
        futures_unordered.push(task::spawn(async move {
            for _ in 0..10000 {
                // will add 1000 sent and 1000 failed messages
                pm_clone_1.add_message_sent(10);
                pm_clone_1.add_message_failed(10);
                pm_clone_1.get_avg_wait_time();
                pm_clone_1.get_msgs_failed();
                pm_clone_1.get_msgs_sent();
            }
        }));
     
        let pm_clone_2 = pm.clone();
        futures_unordered.push(task::spawn(async move {
            for _ in 0..10000 {
                pm_clone_2.get_avg_wait_time();
                // will add 1000 sent and 1000 failed messages
                pm_clone_2.add_message_sent(10);
                pm_clone_2.add_message_failed(10);
                pm_clone_2.get_msgs_failed();
                pm_clone_2.get_msgs_sent();
            }
        }));
    
        let pm_clone_3 = pm.clone();
        futures_unordered.push(task::spawn(async move {
            for _ in 0..10000 {
                pm_clone_3.get_msgs_failed();
                pm_clone_3.get_msgs_sent();
                // will add 1000 sent and 1000 failed messages
                pm_clone_3.add_message_sent(10);
                pm_clone_3.add_message_failed(10);
                pm_clone_3.get_avg_wait_time();
            }
        }));
    }

    futures_unordered.for_each(|_| async {}).await;
    assert_eq!(pm.get_msgs_sent(), 3000000);
    assert_eq!(pm.get_msgs_failed(), 3000000);
    assert_eq!(pm.get_avg_wait_time(), 10);
}

#[tokio::test]
async fn stress_test_add_async() {
    // test that the basic functionality works in a sequential manner
    let pm = Arc::new(ProgressMonitor::new());
    let futures_unordered = futures_unordered::FuturesUnordered::new();
    for _ in 0..10 {
        let pm_clone_1 = pm.clone();
        
        futures_unordered.push(task::spawn(async move {
            for _ in 0..100 {
                pm_clone_1.add_message_sent(1);
            }
        }));
     
        let pm_clone_2 = pm.clone();
        futures_unordered.push (task::spawn(
            async move {
            for _ in 0..100 {
                pm_clone_2.add_message_failed(1);
            }
        }));
    
        let pm_clone_3 = pm.clone();
        futures_unordered.push (task::spawn(async move {
            for _ in 0..100 {
                pm_clone_3.add_message_sent(1);
                pm_clone_3.add_message_failed(1);
            }
        }));
    }

    futures_unordered.for_each(|_| async {}).await;
    assert_eq!(pm.get_msgs_sent(), 2000);
    assert_eq!(pm.get_msgs_failed(), 2000);
    assert_eq!(pm.get_avg_wait_time(), 1);
}


// BASIC FUNCTIONALITY TESTS (not really async)
#[tokio::test]
async fn test_zeroed_at_init() {
    // test that the basic functionality works in a sequential manner
    let pm = ProgressMonitor::new();
    assert_eq!(pm.get_msgs_sent(), 0);
    assert_eq!(pm.get_msgs_failed(), 0);
    assert_eq!(pm.get_avg_wait_time(), 0);
}

#[tokio::test]
async fn test_add_message_sent() {
    // test that the basic functionality works in a sequential manner
    let pm = ProgressMonitor::new();
    pm.add_message_sent(1);
    assert_eq!(pm.get_msgs_sent(), 1);
    assert_eq!(pm.get_msgs_failed(), 0);
    assert_eq!(pm.get_avg_wait_time(), 1);
}

#[tokio::test]
async fn test_add_message_failed() {
    // test that the basic functionality works in a sequential manner
    let pm = ProgressMonitor::new();
    pm.add_message_failed(1);
    assert_eq!(pm.get_msgs_sent(), 0);
    assert_eq!(pm.get_msgs_failed(), 1);
    assert_eq!(pm.get_avg_wait_time(), 1);
}


#[tokio::test]
async fn test_add_message_sent_and_failed() {
    // test that the basic functionality works in a sequential manner
    let pm = ProgressMonitor::new();
    pm.add_message_sent(1);
    pm.add_message_failed(1);
    assert_eq!(pm.get_msgs_sent(), 1);
    assert_eq!(pm.get_msgs_failed(), 1);
    assert_eq!(pm.get_avg_wait_time(), 1);
}


