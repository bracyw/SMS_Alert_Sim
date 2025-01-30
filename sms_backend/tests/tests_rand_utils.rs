use sms_backend::utils::sms_utils::rand_utils::{create_random_pos_number, create_random_string};

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
    let mean = 1.0;
    let mut random_number;
    let mut avg = 0.0;
    let total_num_gen  = 10000;
    for _n in 0..total_num_gen {
        random_number = create_random_pos_number(mean);
        println!("random number: {:?}", random_number);
        avg += random_number;
    }

    avg = avg / total_num_gen as f32;
    assert!(avg <= (mean + 0.5) && avg >= (mean - 0.5), "test that the avg of all random numbers generated is within an expectable range for a large sample");
}
