use thiserror::Error;

#[derive(Error, Debug)]
/// A custom error type for the sender service, used to validate the sender service parameters.
pub enum SenderValidationError {
    #[error("Send time mean was less than zero, got {0}")]
    InvalidSendTimeMean(f32),

    #[error("Failure percentage cannot be negative, got {0}")]
    NegativeFailurePercentage(f64),

    #[error("Failure percentage cannot exceed 1 (percentage should be given in terms of 0..0.5..1)")]
    ExcessiveFailurePercentage(f64),
}


impl SenderValidationError {
    /// Validate the given parameters and return an error if any condition is violated.
    pub fn validate(failure_percentage: f64, send_time_mean: f32) -> Result<(), Self> {
        if send_time_mean < 0.0 {
            return Err(Self::InvalidSendTimeMean(send_time_mean));
        }
        if failure_percentage < 0.0 {
            return Err(Self::NegativeFailurePercentage(failure_percentage));
        }
        if failure_percentage > 1.0 {
            return Err(Self::ExcessiveFailurePercentage(failure_percentage));
        }
        Ok(())
    }
}
