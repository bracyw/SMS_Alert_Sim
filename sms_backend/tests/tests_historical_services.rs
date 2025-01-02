mod test_utils;
use serial_test::serial;
use test_utils::{connect_to_database, reset_and_seed_database};
use sms_backend::services::histroical_services::{get_all_alert_data, get_user_history};

#[tokio::test]
#[serial]
async fn test_get_user_history() {
    // Connect to the database and seed data
    let connection = connect_to_database().await;
    reset_and_seed_database(&connection).await;

    // Test the function with an existing user ID
    let user_id = 1; // Assuming the first user ID is 1
    let history = get_user_history(user_id, connection.clone())
        .await
        .expect("Failed to fetch user history");

    // Assert the correct number of alerts for the user
    assert_eq!(history.len(), 1, "Expected 1 alert for user ID {}", user_id);

    // Verify the details of the alert
    let alert = &history[0];
    assert_eq!(alert.user_id, user_id as i32);
    assert_eq!(alert.send_amount_requested, 100);
    assert_eq!(alert.message.as_deref(), Some("First Alert"));
}

#[tokio::test]
#[serial]
async fn test_get_all_alert_data() {
    // Connect to the database and seed data
    let connection = connect_to_database().await;
    reset_and_seed_database(&connection).await;

    // Test the function
    let alerts = get_all_alert_data(connection.clone())
        .await
        .expect("Failed to fetch all alert data");

    // Assert the total number of alerts
    assert_eq!(alerts.len(), 2, "Expected 2 alerts in total");

    // Verify details of the first alert
    let first_alert = alerts.iter().find(|a| a.send_amount_requested == 100).unwrap();
    assert_eq!(first_alert.user_id, 1);
    assert_eq!(first_alert.message.as_deref(), Some("First Alert"));

    // Verify details of the second alert
    let second_alert = alerts.iter().find(|a| a.send_amount_requested == 200).unwrap();
    assert_eq!(second_alert.user_id, 2);
    assert_eq!(second_alert.message.as_deref(), Some("Second Alert"));
}
