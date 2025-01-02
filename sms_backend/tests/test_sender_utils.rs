use std::sync::Arc;

use sms_backend::{structures::progress_monitor::ProgressMonitor, utils::sms_utils::{error_utils::SenderValidationError, sender_utils::{create_sender_service, create_sim_senders}}};

#[tokio::test]
async fn test_create_sim_senders_success() {
    let num_senders = 5;
    let failure_percent = 10;
    let avg_send_time = 2.0;
    let std_dev = 1.0;

    let senders = create_sim_senders(num_senders, failure_percent, avg_send_time, std_dev);
    assert!(senders.is_ok());
    assert_eq!(senders.unwrap().len(), num_senders as usize);
}

#[tokio::test]
async fn test_create_sim_senders_invalid_input() {
    let result = create_sim_senders(5, 10, -2.0, 1.0);
    assert!(matches!(result, Err(SenderValidationError::InvalidSendTimeMean(-2.0))));
}

#[tokio::test]
async fn test_create_sender_service_success() {
    let pm = Arc::new(ProgressMonitor::new());
    let num_senders = 3;
    let failure_percent = 20;
    let avg_send_time = 1.5;
    let std_dev = 0.5;

    let service = create_sender_service(num_senders, failure_percent, avg_send_time, std_dev, pm.clone()).await;
    assert!(service.is_ok());
}

#[tokio::test]
async fn test_create_sender_service_invalid_input() {
    let pm = Arc::new(ProgressMonitor::new());
    let result = create_sender_service(3, 20, -1.0, 0.5, pm.clone()).await;
    assert!(matches!(result, Err(SenderValidationError::InvalidSendTimeMean(-1.0))));
}
