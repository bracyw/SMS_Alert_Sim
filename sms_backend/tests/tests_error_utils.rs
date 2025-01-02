

    use sms_backend::utils::sms_utils::error_utils::SenderValidationError;
    
        #[tokio::test]
    async fn test_validate_success() {
        let result = SenderValidationError::validate(0.5, 5.0, 2.0);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_invalid_send_time_mean() {
        let result = SenderValidationError::validate(0.5, -1.0, 2.0);
        assert!(matches!(result, Err(SenderValidationError::InvalidSendTimeMean(-1.0))));
    }

    #[tokio::test]
    async fn test_validate_invalid_send_std() {
        let result = SenderValidationError::validate(0.5, 5.0, -2.0);
        assert!(matches!(result, Err(SenderValidationError::InvalidSendStd(-2.0))));
    }

    #[tokio::test]
    async fn test_validate_negative_failure_percentage() {
        let result = SenderValidationError::validate(-0.5, 5.0, 2.0);
        assert!(matches!(result, Err(SenderValidationError::NegativeFailurePercentage(-0.5))));
    }

    #[tokio::test]
    async fn test_validate_excessive_failure_percentage() {
        let result = SenderValidationError::validate(1.5, 5.0, 2.0);
        assert!(matches!(result, Err(SenderValidationError::ExcessiveFailurePercentage(1.5))));
    }
