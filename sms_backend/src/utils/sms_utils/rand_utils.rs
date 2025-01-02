use rand::Rng;
use rand_distr::{Alphanumeric, Distribution};

/// Create a random string of length up to `max_length`
pub fn create_random_string(max_length: usize) -> String {
    rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(rand::thread_rng().gen_range(1..=max_length))
                    .map(char::from)
                    .collect()
}

/// Create a random number centered around `mean` with a standard deviation of `std_dev`
/// 
/// Parameters:
/// 
/// * `mean` - the average number to center the random number around
/// * `std_dev` - the standard deviation of the random number
/// 
/// Returns a random number centered around `mean` with a standard deviation of `std_dev`
pub fn create_random_number(mean: f32, std_dev: f32) -> f32 {
    let normal = rand_distr::Normal::new(mean, std_dev).unwrap();
    normal.sample(&mut rand::thread_rng()).abs()
}