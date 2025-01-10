use sms_backend::utils::sms_utils::rand_utils::{create_random_number, create_random_string};

#[tokio::test]
async fn test_create_random_string() {
    let max_length = 10;
    let random_string = create_random_string(max_length);

    // Assert that the string length is within the expected range
    assert!(random_string.len() <= max_length);
    assert!(random_string.len() >= 1);

    // Assert that the string contains only alphanumeric characters
    assert!(random_string.chars().all(char::is_alphanumeric));
}

#[tokio::test]
async fn test_create_random_number() {
    let mean = 5.0;
    let std_dev = 2.0;
    let random_number = create_random_number(mean, std_dev);

    // Assert that the number is positive (because abs is used)
    assert!(random_number >= 0.0);
}
