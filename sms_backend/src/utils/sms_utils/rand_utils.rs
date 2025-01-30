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

/// Create a random number that over a large amount of calls will center around the given mean.
/// This is calculated by sampling from an exponential decay. 
/// IMPORTANT: by the nature of sampling from an exponential decay the large the mean the greater the chance 
///           you can recieve very large values or very small values... but nonetheless the sum of a large sample
///           will be close to your mean.
/// 
/// If you looking to use something that less of a chance to create super large number, consider using the `Normal`
/// distrubtion offered by the `rand_distr` crate. However, we don't use it here because we want to create a positive
/// number which cannot really by guranteed in a `Normal` distrubution without extra checks.
/// (credit for this approach goes to Ric Newbery from my Analog interview... thank you Ric!)
/// 
/// Parameters:
/// 
/// * `mean` - the average number to center the random number around
/// 
/// Returns a random number centered around `mean` with a standard deviation of `std_dev`
pub fn create_random_pos_number(mean: f32) -> f32 {
    let normal = rand_distr::Exp::new( 1 as f32 / mean).unwrap();
    let random_number = normal.sample(&mut rand::thread_rng());
    return random_number;
}